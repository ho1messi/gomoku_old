use super::super::cross_point::*;

#[test]
fn it_works() {
    let cp = CrossPoint::new();
    assert_eq!(cp.have_chess(), false);

    let mut cp = CrossPoint::create_with_chess(ChessType::CtBlack);
    assert_eq!(cp.have_chess(), true);
    assert_eq!(cp.get_chess(), ChessType::CtBlack);
    cp.remove_chess();
    assert_eq!(cp.have_chess(), false);
    cp.put_chess(ChessType::CtWhite);
    assert_eq!(cp.have_chess(), true);
    assert_eq!(cp.get_chess(), ChessType::CtWhite);

    let mut cp = CrossPoint::create_with_chess(ChessType::CtWhite);
    assert_eq!(cp.have_chess(), true);
    assert_eq!(cp.get_chess(), ChessType::CtWhite);
    cp.remove_chess();
    assert_eq!(cp.have_chess(), false);
    cp.put_chess(ChessType::CtBlack);
    assert_eq!(cp.have_chess(), true);
    assert_eq!(cp.get_chess(), ChessType::CtBlack);
}

#[test]
#[should_panic]
fn get_chess_panic() {
    let cp = CrossPoint::new();
    cp.get_chess();
}

#[test]
#[should_panic]
fn remove_chess_panic() {
    let mut cp = CrossPoint::new();
    cp.remove_chess();
}

#[test]
#[should_panic]
fn put_chess_panic_1() {
    let mut cp = CrossPoint::create_with_chess(ChessType::CtBlack);
    cp.put_chess(ChessType::CtWhite);
}

#[test]
#[should_panic]
fn put_chess_panic_2() {
    let mut cp = CrossPoint::create_with_chess(ChessType::CtWhite);
    cp.put_chess(ChessType::CtBlack);
}
