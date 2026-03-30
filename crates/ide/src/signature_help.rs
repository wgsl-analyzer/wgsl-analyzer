use base_db::{EditionedFileId, FilePosition, TextSize};
use hir::Semantics;
use hir_ty::{
    function::FunctionDetails,
    infer::ResolvedCall,
    ty::pretty::{TypeVerbosity, pretty_fn_inner_with_offsets},
};
use ide_db::RootDatabase;
use rowan::TextRange;
use syntax::{AstNode as _, SyntaxKind, SyntaxToken, ast};

/// Signature help information for a function call.
#[derive(Debug, Clone)]
pub struct SignatureHelp {
    // TODO: add documentation
    // https://github.com/wgsl-analyzer/wgsl-analyzer/issues/971
    // pub doc: Option<Documentation<'static>>,
    pub signatures: Vec<SignatureInformation>,
    pub active_signature: Option<u32>,
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
pub struct ParameterInformation {
    pub range: TextRange,
    pub label: String,
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
    let container = semantics.find_container(file_id, function_call.syntax())?;
    let def_with_body = container.as_def_with_body_id()?;
    let analyzed = semantics.analyze(def_with_body);

    let call_expr = ast::Expression::FunctionCall(function_call);
    let expression_id = analyzed.expression_id(&call_expr);

    let mut signatures = Vec::new();
    let mut active_signature = None;

    if let Some(expr_id) = expression_id
        && let Some(resolved) = analyzed.infer.call_resolution(expr_id)
    {
        match resolved {
            ResolvedCall::Function(func_id) => {
                let function = func_id.lookup(database);
                signatures.push(build_signature(database, &function, None));
                active_signature = Some(0);
            },
            // TODO: implement signature help for other kinds of calls
            // https://github.com/wgsl-analyzer/wgsl-analyzer/issues/970
            ResolvedCall::OtherTypeInitializer(_) => return None,
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
        .map(|(range, label)| ParameterInformation { range, label })
        .collect();

    SignatureInformation {
        label,
        documentation: documentation.map(String::from),
        parameters,
    }
}
