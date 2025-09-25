use std::str::FromStr;

use hir_def::{
    expression::{BinaryOperation, Expression, ExpressionId, UnaryOperator},
    expression_store::ExpressionStore,
    resolver::ResolveType,
};
use wgsl_types::{
    inst::{Instance, LiteralInstance},
    syntax::Enumerant,
};

use crate::{
    infer::{InferenceContext, InferenceDiagnostic, TyLoweringContext, TypeContainer},
    ty::Type,
};

impl<'database> TyLoweringContext<'database> {
    /// Used for template checking.
    /// There, many expressions are guaranteed to evaluate to a type, or a scalar.
    /// e.g. `array<(f32), 3 + 5>`
    /// `None` is returned for the "error" instance
    fn eval_expression(
        &mut self,
        expression: ExpressionId,
    ) -> Option<Instance> {
        let instance: Instance = match &self.store.exprs[expression] {
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

            Expression::Literal(literal) => {
                match literal {
                    hir_def::expression::Literal::Int(value, _) => {
                        Instance::Literal(LiteralInstance::I32(*value as i32))
                    },
                    hir_def::expression::Literal::Uint(value, _) => {
                        Instance::Literal(LiteralInstance::U32(*value as u32))
                    },
                    hir_def::expression::Literal::Float(_, _) => {
                        // Not implemented
                        return None;
                    },
                    hir_def::expression::Literal::Bool(value) => {
                        Instance::Literal(LiteralInstance::Bool(*value))
                    },
                }
            },
            Expression::Call { .. } => {
                // Not implemented
                /*let template_args = type_specifier
                .generics
                .iter()
                .map(|arg| self.eval_tplt_arg(*arg))
                .collect(); */
                return None;
            },
            Expression::TypeSpecifier(type_specifier) => {
                let resolved_ty = self.resolver.resolve_type(&type_specifier.path);
                match &resolved_ty {
                    // TODO: Do something useful here
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

    fn eval_binary_op(
        &mut self,
        _left: ExpressionId,
        _right: ExpressionId,
        _operation: BinaryOperation,
    ) -> Option<Instance> {
        // Not implemented
        // TODO: Implement according to `impl Eval for BinaryExpression` in wesl-rs
        return None;
    }
    fn eval_unary_op(
        &mut self,
        expression: ExpressionId,
        operator: UnaryOperator,
    ) -> Option<Instance> {
        let operator: wgsl_types::syntax::UnaryOperator = match operator {
            UnaryOperator::Minus => wgsl_types::syntax::UnaryOperator::Negation,
            UnaryOperator::Not => wgsl_types::syntax::UnaryOperator::LogicalNegation,
            UnaryOperator::Reference => wgsl_types::syntax::UnaryOperator::AddressOf,
            UnaryOperator::Dereference => wgsl_types::syntax::UnaryOperator::Indirection,
            UnaryOperator::BitNot => wgsl_types::syntax::UnaryOperator::BitwiseComplement,
        };

        // Copied from wesl-rs
        if operator == wgsl_types::syntax::UnaryOperator::AddressOf {
            let operand = self.eval_expression(expression)?;
            operand.op_ref().ok()
        } else {
            let operand = self.eval_expression(expression)?;
            let operand = operand.loaded().ok()?;
            wgsl_types::builtin::call_unary_op(operator, &operand).ok()
        }
    }

    pub fn eval_tplt_arg(
        &mut self,
        tplt: ExpressionId,
    ) -> TpltParam {
        let template_param = match &self.store.exprs[tplt] {
            Expression::TypeSpecifier(ty) => {
                let resolved_ty = self.resolver.resolve_type(&ty.path);
                match &resolved_ty {
                    Some(ResolveType::Struct(_)) | Some(ResolveType::TypeAlias(_)) => {
                        TpltParam::Type(self.lower_ty(ty))
                    },
                    None => {
                        if self.is_predeclared_ty(&ty.path) {
                            TpltParam::Type(self.lower_ty(ty))
                        } else if let Ok(enum_value) = Enumerant::from_str(ty.path.as_str()) {
                            if !ty.generics.is_empty() {
                                // TODO: Report error for such enumerants. Maybe return an error_ty?
                            }
                            TpltParam::Enumerant(enum_value)
                        } else {
                            TpltParam::Instance(self.eval_expression(tplt))
                        }
                    },
                    _ => TpltParam::Instance(self.eval_expression(tplt)),
                }
            },
            _ => TpltParam::Instance(self.eval_expression(tplt)),
        };

        template_param
    }
}

/// A single template parameter.
#[derive(Clone, Debug, PartialEq)]
pub enum TpltParam {
    Type(Type),
    /// The error instance is encoded as a None
    Instance(Option<Instance>),
    Enumerant(Enumerant),
}
