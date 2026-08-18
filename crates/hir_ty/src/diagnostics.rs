use hir_def::{
    expression::ExpressionId,
    expression_store::{ExpressionStoreSource, path::Path},
    item_tree::Name,
};

use crate::{
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
    NotConstructible {
        expression: ExpressionId,
        r#type: Type,
    },
    FunctionCallArgCountMismatch {
        expression: ExpressionId,
        n_expected: usize,
        n_actual: usize,
    },
    NoConstructor {
        expression: ExpressionId,
        r#type: Type,
        parameters: Vec<Type>,
    },
    NoOverload {
        expression: ExpressionId,
        name: Name,
        parameters: Vec<Type>,
    },
    StoreTypeMustBeStorable {
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
    UnexpectedLoweredKind {
        expression: ExpressionId,
        expected: LoweredKind,
        actual: LoweredKind,
        path: Path,
    },
    UnexpectedReturnValue {
        expression: ExpressionId,
        actual: Type,
    },
}
