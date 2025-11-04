use hir_def::{
    database::DefinitionWithBodyId,
    expression::{ArithmeticOperation, BinaryOperation, ExpressionId},
};
use hir_ty::database::HirDatabase;

#[derive(Debug)]
pub enum PrecedenceDiagnostic {
    NeverNested(ExpressionId, BinaryOperation),
    SequencesAllowed(ExpressionId, BinaryOperation),
}

pub fn collect<Function: FnMut(PrecedenceDiagnostic)>(
    database: &dyn HirDatabase,
    body: DefinitionWithBodyId,
    mut diagnostic_builder: Function,
) {
    let (body, _) = database.body_with_source_map(body);

    for (_, expression) in body.store.exprs.iter() {
        // See https://github.com/gpuweb/gpuweb/issues/1146#issuecomment-714721825
        let hir_def::expression::Expression::BinaryOperation {
            operation,
            left_side,
            right_side,
        } = expression
        else {
            continue;
        };

        let not_parenthesis = |index| !body.store.parenthesis_expressions.contains(index);

        let left_hand_side_operator =
            if let hir_def::expression::Expression::BinaryOperation { operation, .. } =
                body.store[*left_side]
            {
                not_parenthesis(left_side).then_some(operation)
            } else {
                None
            };
        let right_hand_side_operator =
            if let hir_def::expression::Expression::BinaryOperation { operation, .. } =
                body.store[*right_side]
            {
                not_parenthesis(right_side).then_some(operation)
            } else {
                None
            };
        let operation = *operation;
        // We have validation for the following cases:
        // - &, | and ^ having (different) binary children
        // - >> and << having binary children
        // - <, >, <=, >=, ==, != being mixed within the group
        // - && and || being mixed

        // &, | and ^ having (different) binary children
        if let BinaryOperation::Arithmetic(
            ArithmeticOperation::BitwiseAnd
            | ArithmeticOperation::BitwiseXor
            | ArithmeticOperation::BitwiseOr,
        ) = operation
        {
            if let Some(lhs_op) = left_hand_side_operator
                && lhs_op != operation
            {
                diagnostic_builder(PrecedenceDiagnostic::SequencesAllowed(
                    *left_side, operation,
                ));
            }
            if let Some(rhs_op) = right_hand_side_operator
                && rhs_op != operation
            {
                diagnostic_builder(PrecedenceDiagnostic::SequencesAllowed(
                    *right_side,
                    operation,
                ));
            }
        }

        // >> and << having binary children
        if let BinaryOperation::Arithmetic(
            ArithmeticOperation::ShiftLeft | ArithmeticOperation::ShiftRight,
        ) = operation
        {
            if left_hand_side_operator.is_some() {
                diagnostic_builder(PrecedenceDiagnostic::NeverNested(*left_side, operation));
            }
            if right_hand_side_operator.is_some() {
                diagnostic_builder(PrecedenceDiagnostic::NeverNested(*right_side, operation));
            }
        }

        // <, >, <=, >=, ==, != being mixed
        if let BinaryOperation::Comparison(_) = operation {
            if let Some(BinaryOperation::Comparison(_)) = left_hand_side_operator {
                diagnostic_builder(PrecedenceDiagnostic::NeverNested(*left_side, operation));
            }
            if let Some(BinaryOperation::Comparison(_)) = right_hand_side_operator {
                diagnostic_builder(PrecedenceDiagnostic::NeverNested(*right_side, operation));
            }
        }

        // && and || being mixed
        if let BinaryOperation::Logical(_) = operation {
            if let Some(lhs_op @ BinaryOperation::Logical(_)) = left_hand_side_operator
                && lhs_op != operation
            {
                diagnostic_builder(PrecedenceDiagnostic::SequencesAllowed(
                    *left_side, operation,
                ));
            }
            if let Some(rhs_op @ BinaryOperation::Logical(_)) = right_hand_side_operator
                && rhs_op != operation
            {
                diagnostic_builder(PrecedenceDiagnostic::SequencesAllowed(
                    *right_side,
                    operation,
                ));
            }
        }
    }
}
