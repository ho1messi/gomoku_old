use std::rc::*;
use std::cell::*;
use std::collections::HashMap;
use std::collections::VecDeque;

use super::evaluation_dfa::*;
use super::board::*;
use super::tuple::*;
use super::cross_point::*;
use super::utils::*;

use super::board::MoveDirection::*;
use super::cross_point::CrossPointType::*;
use super::cross_point::ChessType::*;

use self::GameStatus::*;
use self::MoveFailedType::*;
use self::MoveResult::*;

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum GameStatus {
    GsGameOver(ChessType),
    GsGameContinue,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum MoveFailedType {
    MftBoarder,
    MftDifferenceChess,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum MoveResult {
    MrSuccessful(CoordAndCrossPoint),
    MrFailed(MoveFailedType),
}

impl MoveResult {
    pub fn is_successful(&self) -> bool {
        match self {
            MrSuccessful(_) => return true,
            MrFailed(_) => return false,
        }
    }
}


pub struct RuleChecker {
    board: Rc<Board>,
    status: Cell<GameStatus>,
    score: Cell<i32>,
    tuples: RefCell<Vec<Tuple>>,
    tuple_indices: RefCell<HashMap<MoveDirection, usize>>,
}

impl BoardObserver for RuleChecker {
    fn board_updated(&self, event: BoardEvent) {
        let check_directions = vec![
            [MdLeft, MdRight], [MdUp, MdDown],
            [MdUpLeft, MdDownRight], [MdUpRight, MdDownLeft]
        ];

        for direction in check_directions.iter() {
            self.update_evaluation_by_event(direction, event);
        }
    }
}

impl RuleChecker {
    pub fn create_with_detail(board: Rc<Board>) -> Rc<RuleChecker> {
        let rule_checker = Rc::new(RuleChecker {
            board: board.clone(),
            status: Cell::new(GsGameContinue),
            score: Cell::new(0),
            tuples: RefCell::new(Vec::new()),
            tuple_indices: RefCell::new(HashMap::new()),
        });

        rule_checker.set_all_tuples();
        board.add_observers(Rc::downgrade(&rule_checker));
        return rule_checker;
    }

    pub fn check_game_status(&self) -> GameStatus {
        for tuple in self.tuples.borrow().iter() {

            if tuple.count(CptChess(CtBlack)) == 5 {
                self.status.set(GsGameOver(CtBlack));
                return self.status.get();
            } else if tuple.count(CptChess(CtWhite)) == 5 {
                self.status.set(GsGameOver(CtWhite));
                return self.status.get();
            }
        }

        return GsGameContinue;
    }

    pub fn game_status(&self) -> GameStatus {
        return self.status.get();
    }

    pub fn get_evaluation(&self) -> i32 {
        return self.score.get();
    }

    fn set_all_tuples(&self) {
        let board_cp_count = self.board.size();
        let board_tp_count = board_cp_count - 5;
        let mut tuple_indices_ref = self.tuple_indices.borrow_mut();

        tuple_indices_ref.insert(MdRight, self.tuples.borrow().len());
        self.set_tuples(MdRight, Coord{row: 0, col: 0},
                        Coord{row: board_cp_count, col: board_tp_count});

        tuple_indices_ref.insert(MdDown, self.tuples.borrow().len());
        self.set_tuples(MdDown, Coord{row: 0, col: 0},
                        Coord{row: board_tp_count, col: board_cp_count});

        tuple_indices_ref.insert(MdDownRight, self.tuples.borrow().len());
        self.set_tuples(MdDownRight, Coord{row: 0, col: 0},
                        Coord{row: board_tp_count, col: board_tp_count});

        tuple_indices_ref.insert(MdDownLeft, self.tuples.borrow().len());
        self.set_tuples(MdDownLeft, Coord{row: 0, col: 4},
                        Coord{row: board_tp_count, col: board_tp_count});
    }

    fn set_tuples(&self, md: MoveDirection, offset: Coord, count: Coord) {
        let end = offset + count;

        for row in offset.row..end.row {
            for col in offset.col..end.col {
                self.tuples.borrow_mut()
                    .push(Tuple::create_with_md(5, self.board.clone(), Coord{row, col}, md));
            }
        }
    }

    fn update_evaluation_by_event(&self, md: &[MoveDirection], event: BoardEvent) {
        /*
        let coord = event.get_coord();
        let chess = event.get_chess();
        let mut coord_md = [coord, coord];
        let mut first_chess = [None, None];
        let mut stop_flag = [false, false];
        let mut count = 0; let max_count = 6;
        let mut i = 1;

        while count < max_count {
            i = (i + 1) % 2;
            if stop_flag[i] != true {
                match self.move_to(CoordAndChess { coord: coord_md[i], chess }, md[i]) {
                    MrSuccessful(coord_and_cross_point) => {
                        coord_md[i] = coord_and_cross_point.coord;
                        count += 1;
                        if first_chess[i] == None {
                            match coord_and_cross_point.cross_point {
                                CptChess(_) => first_chess[i] = Some(chess),
                                CptEmpty => {},
                            }
                        }
                    },
                    MrFailed(move_failed_type) => match move_failed_type {
                        MftDifferenceChess => {
                            if first_chess[i] == None {
                                first_chess[i] = Some(chess.get_different_chess());
                            }
                            stop_flag[i] = true;
                        },
                        MftBoarder => stop_flag[i] = true,
                    }
                }
            }
        }
        */

        //println!("coords: {:?}", coord_md);
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

    fn update_tuple_evaluation(&self, coord_and_chess: CoordAndChess,
                               md: MoveDirection, event: BoardEvent) {
        //let mut line = VecDeque::new();
    }

    fn move_to(&self, coord_and_chess: CoordAndChess, md: MoveDirection) -> MoveResult {
        match self.board.move_to(coord_and_chess.coord, md) {
            Ok(coord) => match self.board.get_cross_point_type_at(coord) {
                CptEmpty => return MrSuccessful(CoordAndCrossPoint{coord, cross_point: CptEmpty}),
                CptChess(chess) => match chess == coord_and_chess.chess {
                    false => return MrFailed(MftDifferenceChess),
                    _ => return MrSuccessful(CoordAndCrossPoint{coord, cross_point: CptChess(chess)}),
                }
            }
            Err(error) => match error.kind {
                ErrorKind::CoordInvalid => return MrFailed(MftBoarder),
                _ => panic!("RuleChecker move failed with error {:?}", error.message),
            }
        }
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
