use std::collections::VecDeque;

use hir_def::expression::ExpressionId;
use wgsl_types::{Instance, syntax::Enumerant};

use crate::{
    lower::{TypeContainer, TypeLoweringError, TypeLoweringErrorKind},
    ty::Type,
};

/// A single template parameter.
#[derive(Clone, Debug, PartialEq)]
pub enum TemplateParameter {
    Type(Type),
    /// The error instance is encoded as a `None`.
    Instance(Option<Instance>),
    Enumerant(Enumerant),
}

pub struct TemplateParameters {
    container: TypeContainer,
    inner: VecDeque<(TemplateParameter, ExpressionId)>,
    length: usize,
}

impl TemplateParameters {
    pub fn new(
        container: TypeContainer,
        inner: VecDeque<(TemplateParameter, ExpressionId)>,
    ) -> Self {
        let length = inner.len();
        Self {
            container,
            inner,
            length,
        }
    }

    pub fn has_next(&self) -> bool {
        !self.inner.is_empty()
    }

    pub fn next(&mut self) -> Option<(TemplateParameter, ExpressionId)> {
        self.inner.pop_front()
    }

    pub fn next_as_type(&mut self) -> Result<(Type, ExpressionId), TypeLoweringError> {
        match self.next() {
            Some((TemplateParameter::Type(r#type), id)) => Ok((r#type, id)),
            Some((_, id)) => Err(TypeLoweringError {
                container: TypeContainer::Expression(id),
                kind: TypeLoweringErrorKind::UnexpectedTemplateArgument("a type".to_owned()),
            }),
            None => Err(TypeLoweringError {
                container: self.container,
                kind: TypeLoweringErrorKind::MissingTemplateArgument("a type".to_owned()),
            }),
        }
    }

    pub fn next_as_instance(
        &mut self
    ) -> Result<(Option<Instance>, ExpressionId), TypeLoweringError> {
        match self.next() {
            Some((TemplateParameter::Instance(instance), id)) => Ok((instance, id)),
            Some((_, id)) => Err(TypeLoweringError {
                container: TypeContainer::Expression(id),
                kind: TypeLoweringErrorKind::UnexpectedTemplateArgument("an instance".to_owned()),
            }),
            None => Err(TypeLoweringError {
                container: self.container,
                kind: TypeLoweringErrorKind::MissingTemplateArgument("an instance".to_owned()),
            }),
        }
    }

    pub fn next_as_enumerant(&mut self) -> Result<(Enumerant, ExpressionId), TypeLoweringError> {
        match self.next() {
            Some((TemplateParameter::Enumerant(enumerant), id)) => Ok((enumerant, id)),
            Some((_, id)) => Err(TypeLoweringError {
                container: TypeContainer::Expression(id),
                kind: TypeLoweringErrorKind::UnexpectedTemplateArgument("an enum".to_owned()),
            }),
            None => Err(TypeLoweringError {
                container: self.container,
                kind: TypeLoweringErrorKind::MissingTemplateArgument("an enum".to_owned()),
            }),
        }
    }

    pub(crate) const fn len(&self) -> usize {
        self.length
    }

    pub const fn container(&self) -> &TypeContainer {
        &self.container
    }
}
