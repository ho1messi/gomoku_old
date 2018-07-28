use std::rc::*;
use std::cell::*;

use super::super::cross_point::*;
use super::super::board::*;
use super::super::tuple::*;

use super::super::cross_point::ChessType::*;
use super::super::cross_point::CrossPointType::*;
use super::super::board::MoveDirection::*;

#[test]
fn it_works() {
    let mut board = Rc::new(RefCell::new(Board::create_with_size(5)));
    for row in 0..5 {                                                            // X O _ X O
    for col in 0..5 {                                                            // _ X O _ X
            match (row * 5 + col) % 3 {                                          // O _ X O _
                0 => board.borrow_mut().put_chess_at(Coord{row, col}, ChessType::CtBlack),    // X O _ X O
                1 => board.borrow_mut().put_chess_at(Coord{row, col}, ChessType::CtWhite),    // _ X O _ X
                _ => {},
            }
        }
    }

    let tuple1 = Tuple::create_with_md(5, board.clone(), Coord{row: 0, col: 0}, MdRight);
    let tuple2 = Tuple::create_with_md(5, board.clone(), Coord{row: 0, col: 0}, MdDown);
    let tuple3 = Tuple::create_with_md(5, board.clone(), Coord{row: 0, col: 0}, MdDownRight);
    let tuple4 = Tuple::create_with_md(5, board.clone(), Coord{row: 0, col: 4}, MdDownLeft);

    let tuple_r1 = Tuple::create_with_md(5, board.clone(), Coord{row: 4, col: 4}, MdLeft);
    let tuple_r2 = Tuple::create_with_md(5, board.clone(), Coord{row: 4, col: 4}, MdUp);
    let tuple_r3 = Tuple::create_with_md(5, board.clone(), Coord{row: 4, col: 4}, MdUpLeft);
    let tuple_r4 = Tuple::create_with_md(5, board.clone(), Coord{row: 4, col: 0}, MdUpRight);

    assert_eq!(tuple1.get_cross_point_type_at(0), CptChess(CtBlack));
    assert_eq!(tuple1.get_cross_point_type_at(2), CptEmpty);
    assert_eq!(tuple1.get_cross_point_type_at(1), CptChess(CtWhite));
    assert_eq!(tuple1.get_cross_point_type_at(3), CptChess(CtBlack));
    assert_eq!(tuple1.get_cross_point_type_at(4), CptChess(CtWhite));

    assert_eq!(tuple2.get_cross_point_type_at(0), CptChess(CtBlack));
    assert_eq!(tuple2.get_cross_point_type_at(1), CptEmpty);
    assert_eq!(tuple2.get_cross_point_type_at(2), CptChess(CtWhite));
    assert_eq!(tuple2.get_cross_point_type_at(3), CptChess(CtBlack));
    assert_eq!(tuple2.get_cross_point_type_at(4), CptEmpty);

    assert_eq!(tuple3.get_cross_point_type_at(0), CptChess(CtBlack));
    assert_eq!(tuple3.get_cross_point_type_at(1), CptChess(CtBlack));
    assert_eq!(tuple3.get_cross_point_type_at(2), CptChess(CtBlack));
    assert_eq!(tuple3.get_cross_point_type_at(3), CptChess(CtBlack));
    assert_eq!(tuple3.get_cross_point_type_at(4), CptChess(CtBlack));

    assert_eq!(tuple4.get_cross_point_type_at(0), CptChess(CtWhite));
    assert_eq!(tuple4.get_cross_point_type_at(1), CptEmpty);
    assert_eq!(tuple4.get_cross_point_type_at(2), CptChess(CtBlack));
    assert_eq!(tuple4.get_cross_point_type_at(3), CptChess(CtWhite));
    assert_eq!(tuple4.get_cross_point_type_at(4), CptEmpty);

    assert_eq!(tuple_r1.get_cross_point_type_at(0), CptChess(CtBlack));
    assert_eq!(tuple_r1.get_cross_point_type_at(1), CptEmpty);
    assert_eq!(tuple_r1.get_cross_point_type_at(2), CptChess(CtWhite));
    assert_eq!(tuple_r1.get_cross_point_type_at(3), CptChess(CtBlack));
    assert_eq!(tuple_r1.get_cross_point_type_at(4), CptEmpty);

    assert_eq!(tuple_r2.get_cross_point_type_at(0), CptChess(CtBlack));
    assert_eq!(tuple_r2.get_cross_point_type_at(1), CptChess(CtWhite));
    assert_eq!(tuple_r2.get_cross_point_type_at(2), CptEmpty);
    assert_eq!(tuple_r2.get_cross_point_type_at(3), CptChess(CtBlack));
    assert_eq!(tuple_r2.get_cross_point_type_at(4), CptChess(CtWhite));

    assert_eq!(tuple_r3.get_cross_point_type_at(0), CptChess(CtBlack));
    assert_eq!(tuple_r3.get_cross_point_type_at(1), CptChess(CtBlack));
    assert_eq!(tuple_r3.get_cross_point_type_at(2), CptChess(CtBlack));
    assert_eq!(tuple_r3.get_cross_point_type_at(3), CptChess(CtBlack));
    assert_eq!(tuple_r3.get_cross_point_type_at(4), CptChess(CtBlack));

    assert_eq!(tuple_r4.get_cross_point_type_at(0), CptEmpty);
    assert_eq!(tuple_r4.get_cross_point_type_at(1), CptChess(CtWhite));
    assert_eq!(tuple_r4.get_cross_point_type_at(2), CptChess(CtBlack));
    assert_eq!(tuple_r4.get_cross_point_type_at(3), CptEmpty);
    assert_eq!(tuple_r4.get_cross_point_type_at(4), CptChess(CtWhite));

    assert_eq!(tuple1.count(CptChess(CtBlack)), 2);
    assert_eq!(tuple1.count(CptChess(CtWhite)), 2);
    assert_eq!(tuple1.count(CptEmpty), 1);
    assert_eq!(tuple2.count(CptChess(CtBlack)), 2);
    assert_eq!(tuple2.count(CptChess(CtWhite)), 1);
    assert_eq!(tuple2.count(CptEmpty), 2);
    assert_eq!(tuple3.count(CptChess(CtBlack)), 5);
    assert_eq!(tuple3.count(CptChess(CtWhite)), 0);
    assert_eq!(tuple3.count(CptEmpty), 0);
    assert_eq!(tuple4.count(CptChess(CtBlack)), 1);
    assert_eq!(tuple4.count(CptChess(CtWhite)), 2);
    assert_eq!(tuple4.count(CptEmpty), 2);
    assert_eq!(tuple_r1.count(CptChess(CtBlack)), 2);
    assert_eq!(tuple_r1.count(CptChess(CtWhite)), 1);
    assert_eq!(tuple_r1.count(CptEmpty), 2);
    assert_eq!(tuple_r2.count(CptChess(CtBlack)), 2);
    assert_eq!(tuple_r2.count(CptChess(CtWhite)), 2);
    assert_eq!(tuple_r2.count(CptEmpty), 1);
    assert_eq!(tuple_r3.count(CptChess(CtBlack)), 5);
    assert_eq!(tuple_r3.count(CptChess(CtWhite)), 0);
    assert_eq!(tuple_r3.count(CptEmpty), 0);
    assert_eq!(tuple_r4.count(CptChess(CtBlack)), 1);
    assert_eq!(tuple_r4.count(CptChess(CtWhite)), 2);
    assert_eq!(tuple_r4.count(CptEmpty), 2);

    for row in 0..5 {
        for col in 0..5 {
            if row == 0 {
                assert_eq!(tuple1.have_include(row, col), true);
            } else {
                assert_eq!(tuple1.have_include(row, col), false);
            }
            if col == 0 {
                assert_eq!(tuple2.have_include(row, col), true);
            } else {
                assert_eq!(tuple2.have_include(row, col), false);
            }
            if row == col {
                assert_eq!(tuple3.have_include(row, col), true);
            } else {
                assert_eq!(tuple3.have_include(row, col), false);
            }
            if row + col == 4 {
                assert_eq!(tuple4.have_include(row, col), true);
            } else {
                assert_eq!(tuple4.have_include(row, col), false);
            }
            if row == 4 {
                assert_eq!(tuple_r1.have_include(row, col), true);
            } else {
                assert_eq!(tuple_r1.have_include(row, col), false);
            }
            if col == 4 {
                assert_eq!(tuple_r2.have_include(row, col), true);
            } else {
                assert_eq!(tuple_r2.have_include(row, col), false);
            }
            if row == col {
                assert_eq!(tuple_r3.have_include(row, col), true);
            } else {
                assert_eq!(tuple_r3.have_include(row, col), false);
            }
            if row + col == 4 {
                assert_eq!(tuple_r4.have_include(row, col), true);
            } else {
                assert_eq!(tuple_r4.have_include(row, col), false);
            }
        }
    }
}
