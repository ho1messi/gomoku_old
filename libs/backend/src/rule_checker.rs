use std::rc::*;
use std::cell::*;
use std::collections::HashMap;
use std::collections::VecDeque;

use super::evaluation_dfa::*;
use super::board::*;
use super::tuple::*;
use super::cross_point::*;

use super::board::MoveDirection::*;
use super::cross_point::CrossPointType::*;
use super::cross_point::ChessType::*;

use self::GameStatus::*;
use self::MovesEndReason::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum GameStatus {
    GsGameOver(ChessType),
    GsGameContinue,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum MovesEndReason {
    MerBoarder(Coord),
    MerDifferenceChess(CoordAndChess),
    MerEnoughChess(Coord),
}

impl MovesEndReason {
    pub fn coord(&self) -> Coord {
        match self {
            MerBoarder(coord) => return *coord,
            MerDifferenceChess(coord_and_chess) => return coord_and_chess.coord,
            MerEnoughChess(coord) => return *coord,
        }
    }
}


pub struct RuleChecker {
    board: Weak<RefCell<Board>>,
    status: GameStatus,
    score: i32,
    tuples: Vec<Tuple>,
    tuple_indices: HashMap<MoveDirection, usize>,
    self_weak: Weak<RefCell<RuleChecker>>,
}

impl BoardObserver for RuleChecker {
    fn board_updated(&mut self, event: BoardEvent) {
        let check_directions = vec![
            (MdLeft, MdRight), (MdUp, MdDown),
            (MdUpLeft, MdDownRight), (MdUpRight, MdDownLeft)
        ];

        let coord = event.get_coord();
        for direction in check_directions.iter() {
            self.update_line_evaluation(coord, direction.0, direction.1, event);
        }
    }
}

impl RuleChecker {
    pub fn create_with_detail(board: Weak<RefCell<Board>>) -> Rc<RefCell<Self>> {
        let mut rule_checker = RuleChecker {
            board,
            status: GsGameContinue,
            score: 0,
            tuples: Vec::new(),
            tuple_indices: HashMap::new(),
            self_weak: Weak::new(),
        };

        rule_checker.set_all_tuples();
        let rule_checker_rc = Rc::new(RefCell::new(rule_checker));
        rule_checker_rc.borrow_mut().self_weak = Rc::downgrade(&rule_checker_rc.clone());
        return rule_checker_rc;
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
        return self.score;
    }

    fn get_board_rc(&self) -> Rc<RefCell<Board>> {
        match self.board.upgrade() {
            Some(board_rc) => return board_rc,
            None => panic!("Can not upgrade weak board to rc!"),
        }
    }

    fn set_all_tuples(&mut self) {
        let board = self.get_board_rc();
        let board_cp_count = board.borrow().size();
        let board_tp_count = board_cp_count - 5;
        let mut index;

        index = self.set_tuples(MdRight, Coord{row: 0, col: 0},
                                Coord{row: board_cp_count, col: board_tp_count});
        self.tuple_indices.insert(MdRight, index);

        index = self.set_tuples(MdDown, Coord{row: 0, col: 0},
                                Coord{row: board_tp_count, col: board_cp_count});
        self.tuple_indices.insert(MdDown, index);

        index = self.set_tuples(MdDownRight, Coord{row: 0, col: 0},
                                Coord{row: board_tp_count, col: board_tp_count});
        self.tuple_indices.insert(MdDownRight, index);

        index = self.set_tuples(MdDownLeft, Coord{row: 0, col: 4},
                                Coord{row: board_tp_count, col: board_tp_count});
        self.tuple_indices.insert(MdDownLeft, index);
    }

    fn set_tuples(&mut self, md: MoveDirection, offset: Coord, count: Coord) -> usize {
        let end = offset + count;
        let index = self.tuples.len();

        for row in offset.row..end.row {
            for col in offset.col..end.col {
                let board_rc = self.get_board_rc();
                self.tuples.push(Tuple::create_with_md(5, board_rc, Coord{row, col}, md));
            }
        }

        return index;
    }

    fn update_line_evaluation(&mut self, coord: Coord, md1: MoveDirection,
                              md2: MoveDirection, event: BoardEvent) {
        /*
        let mut line = VecDeque::new();
        let mut coord_e = event.get_coord();
        let chess = event.get_chess();
        */


        /*
        let mut line = Vec::new();
        let chess = event.get_chess();
        let mut row_t = row; let mut col_t = col;

        unsafe {
            let flag1 = self.move_until(&mut row_t, &mut col_t, md1, chess, &line);
            line.clear();
            let flag2 = self.move_until(&mut row_t, &mut col_t, md2, chess, &line);
        }
        */

    }

    /*
    unsafe fn move_until(&self, row: &mut usize, col: &mut usize,
                         md: MoveDirection, chess: ChessType,
                         cross_points: &mut Vec<CrossPointType>) -> MovesEndReason {
        let mut num = 1;

        /*
        while true {
            *cross_points.push(*self.board.get_cross_point_type_at(*row, *col));

            match *self.board.move_to(*row, *col, md) {
                Ok(coord) => {row = coord.0; col = coord.1},
                Err(_) => break,
            }

            if *self.board.have_chess_at(*row, *col) {
                if chess == *self.board.get_chess_at(*row, *col) {
                    num += 1;
                } else {
                    return MerDifferenceChess;
                }
            } else {
                if num == 5 {
                    return MerEnoughChess;
                } else {
                    num += 1;
                }
            }
        }
        */

        return MerBoarder;
    }
    */

    //unsafe fn tuple_evaluation(&mut self, mut row: usize, mut col: usize, md: MoveDirection, )
}
