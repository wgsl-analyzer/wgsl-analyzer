use hir_def::{
    expression::ExpressionId,
    expression_store::{ExpressionStoreSource, path::Path},
    item_tree::Name,
};

use crate::{
    builtins::BuiltinId,
    infer::TypeExpectation,
    lower::{LoweredKind, TypeLoweringError},
    ty::Type,
};

#[derive(PartialEq, Eq, Debug)]
pub struct InferenceDiagnostic {
    pub source: ExpressionStoreSource,
    pub kind: InferenceDiagnosticKind,
}

#[derive(PartialEq, Eq, Debug)]
pub enum InferenceDiagnosticKind {
    AssignmentNotAReference {
        left_side: ExpressionId,
        actual: Type,
    },
    TypeMismatch {
        expression: ExpressionId,
        expected: TypeExpectation,
        actual: Type,
    },
    NoSuchField {
        expression: ExpressionId,
        name: Name,
        r#type: Type,
    },
    ArrayAccessInvalidType {
        expression: ExpressionId,
        r#type: Type,
    },
    UnresolvedName {
        expression: ExpressionId,
        name: Name,
    },
    InvalidConstructionType {
        expression: ExpressionId,
        r#type: Type,
    },
    FunctionCallArgCountMismatch {
        expression: ExpressionId,
        n_expected: usize,
        n_actual: usize,
    },
    NoBuiltinOverload {
        expression: ExpressionId,
        builtin: BuiltinId,
        name: Option<&'static str>,
        parameters: Vec<Type>,
    },
    NoConstructor {
        expression: ExpressionId,
        builtins: BuiltinId,
        r#type: Type,
        parameters: Vec<Type>,
    },
    AddressOfNotReference {
        expression: ExpressionId,
        actual: Type,
    },
    DerefNotAPointer {
        expression: ExpressionId,
        actual: Type,
    },
    InvalidType {
        error: TypeLoweringError,
    },
    CyclicType {
        name: Name,
        range: base_db::TextRange,
    },
    UnexpectedTemplateArgument {
        expression: ExpressionId,
    },
    WgslError {
        expression: ExpressionId,
        message: String,
    },
    ExpectedLoweredKind {
        expression: ExpressionId,
        expected: LoweredKind,
        actual: LoweredKind,
        path: Path,
    },
}
