use hir_def::{
    db::DefWithBodyId,
    expr::{ArithOp, BinaryOp, ExprId},
};
use hir_ty::HirDatabase;

#[derive(Debug)]
pub enum PrecedenceDiagnostic {
    NeverNested(ExprId, BinaryOp),
    SequencesAllowed(ExprId, BinaryOp),
}

pub fn collect(
    db: &dyn HirDatabase,
    body: DefWithBodyId,
    mut f: impl FnMut(PrecedenceDiagnostic),
) {
    let (body, _) = db.body_with_source_map(body);

    for (_, expr) in body.exprs.iter() {
        // See https://github.com/gpuweb/gpuweb/issues/1146#issuecomment-714721825
        let hir_def::expr::Expr::BinaryOp { op, lhs, rhs } = expr else {
            continue;
        };

        let not_paren = |v| !body.paren_exprs.contains(v);

        let lhs_op = if let hir_def::expr::Expr::BinaryOp { op, .. } = body.exprs[*lhs] {
            not_paren(lhs).then_some(op)
        } else {
            None
        };
        let rhs_op = if let hir_def::expr::Expr::BinaryOp { op, .. } = body.exprs[*rhs] {
            not_paren(rhs).then_some(op)
        } else {
            None
        };
        let op = *op;
        // We have validation for the following cases:
        // - &, | and ^ having (different) binary children
        // - >> and << having binary children
        // - <, >, <=, >=, ==, != being mixed within the group
        // - && and || being mixed

        // &, | and ^ having (different) binary children
        if let BinaryOp::ArithOp(ArithOp::BitAnd | ArithOp::BitXor | ArithOp::BitOr) = op {
            if let Some(lhs_op) = lhs_op {
                if lhs_op != op {
                    f(PrecedenceDiagnostic::SequencesAllowed(*lhs, op))
                }
            }
            if let Some(rhs_op) = rhs_op {
                if rhs_op != op {
                    f(PrecedenceDiagnostic::SequencesAllowed(*rhs, op))
                }
            }
        }

        // >> and << having binary children
        if let BinaryOp::ArithOp(ArithOp::Shl | ArithOp::Shr) = op {
            if lhs_op.is_some() {
                f(PrecedenceDiagnostic::NeverNested(*lhs, op))
            }
            if rhs_op.is_some() {
                f(PrecedenceDiagnostic::NeverNested(*rhs, op))
            }
        }

        // <, >, <=, >=, ==, != being mixed
        if let BinaryOp::CmpOp(_) = op {
            if let Some(BinaryOp::CmpOp(_)) = lhs_op {
                f(PrecedenceDiagnostic::NeverNested(*lhs, op))
            }
            if let Some(BinaryOp::CmpOp(_)) = rhs_op {
                f(PrecedenceDiagnostic::NeverNested(*rhs, op))
            }
        }

        // && and || being mixed
        if let BinaryOp::LogicOp(_) = op {
            if let Some(lhs_op @ BinaryOp::LogicOp(_)) = lhs_op {
                if lhs_op != op {
                    f(PrecedenceDiagnostic::SequencesAllowed(*lhs, op))
                }
            }
            if let Some(rhs_op @ BinaryOp::LogicOp(_)) = rhs_op {
                if rhs_op != op {
                    f(PrecedenceDiagnostic::SequencesAllowed(*rhs, op))
                }
            }
        }
    }
}
