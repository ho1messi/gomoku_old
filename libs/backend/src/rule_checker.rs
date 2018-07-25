use std::collections::HashMap;

use super::board::*;
use super::tuple::*;
use super::cross_point::*;

use super::board::MoveDirection::*;
use super::cross_point::CrossPointType::*;
use super::cross_point::ChessType::*;

use self::GameStatus::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum GameStatus {
    GsGameOver(ChessType),
    GsGameContinue,
}

pub struct RuleChecker<'a> {
    board: *const Board,
    status: GameStatus,
    score: i32,
    tuples: Vec<Tuple<'a>>,
    tuple_indices: HashMap<MoveDirection, usize>,
}

impl<'a> RuleChecker<'a> {
    pub fn create_with_detail(board: *const Board) -> Self {
        let mut rule_checker = RuleChecker {
            board,
            status: GsGameContinue,
            score: 0,
            tuples: Vec::new(),
            tuple_indices: HashMap::new(),
        };

        rule_checker.set_all_tuples();
        return rule_checker;
    }

    pub fn check_game_status(&mut self) -> GameStatus {
        for tuple in self.tuples.iter() {
            if tuple.count(CptChess(CtBlack)) == 5 {
                self.status = GsGameOver(CtBlack);
                return self.status;
            } else if tuple.count(CptChess(CtWhite)) == 5 {
                self.status = GsGameOver(CtWhite);
                return self.status;
            }
        }

        return GsGameContinue;
    }

    pub fn game_status(&self) -> GameStatus {
        return self.status;
    }

    pub fn get_evaluation(&self) -> i32 {
        return 0;
    }

    pub fn update_option_evaluation(&mut self, row: usize, col: usize, op: BoardOperation) {
        println!("======================================");
        println!("evaluation score updated!");
        println!("======================================");
    }

    fn set_all_tuples(&mut self) {
        let board_cp_count = unsafe { (*self.board).size() };
        let board_tp_count = board_cp_count - 5;
        let mut index;

        index = self.set_tuples(MdRight, 0, 0,
                                board_cp_count, board_tp_count);
        self.tuple_indices.insert(MdRight, index);

        index = self.set_tuples(MdDown, 0, 0,
                                board_tp_count, board_cp_count);
        self.tuple_indices.insert(MdDown, index);

        index = self.set_tuples(MdDownRight, 0, 0,
                                board_tp_count, board_tp_count);
        self.tuple_indices.insert(MdDownRight, index);

        index = self.set_tuples(MdDownLeft, 0, 4,
                                board_tp_count, board_tp_count);
        self.tuple_indices.insert(MdDownLeft, index);
    }

    fn set_tuples(&mut self, md: MoveDirection, row_offset: usize, col_offset: usize,
                  row_count: usize, col_count: usize) -> usize {
        let (row_end, col_end) = (row_offset + row_count, col_offset + col_count);
        let index = self.tuples.len();

        for row in row_offset..row_end {
            for col in col_offset..col_end {
                unsafe {
                    self.tuples.push(Tuple::create_with_md(5, &*self.board,
                                                           row, col, md));
                }
            }
        }

        return index;
    }
}
