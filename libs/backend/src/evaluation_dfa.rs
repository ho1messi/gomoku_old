use super::cross_point::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum DfaStatus {

}

pub struct EvaluationDfa {

}

impl EvaluationDfa {
    pub fn new() -> EvaluationDfa {
        return EvaluationDfa{};
    }
    pub fn evaluate(cross_points: &[CrossPointType], chess: ChessType) -> i32 {
        return 0;
    }
}
