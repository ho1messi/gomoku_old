use super::super::rule_checker_observer::*;
use super::super::rule_checker::*;
use super::super::board::*;

use super::super::cross_point::ChessType::*;

#[test]
fn it_works() {
    let mut board = Board::new();
    let mut rule_checker = RuleChecker::create_with_detail(&board);
    let mut rule_checker_observer =
        RuleCheckerObserver::create_with_detail(&mut rule_checker);

    board.add_observers(rule_checker_observer);

    println!("1");
    board.put_chess_at(0, 0, CtBlack);
    println!("2");
    board.put_chess_at(0, 1, CtBlack);
    println!("3");
    board.remove_chess_at(0, 0);
    println!("4");
    board.remove_chess_at(0, 1);
}