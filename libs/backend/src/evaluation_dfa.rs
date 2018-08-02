use cross_point::*;
use board::*;

use cross_point::CrossPointType::*;
use board::BoardEvent::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
enum DfaStatus {

}

pub struct EvaluationDfa {

}

impl EvaluationDfa {
    pub fn new() -> EvaluationDfa {
        return EvaluationDfa{};
    }

    pub fn evaluate_event(&self, cross_points: &mut [CrossPointType],
                          index: usize, event: BoardEvent) -> i32 {
        //println!("cross point type is {:?}", cross_points[index]);

        match event {
            BePutChess(_) => {
                let new_score = self.evaluate_tuple(cross_points);
                cross_points[index] = CptEmpty;
                let old_score = self.evaluate_tuple(cross_points);
                return new_score - old_score;
            },
            BeRemoveChess(coord_and_chess) => {
                let new_score = self.evaluate_tuple(cross_points);
                cross_points[index] = CptChess(coord_and_chess.chess);
                let old_score = self.evaluate_tuple(cross_points);
                return new_score - old_score;
            }
        }
    }

    pub fn evaluate_tuple(&self, cross_points: &mut [CrossPointType]) -> i32 {
        return 9;
    }
}
