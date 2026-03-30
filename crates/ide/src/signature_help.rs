use base_db::{EditionedFileId, FilePosition, TextSize};
use hir::{HirDatabase as _, Semantics, database::DefDatabase};
use hir_def::{
    database::{InternDatabase as _, Location},
    item_tree::ModuleItem,
    resolver::ScopeDef,
};
use hir_ty::{
    function::FunctionDetails,
    infer::ResolvedCall,
    ty::pretty::{TypeVerbosity, pretty_fn_inner_with_offsets, pretty_fn_with_verbosity},
};
use ide_db::RootDatabase;
use rowan::{TextLen, TextRange};
use syntax::{AstNode as _, SyntaxKind, SyntaxToken, ast};

/// Signature help information for a function call.
/// This must store a vec because WGSL has function overloading.
/// Same as `lsp_types::SignatureHelp`.
#[derive(Debug)]
pub struct SignatureHelp {
    pub signatures: Vec<OverloadSignatureHelp>,
    pub active_signature: Option<u32>,
    pub active_parameter: Option<u32>,
}

/// Contains information about an item signature as seen from a use site.
///
/// This includes the "active parameter", which is the parameter whose value is currently being
/// edited.
#[expect(
    clippy::partial_pub_fields,
    reason = "not worth refactoring at the moment"
)]
#[derive(Debug)]
pub struct OverloadSignatureHelp {
    pub documentation: Option<String>,
    pub signature: String,
    pub active_parameter: Option<usize>,
    parameters: Vec<TextRange>,
}

impl OverloadSignatureHelp {
    pub fn parameter_labels(&self) -> impl Iterator<Item = &str> + '_ {
        self.parameters
            .iter()
            .map(move |&text_range| &self.signature[text_range])
    }

    #[must_use]
    pub fn parameter_ranges(&self) -> &[TextRange] {
        &self.parameters
    }

    fn push_call_parameter(
        &mut self,
        parameter: &str,
    ) {
        self.push_parameter("(", parameter);
    }

    fn push_generic_param(
        &mut self,
        parameter: &str,
    ) {
        self.push_parameter("<", parameter);
    }

    fn push_record_field(
        &mut self,
        parameter: &str,
    ) {
        self.push_parameter("{ ", parameter);
    }

    fn push_parameter(
        &mut self,
        opening_delimiter: &str,
        parameter: &str,
    ) {
        if !self.signature.ends_with(opening_delimiter) {
            self.signature.push_str(", ");
        }
        let start = TextSize::of(&self.signature);
        self.signature.push_str(parameter);
        let end = TextSize::of(&self.signature);
        self.parameters.push(TextRange::new(start, end));
    }
}

pub(crate) fn signature_help(
    database: &RootDatabase,
    position: FilePosition,
) -> Option<SignatureHelp> {
    let semantics = Semantics::new(database);
    let file_id = EditionedFileId::from_file(database, position.file_id);
    let source_file = semantics.parse(file_id);
    let syntax = source_file.syntax();

    // Find the token at the cursor position
    let token = syntax.token_at_offset(position.offset).left_biased()?;

    // Walk up to find the enclosing Arguments node
    let (function_call, _, active_parameter) = find_enclosing_call(&token, position.offset)?;

    // Try to resolve the function call via type inference
    let text = function_call.ident_expression()?.syntax().text();
    let mut overloads = Vec::new();
    semantics
        .resolver(file_id, syntax)
        .process_all_names(|name, scope_def| {
            if name.as_str().to_owned().contains(&text.to_string())
                && let ScopeDef::ModuleItem(file_id, ModuleItem::Function(module_item_id)) =
                    scope_def
            {
                overloads.push(module_item_id);
            }
        });

    let mut signatures: Vec<_> = overloads
        .into_iter()
        .map(|id| database.intern_function(Location::new(file_id, id)))
        .map(|function_id| database.function_type(function_id))
        .map(|function_type| function_type.lookup(database))
        .filter_map(|function| {
            let length: u32 = function.parameters.len().try_into().unwrap();
            (active_parameter.is_none() || active_parameter.is_some_and(|index| index < length))
                .then(|| build_signature(database, &function, None))
        })
        .collect();
    let mut active_signature = None;

    if signatures.is_empty() {
        return None;
    }

    Some(SignatureHelp {
        signatures,
        active_signature,
        active_parameter,
    })
}

fn find_enclosing_call(
    token: &SyntaxToken,
    offset: TextSize,
) -> Option<(ast::FunctionCall, ast::Arguments, Option<u32>)> {
    // Walk up ancestors to find an Arguments node
    for ancestor in token.parent_ancestors() {
        if let Some(arguments) = ast::Arguments::cast(ancestor.clone()) {
            // The parent of Arguments should be a FunctionCall
            // add support for builtins
            let function_call = ast::FunctionCall::cast(ancestor.parent()?)?;
            if arguments.arguments().next().is_none() {
                return Some((function_call, arguments, None));
            }

            // Count commas before the cursor to determine active parameter
            let mut parameter_index: u32 = 0;
            for child in arguments.syntax().children_with_tokens() {
                if child.text_range().start() >= offset {
                    break;
                }
                if child.kind() == SyntaxKind::Comma {
                    parameter_index += 1;
                }
            }

            return Some((function_call, arguments, Some(parameter_index)));
        }
    }
    None
}

fn build_signature(
    database: &RootDatabase,
    function: &FunctionDetails,
    documentation: Option<&str>,
) -> OverloadSignatureHelp {
    let mut signature = String::new();
    let mut parameters = Vec::new();
    pretty_fn_inner_with_offsets(
        database,
        function,
        &mut signature,
        TypeVerbosity::default(),
        Some(&mut parameters),
    )
    .unwrap();

    OverloadSignatureHelp {
        documentation: documentation.map(String::from),
        signature,
        active_parameter: None,
        parameters,
    }
}

#[cfg(test)]
mod tests {
    use test_utils::extract_offset;

    use crate::Analysis;

    use super::*;

    #[test]
    fn find_enclosing_call_works_0() {
        let text = r#"
fn foo() {
    bar($0);
}
fn bar(x: u32, y: bool) -> f32 { 0.0f }
"#;

        let (offset, text) = extract_offset(text);
        let (analysis, file_id) = Analysis::from_single_file(text);
        let semantics = Semantics::new(&analysis.database);
        let file_id = EditionedFileId::from_file(&analysis.database, file_id);
        let source_file = semantics.parse(file_id);
        let syntax = source_file.syntax();
        let token = syntax.token_at_offset(offset).left_biased().unwrap();
        let result = find_enclosing_call(&token, offset).unwrap();
        assert_eq!(result.2, None);
    }

    #[test]
    fn find_enclosing_call_works_1() {
        let text = r#"
fn foo() {
    bar(1$0);
}
fn bar(x: u32, y: bool) -> f32 { 0.0f }
"#;

        let (offset, text) = extract_offset(text);
        let (analysis, file_id) = Analysis::from_single_file(text);
        let semantics = Semantics::new(&analysis.database);
        let file_id = EditionedFileId::from_file(&analysis.database, file_id);
        let source_file = semantics.parse(file_id);
        let syntax = source_file.syntax();
        let token = syntax.token_at_offset(offset).left_biased().unwrap();
        let result = find_enclosing_call(&token, offset).unwrap();
        assert_eq!(result.2, Some(0));
    }

    #[test]
    fn find_enclosing_call_works2() {
        let text = r#"
fn foo() {
    bar(1, $0);
}
fn bar(x: u32, y: bool) -> f32 { 0.0f }
"#;

        let (offset, text) = extract_offset(text);
        let (analysis, file_id) = Analysis::from_single_file(text);
        let semantics = Semantics::new(&analysis.database);
        let file_id = EditionedFileId::from_file(&analysis.database, file_id);
        let source_file = semantics.parse(file_id);
        let syntax = source_file.syntax();
        let token = syntax.token_at_offset(offset).left_biased().unwrap();
        let result = find_enclosing_call(&token, offset).unwrap();
        assert_eq!(result.2, Some(1));
    }
}
