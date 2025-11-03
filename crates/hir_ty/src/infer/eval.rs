use std::{collections::VecDeque, str::FromStr};

use hir_def::{
    expression::{BinaryOperation, Expression, ExpressionId, UnaryOperator},
    expression_store::ExpressionStore,
    resolver::ResolveKind,
    type_specifier::TypeSpecifier,
};
use wgsl_types::{
    inst::{Instance, LiteralInstance},
    syntax::Enumerant,
};

use crate::{
    database::HirDatabase as _,
    infer::{
        InferenceContext, InferenceDiagnostic, Lowered, TyLoweringContext, TypeContainer,
        TypeLoweringError, TypeLoweringErrorKind,
    },
    ty::{TyKind, Type},
};

impl TyLoweringContext<'_> {
    /// Used for template checking.
    /// There, many expressions are guaranteed to evaluate to a type, or a scalar.
    /// e.g. `array<f32, 3 + 5>`
    /// `None` is returned for the "error" instance
    fn eval_expression(
        &mut self,
        expression: ExpressionId,
    ) -> Option<Instance> {
        let instance: Instance = match &self.store[expression] {
            #[expect(
                clippy::match_same_arms,
                reason = "TODO: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/659"
            )]
            Expression::Missing => {
                // TODO: self.push_diagnostic(InferenceDiagnostic::...)
                return None;
            },
            Expression::BinaryOperation {
                left_side,
                right_side,
                operation,
            } => self.eval_binary_op(*left_side, *right_side, *operation)?,
            Expression::UnaryOperator { expression, op } => self.eval_unary_op(*expression, *op)?,
            Expression::Field { .. } | Expression::Index { .. } => {
                // Not implemented
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
                        // TODO: Not implemented
                        // See: https://github.com/wgsl-analyzer/wgsl-analyzer/issues/670
                        return None;
                    },
                    Literal::Bool(value) => Instance::Literal(LiteralInstance::Bool(*value)),
                }
            },
            Expression::Call { .. } => {
                unimplemented!();
                /*let template_args = type_specifier
                .generics
                .iter()
                .map(|arg| self.eval_tplt_arg(*arg))
                .collect(); */
                return None;
            },
            Expression::IdentExpression(ident_expression) => {
                let resolved_ty = self.resolver.resolve(&ident_expression.path);
                todo!("do something useful here");
                match &resolved_ty {
                    /*
                    Some(ResolveType::GlobalConstant(_)) => todo!(),
                    Some(ResolveType::GlobalVariable(_)) => todo!(),
                    Some(ResolveType::Override(_)) => todo!(),
                    Some(ResolveType::Local(_)) => todo!(),
                    None => todo!("search for predeclared idents"), */
                    _ => {
                        // self.push_diagnostic(InferenceDiagnostic::...);
                        return None;
                    },
                }
            },
        };

        Some(instance)
    }

    #[expect(
        clippy::needless_pass_by_ref_mut,
        clippy::unused_self,
        reason = "TODO, see below"
    )]
    fn eval_binary_op(
        &mut self,
        _left: ExpressionId,
        _right: ExpressionId,
        _operation: BinaryOperation,
    ) -> Option<Instance> {
        todo!(r#"Implement according to `impl Eval for BinaryExpression` in wesl-rs""#);
        unimplemented!();
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

    pub fn eval_tplt_arg(
        &mut self,
        tplt: ExpressionId,
    ) -> TemplateParameter {
        match &self.store[tplt] {
            Expression::IdentExpression(ident_expression) => {
                let resolved_ty = self.lower(
                    TypeContainer::Expression(tplt),
                    &ident_expression.path,
                    &ident_expression.template_parameters,
                );
                match resolved_ty {
                    Lowered::Type(r#type) => TemplateParameter::Type(r#type),
                    Lowered::TypeWithoutTemplate(_) => {
                        self.diagnostics.push(TypeLoweringError {
                            container: TypeContainer::Expression(tplt),
                            kind: TypeLoweringErrorKind::MissingTemplate,
                        });
                        TemplateParameter::Type(TyKind::Error.intern(self.database))
                    },
                    Lowered::Enumerant(enumerant) => TemplateParameter::Enumerant(enumerant),
                    Lowered::Function(_) => {
                        todo!(r#"Report an error "function needs to be called""#);
                        TemplateParameter::Type(self.database.intern_ty(TyKind::Error))
                    },
                    Lowered::BuiltinFunction => {
                        todo!(r#"Report an error "function needs to be called""#);
                        TemplateParameter::Type(self.database.intern_ty(TyKind::Error))
                    },
                    Lowered::GlobalConstant(_)
                    | Lowered::GlobalVariable(_)
                    | Lowered::Override(_)
                    | Lowered::Local(_) => TemplateParameter::Instance(self.eval_expression(tplt)),
                }
            },
            Expression::Missing
            | Expression::BinaryOperation { .. }
            | Expression::UnaryOperator { .. }
            | Expression::Field { .. }
            | Expression::Call { .. }
            | Expression::Index { .. }
            | Expression::Literal(_) => TemplateParameter::Instance(self.eval_expression(tplt)),
        }
    }

    pub fn eval_template_args(
        &mut self,
        container: TypeContainer,
        template_parameters: &[ExpressionId],
    ) -> TemplateParameters {
        let template_parameters: VecDeque<_> = template_parameters
            .iter()
            .map(|arg| (self.eval_tplt_arg(*arg), *arg))
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
    /// The error instance is encoded as a None
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
                container: self.container.clone(),
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
                container: self.container.clone(),
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
                container: self.container.clone(),
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
