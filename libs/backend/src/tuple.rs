use std::rc::*;

use cross_point::*;
use board::*;

pub struct Tuple {
    size: usize,
    coords: Vec<Coord>,
    cross_points: Vec<Rc<CrossPoint>>,
}

impl Tuple {
    pub fn create_with_md(size: usize, board: Rc<Board>, mut coord: Coord,
                      md: MoveDirection) -> Self {
        let mut coords = vec![coord];
        let mut cross_points = vec![board.get_cross_point_at(coord)];
        for _i in 1..size {
            coord = board.move_by_coord(coord, md).unwrap();
            coords.push(coord);
            cross_points.push(board.get_cross_point_at(coord));
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

        return self.cross_points[index].have_chess();
    }

    pub fn get_chess_at(&self, index: usize) -> ChessType {
        if !self.is_index_valid(index) {
            panic!("index out of range")
        }

        return self.cross_points[index].get_chess();
    }

    pub fn get_cross_point_type_at(&self, index: usize) -> CrossPointType {
        if !self.is_index_valid(index) {
            panic!("index out of range")
        }

        return self.cross_points[index].get_cross_point_type();
    }

    pub fn count(&self, cpt: CrossPointType) -> u32 {
        let mut num: u32 = 0;
        for i in 0..self.size {
            if self.cross_points[i].get_cross_point_type() == cpt {
                num += 1;
            }
        }

        return num;
    }

    pub fn have_include(&self, coord: Coord) -> bool {
        for i in 0..self.size {
            if self.coords[i] == coord {
                return true;
            }
        }

        return false;
    }
}