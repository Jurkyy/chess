use crate::game_components::chess_piece::ChessPiece;
use crate::game_components::chessboard::Chessboard;
use crate::game_components::chessboard::Square;

pub fn validate_move(
    chessboard: &Chessboard,
    piece: &dyn ChessPiece,
    start: (usize, usize),
    end: (usize, usize),
) -> bool {
    match &chessboard.get_square(start.0, start.1) {
        Square::Empty => false,
        Square::Occupied(boxed_piece) => {
            if boxed_piece.as_ref().color() != piece.color() {
                return false;
            }
            let valid_moves = piece.valid_moves(start, chessboard);
            valid_moves.contains(&end)
        }
    }
}
