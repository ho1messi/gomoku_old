use std::rc::*;
use std::cell::*;
use std::collections::HashMap;
use slice_deque::SliceDeque;

use evaluation_dfa::*;
use board::*;
use tuple::*;
use cross_point::*;
use utils::*;

use board::MoveDirection::*;
use cross_point::CrossPointType::*;
use cross_point::ChessType::*;

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

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
struct FirstVisitChess {
    first_chess: Option<ChessType>,
}

impl FirstVisitChess {
    pub fn new() -> FirstVisitChess {
        return FirstVisitChess{first_chess: None};
    }

    pub fn set_chess(&mut self, chess: ChessType) {
        match self.first_chess {
            Some(_) => return,
            None => self.first_chess = Some(chess),
        }
    }

    pub fn set_cross_point(&mut self, cp: CrossPointType) {
        match self.first_chess {
            Some(_) => return,
            None => match cp {
                CptChess(chess) => self.first_chess = Some(chess),
                CptEmpty => return,
            }
        }
    }

    pub fn get_first_chess(&self) -> Option<ChessType> {
        return self.first_chess;
    }
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
    evaluation_dfa: EvaluationDfa,
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
            evaluation_dfa: EvaluationDfa::new(),
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

    pub fn get_simple_play(&self) -> Coord {
        let mut tuple_score = Vec::new();
        let mut cp_score = Vec::new();
        tuple_score.resize(self.tuples.borrow().len(), 0);

        for row in 0..self.board.size() {
            for col in 0..self.board.size() {
                let index = row * self.board.size() + col;
                cp_score.push((0, Coord{row, col}));
            }
        }

        for i in 0..self.tuples.borrow().len() {
            let tuple = &self.tuples.borrow()[i];
            tuple_score[i] += self.get_tuple_score(i);
            for row in 0..self.board.size() {
                for col in 0..self.board.size() {
                    if tuple.have_include(Coord{row, col}) {
                        let index = row * self.board.size() + col;
                        cp_score[index].0 += tuple_score[i];
                    }

                }
            }
        }

        let mut max_score = 0; let mut max_index = 0;
        for index in 0..cp_score.len() {
            if !self.board.have_chess_at(cp_score[index].1) {
                if cp_score[index].0 > max_score {
                    max_score = cp_score[index].0;
                    max_index = index;
                }
            }
        }

        return cp_score[max_index].1;
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

    // move by cross point linked list
    // create a struct include SliceDeque and calculate index when push front
    fn update_evaluation_by_event(&self, md: &[MoveDirection], event: BoardEvent) {
        let coord = event.get_coord(); let chess = event.get_chess();
        let mut coord_md = [coord, coord];
        let mut first_chess = [FirstVisitChess::new(), FirstVisitChess::new()];
        let mut continue_flag = [true, true];
        let mut count = 1; let mut i = 1; let max_count = 7; let mut index = 0;
        let mut cpts = SliceDeque::with_capacity(max_count); cpts.push_back(CptChess(chess));

        while count < max_count && (continue_flag[0] || continue_flag[1]){
            i = (i + 1) % 2;
            if continue_flag[i] == true {
                count += 1;
                match self.move_to(CoordAndChess { coord: coord_md[i], chess }, md[i]) {
                    MrSuccessful(coord_and_cross_point) => {
                        first_chess[i].set_cross_point(coord_and_cross_point.cross_point);
                        coord_md[i] = coord_and_cross_point.coord;
                        match i == 0 {
                            true => {
                                cpts.push_front(coord_and_cross_point.cross_point);
                                index += 1;
                            },
                            false => cpts.push_back(coord_and_cross_point.cross_point),
                        }
                    },
                    MrFailed(move_failed_type) => match move_failed_type {
                        MftDifferenceChess => {
                            first_chess[i].set_chess(chess.get_different_chess());
                            continue_flag[i] = false;
                            match i == 0 {
                                true => {
                                    cpts.push_front(CptChess(chess.get_different_chess()));
                                    index += 1;
                                },
                                false => cpts.push_back(CptChess(chess.get_different_chess())),
                            }
                        },
                        MftBoarder => {
                            continue_flag[i] = false;
                            match i == 0 {
                                true => {
                                    cpts.push_front(CptChess(chess.get_different_chess()));
                                    index += 1;
                                },
                                false => cpts.push_back(CptChess(chess.get_different_chess())),
                            }
                        },
                    }
                }
            }
        }

        /*
        println!("mds: {:?}", md);
        println!("coords: {:?}", coord_md);
        println!("first_chess: {:?}", first_chess);
        println!("=================================");
        for cpt in cpts.iter() {
            println!("{:?}", cpt);
        }
        println!("=================================");
        println!("chess is {:?}", chess);
        */

        let score = self.score.get();
        self.score.set(score + self.evaluation_dfa
            .evaluate_event(cpts.as_mut_slice(), index, event));
        for i in 0..2 {
            match first_chess[i].get_first_chess() {
                Some(chess_f) => match chess_f == chess {
                    true => {},
                    false => self.update_tuple_evaluation(
                        CoordAndChess{coord, chess: chess_f},
                        md[i], event
                    ),
                },
                None => {},
            }
        }
    }

    fn update_tuple_evaluation(&self, coord_and_chess: CoordAndChess,
                               md: MoveDirection, event: BoardEvent) {
        let coord = coord_and_chess.coord; let chess = coord_and_chess.chess;
        let mut coord_md = coord;
        let mut count = 1; let max_count = 7;
        let mut cpts = SliceDeque::with_capacity(max_count);
        cpts.push_back(CptChess(chess.get_different_chess()));

        while count < max_count {
            count += 1;
            match self.move_to(CoordAndChess { coord: coord_md, chess }, md) {
                MrSuccessful(coord_and_cross_point) => {
                    coord_md = coord_and_cross_point.coord;
                    cpts.push_back(coord_and_cross_point.cross_point);
                },
                MrFailed(_) => {
                    cpts.push_back(CptChess(chess.get_different_chess()));
                    break;
                },
            }
        }

        /*
        println!("=================================");
        for cpt in cpts.iter() {
            println!("{:?}", cpt);
        }
        println!("=================================");
        println!("chess is {:?}", chess.get_different_chess());
        */

        let score = self.score.get();
        self.score.set(score + self.evaluation_dfa.evaluate_event(cpts.as_mut_slice(), 0, event));
    }

    fn move_to(&self, coord_and_chess: CoordAndChess, md: MoveDirection) -> MoveResult {
        match self.board.move_by_coord(coord_and_chess.coord, md) {
            Ok(coord) => match self.board.get_cross_point_type_at(coord) {
                CptEmpty => return MrSuccessful(CoordAndCrossPoint{coord, cross_point: CptEmpty}),
                CptChess(chess) => match chess == coord_and_chess.chess {
                    false => return MrFailed(MftDifferenceChess),
                    _ => return MrSuccessful(CoordAndCrossPoint{coord, cross_point: CptChess(chess)}),
                }
            }
            Err(error) => match error.kind {
                ErrorKind::CoordInvalid => return MrFailed(MftBoarder),
                //_ => panic!("RuleChecker move failed with error {:?}", error.message),
            }
        }
    }

    fn get_tuple_score(&self, index: usize) -> i32 {
        let white_score_list = vec![0, 35, 800, 15000, 800000, 0];
        let black_score_list = vec![0, 15, 400, 1800, 100000, 0];
        let none_score = 7;
        let both_score = 0;

        let tuple = &self.tuples.borrow()[index];
        let black_count = tuple.count(CptChess(CtBlack));
        let white_count = tuple.count(CptChess(CtWhite));

        if black_count > 0 {
            if white_count > 0 {
                return both_score;
            } else {
                return black_score_list[black_count as usize];
            }
        } else if white_count > 0 {
            return white_score_list[white_count as usize];
        } else {
            return none_score;
        }
    }
}
