use std::rc::Rc;
use std::cell::{Cell, RefCell};

use backend::cross_point::*;
use backend::board::*;
use backend::rule_checker::*;

use backend::cross_point::ChessType::*;
use backend::rule_checker::GameStatus::*;

pub struct BoardController {
    chess: Cell<ChessType>,
    board: Rc<Board>,
    rule_checker: Rc<RuleChecker>,
    steps: RefCell<Vec<CoordAndChess>>,
    winner: Cell<Option<&'static str>>,
    value_changed: Cell<bool>,
}

impl BoardController {
    pub fn new() -> BoardController {
        let chess = Cell::new(ChessType::CtBlack);
        let board = Board::new();
        let rule_checker = RuleChecker::create_with_detail(board.clone());
        let steps = RefCell::new(Vec::new());
        let winner = Cell::new(None);
        let value_changed = Cell::new(false);

        return BoardController {
            chess,
            board,
            rule_checker,
            steps,
            winner,
            value_changed,
        }
    }

    pub fn get_current_chess(&self) -> String {
        return self.chess_type_to_str(self.chess.get()).to_string();
    }

    pub fn put_chess(&self, row: i32, col: i32) {
        let coord = Coord{row: row as usize, col: col as usize};
        self.board.put_chess_at(coord, self.chess.get());
        self.chess.set(self.chess.get().get_different_chess());
        self.steps.borrow_mut().push(CoordAndChess{coord, chess: self.chess.get()});
        self.value_changed.set(true);
    }

    pub fn remove_last_chess(&self) -> Option<(i32, i32)> {
        if let Some(last_step) = self.steps.borrow_mut().pop() {
            let coord = last_step.coord;
            self.board.remove_chess_at(coord);
            self.chess.set(self.chess.get().get_different_chess());
            self.value_changed.set(true);
            return Some((coord.row as i32, coord.col as i32));
        };

        return None;
    }

    pub fn have_game_over(&self) -> bool {
        self.update_game_status();
        return self.winner.get().is_some();
    }

    pub fn get_winner(&self) -> Option<String> {
        self.update_game_status();
        if let Some(winner_str) = self.winner.get() {
            return Some(winner_str.to_string());
        }

        return None;
    }

    pub fn restart_game(&self) {
        let mut len = self.steps.borrow().len();
        while len > 0 {
            self.remove_last_chess();
            len -= 1;
        }
    }

    fn update_game_status(&self) {
        if self.value_changed.get() {
            if let GsGameOver(winner) = self.rule_checker.check_game_status() {
                self.winner.set(Some(self.chess_type_to_str(winner)));
            } else {
                self.winner.set(None);
            }
            self.value_changed.set(false);
        }
    }

    fn chess_type_to_str(&self, chess: ChessType) -> &'static str {
        match chess {
            CtBlack => return "black",
            CtWhite => return "white",
        }
    }
}