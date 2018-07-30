use std::rc::Rc;
use std::cell::Cell;

use self::ChessType::*;
use self::CrossPointType::*;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum ChessType {
    CtBlack,
    CtWhite,
}

impl ChessType {
    pub fn get_different_chess(&self) -> ChessType {
        match *self {
            CtBlack => return CtWhite,
            CtWhite => return CtBlack,
        }
    }
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum CrossPointType {
    CptEmpty,
    CptChess(ChessType),
}

#[derive(PartialEq, Clone, Debug)]
pub struct CrossPoint {
    status: Cell<CrossPointType>,
}

impl CrossPoint {
    pub fn new() -> Rc<CrossPoint> {
        let cp = Rc::new(CrossPoint{status: Cell::new(CptEmpty)});
        return cp;
    }

    pub fn create_with_chess(chess: ChessType) -> Rc<CrossPoint> {
        let cp = Rc::new(CrossPoint{status: Cell::new(CptChess(chess))});
        return cp;
    }

    pub fn have_chess(&self) -> bool {
        return self.status.get() != CptEmpty;
    }

    pub fn get_chess(&self) -> ChessType {
        return match self.status.get() {
            CptChess(CtBlack) => CtBlack,
            CptChess(CtWhite) => CtWhite,
            CptEmpty => panic!("no chess in this cross point"),
        }
    }

    pub fn put_chess(&self, chess: ChessType) {
        match self.status.get() {
            CptEmpty => self.status.set(CptChess(chess)),
            _ => panic!("there already have a chess"),
        }
    }

    pub fn remove_chess(&self) {
        match self.status.get() {
            CptChess(_) => self.status.set(CptEmpty),
            _ => panic!("no chess to remove here"),
        }
    }

    pub fn get_cross_point_type(&self) -> CrossPointType {
        return self.status.get();
    }
}
