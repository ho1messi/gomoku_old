use super::super::board::*;
use super::super::board::MoveDirection::*;
use super::super::cross_point::*;

#[test]
fn size() {
    let b = Board::new();
    assert_eq!(b.size(), 15);
    assert_eq!(b.cp_count(), 225);

    for i in 0..100 {
        let b = Board::create_with_size(i);
        assert_eq!(b.size(), i);
        assert_eq!(b.cp_count(), i * i);
    }
}

#[test]
fn index_valid() {
    let b = Board::new();

    for i in 0..20 {
        for j in 0..20 {
            if i < 15 && j < 15 {
                assert_eq!(b.is_index_valid(Coord{row: i, col: j}), true);
            } else {
                assert_eq!(b.is_index_valid(Coord{row: i, col: j}), false);
            }
        }
    }
}

#[test]
fn chess() {
    //let mut b = Board::new();
    let mut b = Board::create_with_size(3);

    for i in 0..3 {
        for j in 0..3 {
            assert_eq!(b.have_chess_at(Coord{row: i, col: j}), false);
            b.put_chess_at(Coord{row: i, col: j}, ChessType::CtBlack);
            assert_eq!(b.have_chess_at(Coord{row: i, col: j}), true);
            assert_eq!(b.get_chess_at(Coord{row: i, col: j}), ChessType::CtBlack);
            b.remove_chess_at(Coord{row: i, col: j});
            assert_eq!(b.have_chess_at(Coord{row: i, col: j}), false);
            b.put_chess_at(Coord{row: i, col: j}, ChessType::CtWhite);
            assert_eq!(b.have_chess_at(Coord{row: i, col: j}), true);
            assert_eq!(b.get_chess_at(Coord{row: i, col: j}), ChessType::CtWhite);
        }
    }
}

#[test]
fn move_to() {
    let b = Board::new();

    for i in 1..14 {
        for j in 1..14 {
            assert_eq!(b.move_to(Coord{row: i, col: j}, MdUp),
                       Ok(Coord{row: i - 1, col: j}));
            assert_eq!(b.move_to(Coord{row: i, col: j}, MdDown),
                       Ok(Coord{row: i + 1, col: j}));
            assert_eq!(b.move_to(Coord{row: i, col: j}, MdLeft),
                       Ok(Coord{row: i, col: j - 1}));
            assert_eq!(b.move_to(Coord{row: i, col: j}, MdRight),
                       Ok(Coord{row: i, col: j + 1}));
            assert_eq!(b.move_to(Coord{row: i, col: j}, MdUpLeft),
                       Ok(Coord{row: i - 1, col: j - 1}));
            assert_eq!(b.move_to(Coord{row: i, col: j}, MdUpRight),
                       Ok(Coord{row: i - 1, col: j + 1}));
            assert_eq!(b.move_to(Coord{row: i, col: j}, MdDownLeft),
                       Ok(Coord{row: i + 1, col: j - 1}));
            assert_eq!(b.move_to(Coord{row: i, col: j}, MdDownRight),
                       Ok(Coord{row: i + 1, col: j + 1}));
        }
    }

    for i in 0..15 {
        assert_eq!(b.move_to(Coord{row: 0, col: i}, MdUp).is_err(), true);
        assert_eq!(b.move_to(Coord{row: 0, col: i}, MdUpLeft).is_err(), true);
        assert_eq!(b.move_to(Coord{row: 0, col: i}, MdUpRight).is_err(), true);
        assert_eq!(b.move_to(Coord{row: 14, col: i}, MdDown).is_err(), true);
        assert_eq!(b.move_to(Coord{row: 14, col: i}, MdDownLeft).is_err(), true);
        assert_eq!(b.move_to(Coord{row: 14, col: i}, MdDownRight).is_err(), true);
        assert_eq!(b.move_to(Coord{row: i, col: 0}, MdLeft).is_err(), true);
        assert_eq!(b.move_to(Coord{row: i, col: 0}, MdUpLeft).is_err(), true);
        assert_eq!(b.move_to(Coord{row: i, col: 0}, MdDownLeft).is_err(), true);
        assert_eq!(b.move_to(Coord{row: i, col: 14}, MdRight).is_err(), true);
        assert_eq!(b.move_to(Coord{row: i, col: 14}, MdUpRight).is_err(), true);
        assert_eq!(b.move_to(Coord{row: i, col: 14}, MdDownRight).is_err(), true);
    }
}
