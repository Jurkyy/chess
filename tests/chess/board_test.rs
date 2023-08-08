```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::board::Board;
    use crate::chess::pieces::{Piece, King, Rook};
    use crate::chess::game::Game;
    use crate::chess::move::Move;

    #[test]
    fn test_is_valid_move_castling() {
        let mut board = Board::new();
        let king = King::new(PieceColor::White);
        let rook = Rook::new(PieceColor::White);

        board.set_piece_at(4, 0, Piece::King(king));
        board.set_piece_at(7, 0, Piece::Rook(rook));

        let castling_move = Move::new((4, 0), (6, 0));
        assert!(board.is_valid_move(&castling_move));
    }

    #[test]
    fn test_make_move_castling() {
        let mut board = Board::new();
        let king = King::new(PieceColor::White);
        let rook = Rook::new(PieceColor::White);

        board.set_piece_at(4, 0, Piece::King(king));
        board.set_piece_at(7, 0, Piece::Rook(rook));

        let castling_move = Move::new((4, 0), (6, 0));
        board.make_move(&castling_move);

        match board.get_piece_at(6, 0) {
            Some(Piece::King(_)) => assert!(true),
            _ => assert!(false),
        }

        match board.get_piece_at(5, 0) {
            Some(Piece::Rook(_)) => assert!(true),
            _ => assert!(false),
        }
    }
}
```