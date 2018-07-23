use self::ChessType::*;
use self::CrossPointType::*;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum ChessType {
    CtBlack,
    CtWhite,
}

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum CrossPointType {
    CptEmpty,
    CptChess(ChessType),
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct CrossPoint {
    status: CrossPointType,
}

impl CrossPoint {
    pub fn new() -> Self {
        let cp = CrossPoint{status: CptEmpty};
        return cp;
    }

    pub fn create_with_chess(chess: ChessType) -> Self {
        let cp = CrossPoint{status: CptChess(chess)};
        return cp;
    }

    pub fn have_chess(&self) -> bool {
        return self.status != CptEmpty;
    }

    pub fn get_chess(&self) -> ChessType {
        return match self.status {
            CptChess(CtBlack) => CtBlack,
            CptChess(CtWhite) => CtWhite,
            CptEmpty => panic!("no chess in this cross point"),
        }
    }

    pub fn put_chess(& mut self, chess: ChessType) {
        match self.status {
            CptEmpty => self.status = CptChess(chess),
            _ => panic!("there already have a chess"),
        }
    }

    pub fn remove_chess(& mut self) {
        match self.status {
            CptChess(_) => self.status = CptEmpty,
            _ => panic!("no chess to remove here"),
        }
    }

    pub fn get_cross_point_type(&self) -> CrossPointType {
        return self.status;
    }
}
