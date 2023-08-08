```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::board::Board;
    use crate::chess::pieces::rook::Rook;
    use crate::chess::pieces::Piece;
    use crate::chess::game::Game;
    use crate::chess::move::Move;

    #[test]
    fn test_rook_valid_castling() {
        let mut game = Game::new();
        let rook = Rook::new(PieceColor::White);
        let move = Move::new((0, 0), (0, 3));
        game.board.set_piece_at(0, 0, Piece::Rook(rook));
        assert!(game.is_valid_move(&move));
        assert!(game.is_castling_move(&move));
    }

    #[test]
    fn test_rook_invalid_castling() {
        let mut game = Game::new();
        let rook = Rook::new(PieceColor::White);
        let move = Move::new((0, 0), (0, 2));
        game.board.set_piece_at(0, 0, Piece::Rook(rook));
        assert!(!game.is_valid_move(&move));
        assert!(!game.is_castling_move(&move));
    }

    #[test]
    fn test_rook_perform_castling() {
        let mut game = Game::new();
        let rook = Rook::new(PieceColor::White);
        let move = Move::new((0, 0), (0, 3));
        game.board.set_piece_at(0, 0, Piece::Rook(rook));
        assert!(game.is_valid_move(&move));
        assert!(game.is_castling_move(&move));
        game.make_move(&move);
        assert_eq!(game.board.get_piece_at(0, 3), Some(Piece::Rook(rook)));
    }
}
```