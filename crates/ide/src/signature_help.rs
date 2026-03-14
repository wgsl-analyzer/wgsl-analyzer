use base_db::{FilePosition, TextSize};
use hir::{Definition, Function, HirDatabase as _, ModuleDef, Semantics};
use hir_def::{
    database::{DefDatabase as _, Lookup as _},
    item_tree::Name,
};
use hir_ty::{
    builtins::Builtin,
    function::FunctionDetails,
    infer::ResolvedCall,
    ty::pretty::{TypeVerbosity, pretty_fn_inner_with_offsets},
};
use ide_db::RootDatabase;
use syntax::{AstNode as _, SyntaxKind, SyntaxToken, ast};

/// Signature help information for a function call.
#[derive(Debug, Clone)]
pub struct SignatureHelp {
    pub signatures: Vec<SignatureInformation>,
    pub active_signature: Option<usize>,
    pub active_parameter: Option<u32>,
}

/// Information about a single function signature.
#[derive(Debug, Clone)]
pub struct SignatureInformation {
    pub label: String,
    pub documentation: Option<String>,
    pub parameters: Vec<ParameterInformation>,
}

/// Information about a single parameter.
#[derive(Debug, Clone)]
#[expect(
    clippy::struct_field_names,
    reason = "label_start/label_end clearly describe byte offset range"
)]
pub struct ParameterInformation {
    /// Byte offset range within the signature label string.
    pub label_start: u32,
    pub label_end: u32,
}

pub(crate) fn signature_help(
    database: &RootDatabase,
    position: FilePosition,
) -> Option<SignatureHelp> {
    let semantics = Semantics::new(database);
    let file_id = database.editioned_file_id(position.file_id);
    let source_file = semantics.parse(file_id);
    let syntax = source_file.syntax();

    // Find the token at the cursor position
    let token = syntax.token_at_offset(position.offset).left_biased()?;

    // Walk up to find the enclosing Arguments node
    let (function_call, arguments_node, active_parameter) =
        find_enclosing_call(&token, position.offset)?;

    // Try to resolve the function call via type inference
    let container = semantics.find_container(file_id.into(), function_call.syntax())?;
    let def_with_body = container.as_def_with_body_id()?;
    let analyzed = semantics.analyze(def_with_body);

    let call_expr = ast::Expression::FunctionCall(function_call.clone());
    let expression_id = analyzed.expression_id(&call_expr);

    let mut signatures = Vec::new();
    let mut active_signature = None;

    // Try to extract doc comments for the function being called
    let fn_doc = function_call.ident_expression().and_then(|ident_expr| {
        let name_token = ident_expr.syntax().first_token()?;
        let definition = Definition::from_token(&semantics, file_id.into(), &name_token)?;
        definition.doc_comments(database)
    });

    if let Some(expr_id) = expression_id
        && let Some(resolved) = analyzed.infer.call_resolution(expr_id)
    {
        match resolved {
            ResolvedCall::Function(func_id) => {
                let function = func_id.lookup(database);
                signatures.push(build_signature(database, &function, fn_doc.as_deref()));
                active_signature = Some(0);
            },
            ResolvedCall::OtherTypeInitializer(_) => return None,
        }
    }

    // If we couldn't resolve via inference, try name-based lookup for builtins
    if signatures.is_empty()
        && let Some(ident_expr) = function_call.ident_expression()
    {
        let name_text = ident_expr.syntax().text().to_string();
        // Remove template parameters if present
        let name_text = name_text.split('<').next().unwrap_or(&name_text).trim();
        let name = Name::from(name_text);
        if let Some(builtin) = Builtin::for_name(database, &name) {
            // Collect types of already-typed arguments to filter overloads
            let arg_types: Vec<_> = arguments_node
                .arguments()
                .filter_map(|arg| analyzed.type_of_expression(&arg))
                .collect();

            for (overload_index, (_, overload)) in builtin
                .matching_overloads(database, &arg_types)
                .iter()
                .enumerate()
            {
                let function = overload.r#type.lookup(database);
                signatures.push(build_signature(database, &function, None));
                if overload_index == 0 {
                    active_signature = Some(0);
                }
            }
        }
    }

    if signatures.is_empty() {
        return None;
    }

    Some(SignatureHelp {
        signatures,
        active_signature,
        active_parameter: Some(active_parameter),
    })
}

fn find_enclosing_call(
    token: &SyntaxToken,
    offset: TextSize,
) -> Option<(ast::FunctionCall, ast::Arguments, u32)> {
    // Walk up ancestors to find an Arguments node
    for ancestor in token.parent_ancestors() {
        if let Some(arguments) = ast::Arguments::cast(ancestor.clone()) {
            // The parent of Arguments should be a FunctionCall
            let function_call = ast::FunctionCall::cast(ancestor.parent()?)?;

            // Count commas before the cursor to determine active parameter
            let mut param_index: u32 = 0;
            for child in arguments.syntax().children_with_tokens() {
                if child.text_range().start() >= offset {
                    break;
                }
                if child.kind() == SyntaxKind::Comma {
                    param_index += 1;
                }
            }

            return Some((function_call, arguments, param_index));
        }
    }
    None
}

fn build_signature(
    database: &RootDatabase,
    function: &FunctionDetails,
    documentation: Option<&str>,
) -> SignatureInformation {
    let mut label = String::new();
    let mut offsets = Vec::new();
    pretty_fn_inner_with_offsets(
        database,
        function,
        &mut label,
        TypeVerbosity::default(),
        Some(&mut offsets),
    )
    .unwrap();

    let parameters = offsets
        .into_iter()
        .map(|(start, end)| ParameterInformation {
            label_start: start,
            label_end: end,
        })
        .collect();

    SignatureInformation {
        label,
        documentation: documentation.map(String::from),
        parameters,
    }
}
