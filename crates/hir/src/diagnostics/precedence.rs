use hir_def::{
    db::DefinitionWithBodyId,
    expression::{ArithmeticOperation, BinaryOperation, ExpressionId},
};
use hir_ty::db::HirDatabase;

#[derive(Debug)]
pub enum PrecedenceDiagnostic {
    NeverNested(ExpressionId, BinaryOperation),
    SequencesAllowed(ExpressionId, BinaryOperation),
}

pub fn collect(
    db: &dyn HirDatabase,
    body: DefinitionWithBodyId,
    mut f: impl FnMut(PrecedenceDiagnostic),
) {
    let (body, _) = db.body_with_source_map(body);

    for (_, expression) in body.exprs.iter() {
        // See https://github.com/gpuweb/gpuweb/issues/1146#issuecomment-714721825
        let hir_def::expression::Expression::BinaryOperation {
            operation,
            left_side,
            right_side,
        } = expression
        else {
            continue;
        };

        let not_paren = |v| !body.parenthesis_expressions.contains(v);

        let lhs_op =
            if let hir_def::expression::Expression::BinaryOperation { operation: op, .. } =
                body.exprs[*left_side]
            {
                not_paren(left_side).then_some(op)
            } else {
                None
            };
        let rhs_op =
            if let hir_def::expression::Expression::BinaryOperation { operation: op, .. } =
                body.exprs[*right_side]
            {
                not_paren(right_side).then_some(op)
            } else {
                None
            };
        let op = *operation;
        // We have validation for the following cases:
        // - &, | and ^ having (different) binary children
        // - >> and << having binary children
        // - <, >, <=, >=, ==, != being mixed within the group
        // - && and || being mixed

        // &, | and ^ having (different) binary children
        if let BinaryOperation::Arithmetic(
            ArithmeticOperation::BitAnd | ArithmeticOperation::BitXor | ArithmeticOperation::BitOr,
        ) = op
        {
            if let Some(lhs_op) = lhs_op {
                if lhs_op != op {
                    f(PrecedenceDiagnostic::SequencesAllowed(*left_side, op))
                }
            }
            if let Some(rhs_op) = rhs_op {
                if rhs_op != op {
                    f(PrecedenceDiagnostic::SequencesAllowed(*right_side, op))
                }
            }
        }

        // >> and << having binary children
        if let BinaryOperation::Arithmetic(ArithmeticOperation::Shl | ArithmeticOperation::Shr) = op
        {
            if lhs_op.is_some() {
                f(PrecedenceDiagnostic::NeverNested(*left_side, op))
            }
            if rhs_op.is_some() {
                f(PrecedenceDiagnostic::NeverNested(*right_side, op))
            }
        }

        // <, >, <=, >=, ==, != being mixed
        if let BinaryOperation::Comparison(_) = op {
            if let Some(BinaryOperation::Comparison(_)) = lhs_op {
                f(PrecedenceDiagnostic::NeverNested(*left_side, op))
            }
            if let Some(BinaryOperation::Comparison(_)) = rhs_op {
                f(PrecedenceDiagnostic::NeverNested(*right_side, op))
            }
        }

        // && and || being mixed
        if let BinaryOperation::Logical(_) = op {
            if let Some(lhs_op @ BinaryOperation::Logical(_)) = lhs_op {
                if lhs_op != op {
                    f(PrecedenceDiagnostic::SequencesAllowed(*left_side, op))
                }
            }
            if let Some(rhs_op @ BinaryOperation::Logical(_)) = rhs_op {
                if rhs_op != op {
                    f(PrecedenceDiagnostic::SequencesAllowed(*right_side, op))
                }
            }
        }
    }
}
