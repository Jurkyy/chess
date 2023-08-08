```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::rook::Rook;
    use crate::chess::board::Board;
    use crate::chess::game::Game;
    use crate::chess::piece::Piece;
    use crate::chess::PIECES;

    #[test]
    fn test_rook_valid_moves() {
        let mut game = Game::new();
        let rook = Piece::new(PIECES::Rook, "white", (0, 0));
        game.board.set_piece_at(rook.position, Some(rook));

        let valid_moves = game.board.get_valid_moves((0, 0));
        assert_eq!(valid_moves.len(), 14);
    }

    #[test]
    fn test_rook_invalid_moves() {
        let mut game = Game::new();
        let rook = Piece::new(PIECES::Rook, "white", (0, 0));
        game.board.set_piece_at(rook.position, Some(rook));

        assert!(!game.board.is_valid_move((0, 0), (2, 2)));
    }

    #[test]
    fn test_rook_capture() {
        let mut game = Game::new();
        let rook = Piece::new(PIECES::Rook, "white", (0, 0));
        let pawn = Piece::new(PIECES::Pawn, "black", (0, 1));
        game.board.set_piece_at(rook.position, Some(rook));
        game.board.set_piece_at(pawn.position, Some(pawn));

        assert!(game.board.is_valid_move((0, 0), (0, 1)));
    }

    #[test]
    fn test_rook_castling() {
        let mut game = Game::new();
        let rook = Piece::new(PIECES::Rook, "white", (0, 0));
        let king = Piece::new(PIECES::King, "white", (0, 4));
        game.board.set_piece_at(rook.position, Some(rook));
        game.board.set_piece_at(king.position, Some(king));

        assert!(game.can_castle("white"));
    }
}
```