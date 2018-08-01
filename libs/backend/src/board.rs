use std::rc::*;
use std::cell::*;
use std::ops::Add;
use std::collections::HashMap;

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
pub struct CoordAndCrossPoint {
    pub coord: Coord,
    pub cross_point: CrossPointType,
}

#[derive(Clone, Debug)]
pub struct CrossPointLinkedList {
    cross_point: Rc<CrossPoint>,
    coord: Coord,
    link_map: RefCell<HashMap<MoveDirection, Weak<CrossPointLinkedList>>>,
}

impl CrossPointLinkedList {
    pub fn create(cross_point: Rc<CrossPoint>, coord: Coord) -> Rc<CrossPointLinkedList> {
        return Rc::new(CrossPointLinkedList {
            cross_point,
            coord,
            link_map: RefCell::new(HashMap::new()),
        });
    }

    pub fn insert_link (&self, md: MoveDirection, destination_weak: Weak<CrossPointLinkedList>) {
        self.link_map.borrow_mut().insert(md, destination_weak);
    }

    pub fn get_next(&self, md: MoveDirection) -> Option<Rc<CrossPointLinkedList>> {
        match self.link_map.borrow().get(&md) {
            Some(cp_weak) => return cp_weak.upgrade(),
            None => return None,
        }
    }

    pub fn get_cross_point(&self) -> Rc<CrossPoint> {
        return self.cross_point.clone();
    }

    pub fn get_coord(&self) -> Coord {
        return self.coord;
    }
}


#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub enum BoardEvent {
    BePutChess(CoordAndChess),
    BeRemoveChess(CoordAndChess),
}

impl BoardEvent {
    pub fn get_coord_and_chess(&self) -> CoordAndChess {
        match self {
            BoardEvent::BePutChess(coord_and_chess) => return *coord_and_chess,
            BoardEvent::BeRemoveChess(coord_and_chess) => return *coord_and_chess,
        }
    }

    pub fn get_coord(&self) -> Coord {
        match self {
            BoardEvent::BePutChess(coord_and_chess) => return coord_and_chess.coord,
            BoardEvent::BeRemoveChess(coord_and_chess) => return coord_and_chess.coord,
        }
    }

    pub fn get_chess(&self) -> ChessType {
        match self {
            BoardEvent::BePutChess(coord_and_chess) => return coord_and_chess.chess,
            BoardEvent::BeRemoveChess(coord_and_chess) => return coord_and_chess.chess,
        }
    }
}


pub trait BoardObserver {
    fn board_updated(&self, event: BoardEvent);
}


pub struct Board {
    size: usize,
    cp_count: usize,
    cross_points: Vec<Rc<CrossPoint>>,
    cross_point_linked_lists: Vec<Rc<CrossPointLinkedList>>,
    observers: RefCell<Vec<Weak<BoardObserver>>>,
}

impl Board {
    pub fn new() -> Rc<Board> {
        let mut b = Board {
            size: 15,
            cp_count: 15 * 15,
            cross_points: Vec::new(),
            cross_point_linked_lists: Vec::new(),
            observers: RefCell::new(Vec::new()),
        };

        b.initial_cross_points();
        return Rc::new(b);
    }

    pub fn create_with_size(size: usize) -> Rc<Board> {
        let mut b = Board {
            size,
            cp_count: size * size,
            cross_points: Vec::new(),
            cross_point_linked_lists: Vec::new(),
            observers: RefCell::new(Vec::new()),
        };

        b.initial_cross_points();
        return Rc::new(b);
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

        return self.cross_points[self.coord_to_index(coord)].have_chess();
    }

    pub fn get_chess_at(&self, coord: Coord) -> ChessType {
        if !self.is_index_valid(coord) {
            panic!("coord is not valid");
        }

        return self.cross_points[self.coord_to_index(coord)].get_chess();
    }

    pub fn put_chess_at(&self, coord: Coord, chess: ChessType) {
        if !self.is_index_valid(coord) {
            panic!("coord is not valid");
        }

        let index = self.coord_to_index(coord);
        self.cross_points[index].put_chess(chess);

        self.notify_observers(BoardEvent::BePutChess(CoordAndChess {coord, chess}));
    }

    pub fn remove_chess_at(&self, coord: Coord) -> ChessType {
        if !self.is_index_valid(coord) {
            panic!("coord is not valid");
        }

        let index = self.coord_to_index(coord);
        let chess = self.cross_points[index].get_chess();
        self.cross_points[index].remove_chess();

        self.notify_observers(BoardEvent::BeRemoveChess(CoordAndChess {coord, chess}));
        return chess;
    }

    pub fn get_cross_point_type_at(&self, coord: Coord) -> CrossPointType {
        if !self.is_index_valid(coord) {
            panic!("coord is not valid");
        }

        return self.cross_points[self.coord_to_index(coord)].get_cross_point_type();
    }

    pub fn move_by_coord(&self, coord: Coord, md: MoveDirection) -> Result<Coord, Error> {
        if !self.is_index_valid(coord) {
            panic!("coord is not valid");
        }

        let cross_point_l = &self.cross_point_linked_lists[self.coord_to_index(coord)];
        match cross_point_l.get_next(md) {
            Some(cross_point_l_r) => return Ok(cross_point_l_r.get_coord()),
            None => return Err(Error::create_with_detail(
                ErrorKind::CoordInvalid, "moved to out of bound"
            )),
        }
    }

    pub fn move_by_cross_point_l(&self, coord: Coord, md: MoveDirection)
        -> Result<Rc<CrossPointLinkedList>, Error> {
        if !self.is_index_valid(coord) {
            panic!("coord is not valid");
        }

        let cross_point_l = &self.cross_point_linked_lists[self.coord_to_index(coord)];
        match cross_point_l.get_next(md) {
            Some(cross_point_l_r) => return Ok(cross_point_l_r),
            None => return Err(Error::create_with_detail(
                ErrorKind::CoordInvalid, "moved to out of bound"
            )),
        }
    }

    pub fn get_cross_point_at(&self, coord: Coord) -> Rc<CrossPoint> {
        return self.cross_points[self.coord_to_index(coord)].clone();
    }

    pub fn get_cross_point_l_at(&self, coord: Coord) -> Rc<CrossPointLinkedList> {
        return self.cross_point_linked_lists[self.coord_to_index(coord)].clone();
    }

    pub fn add_observers<T>(&self, observer: Weak<T>)
        where T: BoardObserver + 'static {
        self.observers.borrow_mut().push(observer);
    }

    pub fn remove_observers<T>(&self, observer: Rc<T>)
        where T: BoardObserver + 'static {
        let len = self.observers.borrow().len();
        for i in 0..len {
            let result = self.observers.borrow()[i].upgrade();
            match result {
                Some(observer_rc) => {
                    match Rc::into_raw(observer_rc.clone()) == Rc::into_raw(observer.clone()) {
                        true => self.observers.borrow_mut().remove(i),
                        false => continue,
                    }
                },
                None => self.observers.borrow_mut().remove(i),
            };
        }
    }

    pub fn notify_observers(&self, event: BoardEvent) {
        for observer in self.observers.borrow().iter() {
            match observer.upgrade() {
                Some(observer_rc) => observer_rc.board_updated(event),
                None => continue,
            }
        }
    }

    fn initial_cross_points(&mut self) {
        for row in 0..self.size {
            for col in 0..self.size {
                let cross_point = CrossPoint::new();
                self.cross_points.push(cross_point.clone());
                self.cross_point_linked_lists.push(
                    CrossPointLinkedList::create(cross_point, Coord{row, col})
                );
            }
        }

        let mds = vec![MdUp, MdDown, MdLeft, MdRight, MdUpLeft, MdUpRight, MdDownLeft, MdDownRight];
        for row in 0..self.size {
            for col in 0..self.size {
                let coord = Coord{row, col};
                let index = self.coord_to_index(coord);
                for md in mds.iter() {
                    match self.move_to(coord, *md) {
                        Ok(coord_t) => {
                            let index_t = self.coord_to_index(coord_t);
                            self.cross_point_linked_lists[index].insert_link(
                                *md, Rc::downgrade(&self.cross_point_linked_lists[index_t])
                            );
                        }
                        Err(_) => {},
                    }
                }
            }
        }
    }

    fn move_to(&self, coord: Coord, md: MoveDirection)
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
}
