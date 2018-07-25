use super::super::board::*;
use super::super::rule_checker::*;

use super::super::cross_point::ChessType::*;
use super::super::rule_checker::GameStatus::*;

#[test]
fn check_game_status() {
    for row in 0..10 {
        for col in 0..10 {
            let mut board = Board::new();
            let mut rc = RuleChecker::create_with_detail(&board as *const Board);

            board.put_chess_at(row + 0, col + 0, CtBlack);  // X O _ X O
            board.put_chess_at(row + 0, col + 1, CtWhite);  // O _ X O _
            board.put_chess_at(row + 0, col + 3, CtBlack);  // _ X O _ X
            board.put_chess_at(row + 0, col + 4, CtWhite);  // X O _ X O
            board.put_chess_at(row + 1, col + 0, CtWhite);  // _ _ X O _
            board.put_chess_at(row + 1, col + 2, CtBlack);
            board.put_chess_at(row + 1, col + 3, CtWhite);
            board.put_chess_at(row + 2, col + 1, CtBlack);
            board.put_chess_at(row + 2, col + 2, CtWhite);
            board.put_chess_at(row + 2, col + 4, CtBlack);
            board.put_chess_at(row + 3, col + 0, CtBlack);
            board.put_chess_at(row + 3, col + 1, CtWhite);
            board.put_chess_at(row + 3, col + 3, CtBlack);
            board.put_chess_at(row + 3, col + 4, CtWhite);
            board.put_chess_at(row + 4, col + 2, CtBlack);
            board.put_chess_at(row + 4, col + 3, CtWhite);  // X O _ X O
            assert_eq!(rc.check_game_status(), GsGameContinue);           // O _ X O _
                                                                          // _ X O _ X
            board.put_chess_at(row + 4, col + 0, CtWhite);  // X O _ X O
            assert_eq!(rc.check_game_status(), GsGameOver(CtWhite));      // O _ X O _
            assert_eq!(rc.game_status(), GsGameOver(CtWhite));

            board.remove_chess_at(row + 2, col + 2);              // X O _ X O
            board.put_chess_at(row + 1, col + 1, CtBlack);  // O X X O _
            board.put_chess_at(row + 2, col + 2, CtBlack);  // _ X X _ X
            board.put_chess_at(row + 4, col + 4, CtBlack);  // X O _ X O
            assert_eq!(rc.check_game_status(), GsGameOver(CtBlack));      // O _ X O X
            assert_eq!(rc.game_status(), GsGameOver(CtBlack));

            board.remove_chess_at(row + 0, col + 0);              // _ O X X O
            board.put_chess_at(row + 0, col + 2, CtBlack);  // O X X O _
            board.put_chess_at(row + 3, col + 2, CtBlack);  // _ X X _ X
            assert_eq!(rc.check_game_status(), GsGameOver(CtBlack));      // X O X X O
            assert_eq!(rc.game_status(), GsGameOver(CtBlack));            // O _ X O X

            board.remove_chess_at(row + 1, col + 1);
            board.remove_chess_at(row + 1, col + 2);              // _ O X X O
            board.put_chess_at(row + 1, col + 1, CtWhite);  // O O O O O
            board.put_chess_at(row + 1, col + 2, CtWhite);  // _ X X _ X
            board.put_chess_at(row + 1, col + 4, CtWhite);  // X O X X O
            assert_eq!(rc.check_game_status(), GsGameOver(CtWhite));      // O _ X O X
            assert_eq!(rc.game_status(), GsGameOver(CtWhite));
        }
    }
}