use super::cross_point::*;
use super::board::*;

pub struct Tuple<'a> {
    size: usize,
    coords: Vec<Coord>,
    cross_points: Vec<&'a CrossPoint>,
}

impl<'a> Tuple<'a> {
    pub fn create_with_md(size: usize, board: &'a Board, row: usize,
                      col: usize, md: MoveDirection) -> Self {
        let mut coords = vec![Coord{ row, col}];
        let mut cross_points = vec![board.get_cross_point_at(row, col)];
        for _i in 1..size {
            let (row, col) = board.move_to(row, col, md).unwrap();
            coords.push(Coord{ row, col });
            cross_points.push(board.get_cross_point_at(row, col));
        }

        return Tuple { size, coords, cross_points };
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

    pub fn have_include(&self, row: usize, col: usize) -> bool {
        let coord = Coord{ row, col };
        for i in 0..self.size {
            if self.coords[i] == coord {
                return true;
            }
        }

        return false;
    }
}