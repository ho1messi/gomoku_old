use self::MoveDirection::*;
use super::cross_point::*;
use super::utils::*;

#[derive(Hash, Eq, PartialEq, Copy, Clone, Debug)]
pub enum MoveDirection {
    MdUp,
    MdDown,
    MdLeft,
    MdRight,
    MdUpLeft,
    MdUpRight,
    MdDownLeft,
    MdDownRight,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum BoardOperation {
    BoPutChess(ChessType),
    BoRemoveChess,
}

#[derive(PartialEq, Copy, Clone, Debug)]
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

pub trait BoardObserver {
    fn board_updated(&self, row: usize, col: usize, op: BoardOperation);
}

pub struct Board {
    size: usize,
    cp_count: usize,
    cross_points: Vec<CrossPoint>,
    observers: Vec<Box<BoardObserver>>,
}

impl Board {
    pub fn new() -> Self {
        let mut b = Board {
            size: 15,
            cp_count: 15 * 15,
            cross_points: Vec::new(),
            observers: Vec::new(),
        };

        b.cross_points.resize(b.cp_count, CrossPoint::new());
        return b;
    }

    pub fn create_with_size(size: usize) -> Self {
        let mut b = Board {
            size,
            cp_count: size * size,
            cross_points: Vec::new(),
            observers: Vec::new(),
        };

        b.cross_points.resize(b.cp_count, CrossPoint::new());
        return b;
    }

    pub fn size(&self) -> usize {
        return self.size;
    }

    pub fn cp_count(&self) -> usize {
        return self.cp_count;
    }

    pub fn is_index_valid(&self, row: usize, col: usize) -> bool {
        return row < self.size && col < self.size;
    }

    pub fn coord_to_index(&self, row: usize, col: usize) -> usize {
        return row * self.size + col;
    }

    pub fn have_chess_at(&self, row: usize, col: usize) -> bool {
        if !self.is_index_valid(row, col) {
            panic!("coord is not valid");
        }

        return self.cross_points[self.coord_to_index(row, col)].have_chess();
    }

    pub fn get_chess_at(&self, row: usize, col: usize) -> ChessType {
        if !self.is_index_valid(row, col) {
            panic!("coord is not valid");
        }

        return self.cross_points[self.coord_to_index(row, col)].get_chess();
    }

    pub fn put_chess_at(& mut self, row: usize, col: usize, chess: ChessType) {
        if !self.is_index_valid(row, col) {
            panic!("coord is not valid");
        }

        let index = self.coord_to_index(row, col);
        self.cross_points[index].put_chess(chess);

        self.notify_observers(row, col, BoardOperation::BoPutChess(chess));
    }

    pub fn remove_chess_at(& mut self, row: usize, col: usize) {
        if !self.is_index_valid(row, col) {
            panic!("coord is not valid");
        }

        let index = self.coord_to_index(row, col);
        self.cross_points[index].remove_chess();

        self.notify_observers(row, col, BoardOperation::BoRemoveChess);
    }

    pub fn get_cross_point_type_at(&self, row: usize, col: usize) -> CrossPointType {
        if !self.is_index_valid(row, col) {
            panic!("coord is not valid");
        }

        return self.cross_points[row * self.size + col].get_cross_point_type();
    }

    pub fn move_to(&self, row: usize, col: usize, md: MoveDirection)
        -> Result<(usize, usize), Error> {

        if !self.is_index_valid(row, col) {
            panic!("coord is not valid");
        }

        let (mut row_i, mut col_i) = (row as i32, col as i32);

        match md {
            MdUp => row_i = row_i - 1 as i32,
            MdDown => row_i = row_i + 1 as i32,
            MdLeft => col_i = col_i - 1 as i32,
            MdRight => col_i = col_i + 1 as i32,
            MdUpLeft => {row_i = row_i - 1 as i32; col_i = col_i - 1 as i32},
            MdUpRight => {row_i = row_i - 1 as i32; col_i = col_i + 1 as i32},
            MdDownLeft => {row_i = row_i + 1 as i32; col_i = col_i - 1 as i32},
            MdDownRight => {row_i = row_i + 1 as i32; col_i = col_i + 1 as i32},
        };

        if row_i < 0 || col_i < 0 {
            return Result::Err(Error::create_with_detail(ErrorKind::CoordInvalid,
                                                         "moved to out of bound"));
        }

        if self.is_index_valid(row_i as usize, col_i as usize) {
            return Result::Ok((row_i as usize, col_i as usize));
        } else {
            return Result::Err(Error::create_with_detail(ErrorKind::CoordInvalid,
                                                         "moved to out of bound"));
        }
    }

    pub fn get_cross_point_at(&self, row: usize, col: usize) -> &CrossPoint {
        return &self.cross_points[self.coord_to_index(row, col)];
    }

    pub fn add_observers<T>(&mut self, observer: T)
        where T: BoardObserver + 'static {
        self.observers.push(Box::new(observer));
    }

    pub fn notify_observers(&self, row: usize, col: usize, op: BoardOperation) {
        for observer in self.observers.iter() {
            observer.board_updated(row, col, op);
        }
    }
}
