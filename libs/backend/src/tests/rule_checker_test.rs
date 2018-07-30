use std::rc::*;
use std::cell::*;

use super::super::board::*;
use super::super::rule_checker::*;

use super::super::cross_point::ChessType::*;
use super::super::rule_checker::GameStatus::*;

#[test]
fn check_game_status() {
    for row in 0..10 {
        for col in 0..10 {
            let board = Board::new();
            let rule_checker = RuleChecker::create_with_detail(board.clone());

            board.put_chess_at(Coord{row: row + 0, col: col + 0}, CtBlack);     // X O _ X O
            board.put_chess_at(Coord{row: row + 0, col: col + 1}, CtWhite);     // O _ X O _
            board.put_chess_at(Coord{row: row + 0, col: col + 3}, CtBlack);     // _ X O _ X
            board.put_chess_at(Coord{row: row + 0, col: col + 4}, CtWhite);     // X O _ X O
            board.put_chess_at(Coord{row: row + 1, col: col + 0}, CtWhite);     // _ _ X O _
            board.put_chess_at(Coord{row: row + 1, col: col + 2}, CtBlack);
            board.put_chess_at(Coord{row: row + 1, col: col + 3}, CtWhite);
            board.put_chess_at(Coord{row: row + 2, col: col + 1}, CtBlack);
            board.put_chess_at(Coord{row: row + 2, col: col + 2}, CtWhite);
            board.put_chess_at(Coord{row: row + 2, col: col + 4}, CtBlack);
            board.put_chess_at(Coord{row: row + 3, col: col + 0}, CtBlack);
            board.put_chess_at(Coord{row: row + 3, col: col + 1}, CtWhite);
            board.put_chess_at(Coord{row: row + 3, col: col + 3}, CtBlack);
            board.put_chess_at(Coord{row: row + 3, col: col + 4}, CtWhite);
            board.put_chess_at(Coord{row: row + 4, col: col + 2}, CtBlack);
            board.put_chess_at(Coord{row: row + 4, col: col + 3}, CtWhite);     // X O _ X O
            assert_eq!(rule_checker.check_game_status(), GsGameContinue);       // O _ X O _
                                                                                // _ X O _ X
            board.put_chess_at(Coord{row: row + 4, col: col + 0}, CtWhite);     // X O _ X O
            assert_eq!(rule_checker.check_game_status(), GsGameOver(CtWhite));  // O _ X O _
            assert_eq!(rule_checker.game_status(), GsGameOver(CtWhite));

            board.remove_chess_at(Coord{row: row + 2, col: col + 2});           // X O _ X O
            board.put_chess_at(Coord{row: row + 1, col: col + 1}, CtBlack);     // O X X O _
            board.put_chess_at(Coord{row: row + 2, col: col + 2}, CtBlack);     // _ X X _ X
            board.put_chess_at(Coord{row: row + 4, col: col + 4}, CtBlack);     // X O _ X O
            assert_eq!(rule_checker.check_game_status(), GsGameOver(CtBlack));  // O _ X O X
            assert_eq!(rule_checker.game_status(), GsGameOver(CtBlack));

            board.remove_chess_at(Coord{row: row + 0, col: col + 0});           // _ O X X O
            board.put_chess_at(Coord{row: row + 0, col: col + 2}, CtBlack);     // O X X O _
            board.put_chess_at(Coord{row: row + 3, col: col + 2}, CtBlack);     // _ X X _ X
            assert_eq!(rule_checker.check_game_status(), GsGameOver(CtBlack));  // X O X X O
            assert_eq!(rule_checker.game_status(), GsGameOver(CtBlack));        // O _ X O X

            board.remove_chess_at(Coord{row: row + 1, col: col + 1});
            board.remove_chess_at(Coord{row: row + 1, col: col + 2});           // _ O X X O
            board.put_chess_at(Coord{row: row + 1, col: col + 1}, CtWhite);     // O O O O O
            board.put_chess_at(Coord{row: row + 1, col: col + 2}, CtWhite);     // _ X X _ X
            board.put_chess_at(Coord{row: row + 1, col: col + 4}, CtWhite);     // X O X X O
            assert_eq!(rule_checker.check_game_status(), GsGameOver(CtWhite));  // O _ X O X
            assert_eq!(rule_checker.game_status(), GsGameOver(CtWhite));
        }
    }
}

#[test]
fn update_evaluation_by_event() {
    let board = Board::create_with_size(5);
    let rule_checker = RuleChecker::create_with_detail(board.clone());

    println!("strong count of board is {}", Rc::strong_count(&board));
    println!("strong count of rule_checker is {}", Rc::strong_count(&rule_checker));

    for row in 0..board.size() {
        for col in 0..board.size() {
            let coord = Coord{row, col};
            println!("strong count of cross_point_{} in board is {}", board.coord_to_index(coord),
                     Rc::strong_count(&board.get_cross_point_at(coord)));
        }
    }

    //let a = Rc::into_raw(rule_checker.clone());
    //board.remove_observers(rule_checker.clone());
    //println!("strong count of rule_checker is {}", Rc::strong_count(&rule_checker));

    assert!(false);
}