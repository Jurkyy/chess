use crate::game_components::chessboard::Chessboard;
use crate::game_components::chessboard::Square;

pub fn validate_move(chessboard: &Chessboard, start: (usize, usize), end: (usize, usize)) -> bool {
    match &chessboard.get_square(start.0, start.1) {
        Square::Empty => {
            println!("No piece at the starting position.");
            false
        }
        Square::Occupied(piece) => {
            let valid_moves = piece.valid_moves(start, chessboard);
            if valid_moves.contains(&end) {
                true
            } else {
                println!("Invalid move for this piece.");
                false
            }
        }
    }
}
