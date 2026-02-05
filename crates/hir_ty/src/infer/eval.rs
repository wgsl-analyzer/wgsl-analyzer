use std::collections::VecDeque;

use hir_def::expression::{BinaryOperation, Expression, ExpressionId, UnaryOperator};
use wgsl_types::{
    inst::{Instance, LiteralInstance},
    syntax::Enumerant,
};

use crate::{
    infer::{
        Lowered, TypeContainer, TypeLoweringContext, TypeLoweringError, TypeLoweringErrorKind,
    },
    ty::{Type, TypeKind},
};

impl TypeLoweringContext<'_> {
    /// Used for template checking.
    /// There, many expressions are guaranteed to evaluate to a type, or a scalar.
    /// For example, `array<f32, 3 + 5>`.
    /// `None` is returned for the "error" instance.
    fn eval_expression(
        &mut self,
        expression: ExpressionId,
    ) -> Option<Instance> {
        let instance: Instance = match &self.store[expression] {
            Expression::Missing => {
                return None; // missing expression are a parser error
            },
            Expression::BinaryOperation {
                left_side,
                right_side,
                operation,
            } => self.eval_binary_op(*left_side, *right_side, *operation)?,
            Expression::UnaryOperator {
                expression,
                operator,
            } => self.eval_unary_op(*expression, *operator)?,
            #[expect(
                clippy::match_same_arms,
                reason = "TODO: const evaluation not implemented, see https://github.com/wgsl-analyzer/wgsl-analyzer/issues/670"
            )]
            Expression::Field { .. }
            | Expression::Index { .. }
            | Expression::Call { .. }
            | Expression::IdentExpression(_) => {
                return None;
            },
            #[expect(
                clippy::cast_possible_truncation,
                clippy::cast_possible_wrap,
                clippy::as_conversions,
                reason = "TODO: make invalid state unrepresentable; see: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/675"
            )]
            Expression::Literal(literal) => {
                use hir_def::expression::{BuiltinInt, Literal};
                match literal {
                    Literal::Int(value, BuiltinInt::I32) => {
                        Instance::Literal(LiteralInstance::I32(*value as i32))
                    },
                    Literal::Int(value, BuiltinInt::U32) => {
                        Instance::Literal(LiteralInstance::U32(*value as u32))
                    },
                    Literal::Int(value, BuiltinInt::Abstract) => {
                        Instance::Literal(LiteralInstance::AbstractInt(*value as i64))
                    },
                    Literal::Float(_, _) => {
                        // TODO: const evaluation not implemented
                        // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/670
                        return None;
                    },
                    Literal::Bool(value) => Instance::Literal(LiteralInstance::Bool(*value)),
                }
            },
        };

        Some(instance)
    }

    #[expect(
        clippy::needless_pass_by_ref_mut,
        clippy::unused_self,
        clippy::missing_const_for_fn,
        reason = "TODO, see below"
    )]
    fn eval_binary_op(
        &mut self,
        _left: ExpressionId,
        _right: ExpressionId,
        _operation: BinaryOperation,
    ) -> Option<Instance> {
        // TODO: const evaluation not implemented
        // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/670
        // Implement according to `impl Eval for BinaryExpression` in wesl-rs
        None
    }

    fn eval_unary_op(
        &mut self,
        expression: ExpressionId,
        operator: UnaryOperator,
    ) -> Option<Instance> {
        let operator: wgsl_types::syntax::UnaryOperator = match operator {
            UnaryOperator::Negation => wgsl_types::syntax::UnaryOperator::Negation,
            UnaryOperator::LogicalNegation => wgsl_types::syntax::UnaryOperator::LogicalNegation,
            UnaryOperator::AddressOf => wgsl_types::syntax::UnaryOperator::AddressOf,
            UnaryOperator::Indirection => wgsl_types::syntax::UnaryOperator::Indirection,
            UnaryOperator::BitwiseComplement => {
                wgsl_types::syntax::UnaryOperator::BitwiseComplement
            },
        };

        // Copied from wesl-rs
        let operand = self.eval_expression(expression)?;
        if operator == wgsl_types::syntax::UnaryOperator::AddressOf {
            operand.op_ref().ok()
        } else {
            let operand = operand.loaded().ok()?;
            wgsl_types::builtin::call_unary_op(operator, &operand).ok()
        }
    }

    pub fn evaluate_template_argument(
        &mut self,
        template_argument: ExpressionId,
    ) -> TemplateParameter {
        match &self.store[template_argument] {
            Expression::IdentExpression(ident_expression) => {
                let resolved_type = self.lower(
                    TypeContainer::Expression(template_argument),
                    &ident_expression.path,
                    &ident_expression.template_parameters,
                );
                match resolved_type {
                    Lowered::Type(r#type) => TemplateParameter::Type(r#type),
                    Lowered::TypeWithoutTemplate(_) => {
                        self.diagnostics.push(TypeLoweringError {
                            container: TypeContainer::Expression(template_argument),
                            kind: TypeLoweringErrorKind::MissingTemplate,
                        });
                        TemplateParameter::Type(TypeKind::Error.intern(self.database))
                    },
                    Lowered::Enumerant(enumerant) => TemplateParameter::Enumerant(enumerant),
                    Lowered::Function(_) | Lowered::BuiltinFunction => {
                        // function<another_function>()
                        self.diagnostics.push(TypeLoweringError {
                            container: TypeContainer::Expression(template_argument),
                            kind: TypeLoweringErrorKind::ExpectedFunctionToBeCalled(
                                ident_expression.path.clone(),
                            ),
                        });
                        TemplateParameter::Type(self.database.intern_type(TypeKind::Error))
                    },
                    Lowered::GlobalConstant(_)
                    | Lowered::GlobalVariable(_)
                    | Lowered::Override(_)
                    | Lowered::Local(_) => {
                        TemplateParameter::Instance(self.eval_expression(template_argument))
                    },
                }
            },
            Expression::Missing
            | Expression::BinaryOperation { .. }
            | Expression::UnaryOperator { .. }
            | Expression::Field { .. }
            | Expression::Call { .. }
            | Expression::Index { .. }
            | Expression::Literal(_) => {
                TemplateParameter::Instance(self.eval_expression(template_argument))
            },
        }
    }

    pub fn eval_template_args(
        &mut self,
        container: TypeContainer,
        template_parameters: &[ExpressionId],
    ) -> TemplateParameters {
        let template_parameters: VecDeque<_> = template_parameters
            .iter()
            .map(|argument| (self.evaluate_template_argument(*argument), *argument))
            .collect();
        let length = template_parameters.len();
        TemplateParameters {
            container,
            inner: template_parameters,
            length,
        }
    }
}

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
