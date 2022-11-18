use hir_def::{
    db::DefWithBodyId,
    expr::{ArithOp, BinaryOp, ExprId},
};
use hir_ty::HirDatabase;

#[derive(Debug)]
pub enum PrecedenceDiagnostic {
    BracesRequired(ExprId),
}

pub fn collect(db: &dyn HirDatabase, body: DefWithBodyId, mut f: impl FnMut(PrecedenceDiagnostic)) {
    let (body, _) = db.body_with_source_map(body);
    for (_, expr) in body.exprs.iter() {
        match expr {
            hir_def::expr::Expr::BinaryOp {
                op: BinaryOp::ArithOp(ArithOp::Shl | ArithOp::Shr),
                lhs,
                rhs,
            } => {
                if let hir_def::expr::Expr::BinaryOp { .. } = body.exprs[*lhs] {
                    f(PrecedenceDiagnostic::BracesRequired(*lhs))
                }
                if let hir_def::expr::Expr::BinaryOp { .. } = body.exprs[*rhs] {
                    f(PrecedenceDiagnostic::BracesRequired(*rhs))
                }
            }
            _ => (),
        };
    }
}
