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
                assert_eq!(b.is_index_valid(i, j), true);
            } else {
                assert_eq!(b.is_index_valid(i, j), false);
            }
        }
    }
}

#[test]
fn chess() {
    let mut b = Board::new();

    for i in 0..15 {
        for j in 0..15 {
            assert_eq!(b.have_chess_at(i, j), false);
            b.put_chess_at(i, j, ChessType::CtBlack);
            assert_eq!(b.have_chess_at(i, j), true);
            assert_eq!(b.get_chess_at(i, j), ChessType::CtBlack);
            b.remove_chess_at(i, j);
            assert_eq!(b.have_chess_at(i, j), false);
            b.put_chess_at(i, j, ChessType::CtWhite);
            assert_eq!(b.have_chess_at(i, j), true);
            assert_eq!(b.get_chess_at(i, j), ChessType::CtWhite);
        }
    }
}

#[test]
fn move_to() {
    let b = Board::new();

    for i in 1..14 {
        for j in 1..14 {
            assert_eq!(b.move_to(i, j, MdUp), Ok((i - 1, j)));
            assert_eq!(b.move_to(i, j, MdDown), Ok((i + 1, j)));
            assert_eq!(b.move_to(i, j, MdLeft), Ok((i, j - 1)));
            assert_eq!(b.move_to(i, j, MdRight), Ok((i, j + 1)));
            assert_eq!(b.move_to(i, j, MdUpLeft), Ok((i - 1, j - 1)));
            assert_eq!(b.move_to(i, j, MdUpRight), Ok((i - 1, j + 1)));
            assert_eq!(b.move_to(i, j, MdDownLeft), Ok((i + 1, j - 1)));
            assert_eq!(b.move_to(i, j, MdDownRight), Ok((i + 1, j + 1)));
        }
    }

    for i in 0..15 {
        assert_eq!(b.move_to(0, i, MdUp).is_err(), true);
        assert_eq!(b.move_to(0, i, MdUpLeft).is_err(), true);
        assert_eq!(b.move_to(0, i, MdUpRight).is_err(), true);
        assert_eq!(b.move_to(14, i, MdDown).is_err(), true);
        assert_eq!(b.move_to(14, i, MdDownLeft).is_err(), true);
        assert_eq!(b.move_to(14, i, MdDownRight).is_err(), true);
        assert_eq!(b.move_to(i, 0, MdLeft).is_err(), true);
        assert_eq!(b.move_to(i, 0, MdUpLeft).is_err(), true);
        assert_eq!(b.move_to(i, 0, MdDownLeft).is_err(), true);
        assert_eq!(b.move_to(i, 14, MdRight).is_err(), true);
        assert_eq!(b.move_to(i, 14, MdUpRight).is_err(), true);
        assert_eq!(b.move_to(i, 14, MdDownRight).is_err(), true);
    }
}
