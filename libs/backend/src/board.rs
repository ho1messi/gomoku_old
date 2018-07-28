use std::rc::*;
use std::cell::*;
use std::ops::Add;

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
pub struct Coord {
    pub row: usize,
    pub col: usize,
}

impl Add for Coord {
    type Output = Coord;
    fn add(self, other: Coord) -> Self::Output {
        return Coord{
            row: self.row + other.row,
            col: self.col + other.col
        }
    }
}

impl Coord {
    pub fn from_i32s(row_i: i32, col_i: i32) -> Self {
        return Coord{row: row_i as usize, col: col_i as usize};
    }

    pub fn as_i32s(&self) -> (i32, i32) {
        return (self.row as i32, self.col as i32);
    }
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct CoordAndChess {
    pub coord: Coord,
    pub chess: ChessType,
}

#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum BoardEvent {
    BoPutChess(CoordAndChess),
    BoRemoveChess(CoordAndChess),
}

impl BoardEvent {
    pub fn get_coord_and_chess(&self) -> CoordAndChess {
        match self {
            BoardEvent::BoPutChess(coord_and_chess) => return *coord_and_chess,
            BoardEvent::BoRemoveChess(coord_and_chess) => return *coord_and_chess,
        }
    }

    pub fn get_coord(&self) -> Coord {
        match self {
            BoardEvent::BoPutChess(coord_and_chess) => return coord_and_chess.coord,
            BoardEvent::BoRemoveChess(coord_and_chess) => return coord_and_chess.coord,
        }
    }

    pub fn get_chess(&self) -> ChessType {
        match self {
            BoardEvent::BoPutChess(coord_and_chess) => return coord_and_chess.chess,
            BoardEvent::BoRemoveChess(coord_and_chess) => return coord_and_chess.chess,
        }
    }
}


pub trait BoardObserver {
    fn board_updated(&self, event: BoardEvent);
}


pub struct Board {
    size: usize,
    cp_count: usize,
    cross_points: Vec<Rc<RefCell<CrossPoint>>>,
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

        for i in 0..b.cp_count {
            b.cross_points.push(Rc::new(RefCell::new(CrossPoint::new())));
        }
        return b;
    }

    pub fn create_with_size(size: usize) -> Self {
        let mut b = Board {
            size,
            cp_count: size * size,
            cross_points: Vec::new(),
            observers: Vec::new(),
        };

        for i in 0..b.cp_count {
            b.cross_points.push(Rc::new(RefCell::new(CrossPoint::new())));
        }
        return b;
    }

    pub fn size(&self) -> usize {
        return self.size;
    }

    pub fn cp_count(&self) -> usize {
        return self.cp_count;
    }

    pub fn is_index_valid(&self, coord: Coord) -> bool {
        return coord.row < self.size && coord.col < self.size;
    }

    pub fn coord_to_index(&self, coord: Coord) -> usize {
        return coord.row * self.size + coord.col;
    }

    pub fn have_chess_at(&self, coord: Coord) -> bool {
        if !self.is_index_valid(coord) {
            panic!("coord is not valid");
        }

        println!("{:?}", coord);
        println!("{}", self.coord_to_index(coord));
        let cps = self.cross_points[self.coord_to_index(coord)].clone();
        println!("{:?}", self.cross_points[1].borrow().get_cross_point_type());
        println!("{:?}", cps);

        return self.cross_points[self.coord_to_index(coord)].borrow().have_chess();
    }

    pub fn get_chess_at(&self, coord: Coord) -> ChessType {
        if !self.is_index_valid(coord) {
            panic!("coord is not valid");
        }

        return self.cross_points[self.coord_to_index(coord)].borrow().get_chess();
    }

    pub fn put_chess_at(& mut self, coord: Coord, chess: ChessType) {
        if !self.is_index_valid(coord) {
            panic!("coord is not valid");
        }

        let index = self.coord_to_index(coord);
        self.cross_points[index].borrow_mut().put_chess(chess);

        for i in 0..self.size {
            println!("{}\t{:?}", i, self.cross_points[i]);
        }

        self.notify_observers(BoardEvent::BoPutChess(CoordAndChess {coord, chess}));
    }

    pub fn remove_chess_at(& mut self, coord: Coord) -> ChessType {
        if !self.is_index_valid(coord) {
            panic!("coord is not valid");
        }

        let index = self.coord_to_index(coord);
        let chess = self.cross_points[index].borrow().get_chess();
        self.cross_points[index].borrow_mut().remove_chess();

        for i in 0..self.size {
            println!("{}\t{:?}", i, self.cross_points[i]);
        }

        self.notify_observers(BoardEvent::BoRemoveChess(CoordAndChess {coord, chess}));
        return chess;
    }

    pub fn get_cross_point_type_at(&self, coord: Coord) -> CrossPointType {
        if !self.is_index_valid(coord) {
            panic!("coord is not valid");
        }

        return self.cross_points[coord.row * self.size + coord.col].borrow().get_cross_point_type();
    }

    pub fn move_to(&self, coord: Coord, md: MoveDirection)
        -> Result<Coord, Error> {
        if !self.is_index_valid(coord) {
            panic!("coord is not valid");
        }

        let (mut row_i, mut col_i) = coord.as_i32s();
        match md {
            MdUp => row_i = row_i - 1,
            MdDown => row_i = row_i + 1,
            MdLeft => col_i = col_i - 1,
            MdRight => col_i = col_i + 1,
            MdUpLeft => {row_i = row_i - 1; col_i = col_i - 1},
            MdUpRight => {row_i = row_i - 1; col_i = col_i + 1},
            MdDownLeft => {row_i = row_i + 1; col_i = col_i - 1},
            MdDownRight => {row_i = row_i + 1; col_i = col_i + 1},
        };

        if row_i < 0 || col_i < 0 {
            return Result::Err(
                Error::create_with_detail(ErrorKind::CoordInvalid, "moved to out of bound")
            );
        }
        let coord_return = Coord::from_i32s(row_i, col_i);
        if self.is_index_valid(coord_return) {
            return Result::Ok(coord_return);
        } else {
            return Result::Err(Error::create_with_detail(ErrorKind::CoordInvalid,
                                                         "moved to out of bound"));
        }
    }

    pub fn get_cross_point_at(&self, coord: Coord) -> Rc<RefCell<CrossPoint>> {
        return Rc::clone(&self.cross_points[self.coord_to_index(coord)]);
    }

    pub fn add_observers<T>(&mut self, observer: T)
        where T: BoardObserver + 'static {
        self.observers.push(Box::new(observer));
    }

    pub fn notify_observers(&self, event: BoardEvent) {
        for observer in self.observers.iter() {
            observer.board_updated(event);
        }
    }
}
