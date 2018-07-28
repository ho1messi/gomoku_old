use std::rc::*;
use std::cell::*;

use super::cross_point::*;
use super::board::*;

pub struct Tuple {
    size: usize,
    coords: Vec<Coord>,
    cross_points: Vec<Weak<RefCell<CrossPoint>>>,
}

impl Tuple {
    pub fn create_with_md(size: usize, board: Rc<RefCell<Board>>, mut coord: Coord,
                      md: MoveDirection) -> Self {
        let mut coords = vec![coord];
        let mut cross_points = vec![Rc::downgrade(&board.borrow().get_cross_point_at(coord))];
        for _i in 1..size {
            coord = board.borrow().move_to(coord, md).unwrap();
            coords.push(coord);
            cross_points.push(Rc::downgrade(&board.borrow().get_cross_point_at(coord)));
        }

        return Tuple { size, coords, cross_points };
    }

    pub fn coord_at(&self, index: usize) -> Coord {
        return self.coords[index];
    }

    pub fn is_index_valid(&self, index: usize) -> bool {
        return index < self.size;
    }

    pub fn have_chess_at(&self, index: usize) -> bool {
        if !self.is_index_valid(index) {
            panic!("index out of range")
        }

        let board = self.get_cross_point_rc(index);
        return board.borrow().have_chess();
    }

    pub fn get_chess_at(&self, index: usize) -> ChessType {
        if !self.is_index_valid(index) {
            panic!("index out of range")
        }

        let board = self.get_cross_point_rc(index);
        return board.borrow().get_chess();
    }

    pub fn get_cross_point_type_at(&self, index: usize) -> CrossPointType {
        if !self.is_index_valid(index) {
            panic!("index out of range")
        }

        let board = self.get_cross_point_rc(index);
        return board.borrow().get_cross_point_type();
    }

    pub fn count(&self, cpt: CrossPointType) -> u32 {
        let mut num: u32 = 0;
        for i in 0..self.size {
            let board = self.get_cross_point_rc(i);
            if board.borrow().get_cross_point_type() == cpt {
                num += 1;
            }
        }

        return num;
    }

    pub fn have_include(&self, row: usize, col: usize) -> bool {
        let coord = Coord{ row, col };
        for i in 0..self.size {
            if self.coords[i] == coord {
                return true;
            }
        }

        return false;
    }

    fn get_cross_point_rc(&self, index: usize) -> Rc<RefCell<CrossPoint>> {
        match self.cross_points[index].upgrade() {
            Some(cross_point_rc) => return cross_point_rc,
            None => panic!("Can not upgrade weak cross point to rc!"),
        }
    }
}