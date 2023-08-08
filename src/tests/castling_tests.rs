```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::color::Color;
    use crate::chess::board::Board;
    use crate::chess::game::Game;
    use crate::chess::king::King;
    use crate::chess::rook::Rook;

    #[test]
    fn test_castling() {
        let mut game = Game::new();
        let board = Board::new();

        // Place the king and rook at their initial positions
        let king = King::new(Color::White);
        let rook = Rook::new(Color::White);
        board.set_piece_at(4, 0, Some(Box::new(king)));
        board.set_piece_at(7, 0, Some(Box::new(rook)));

        // Test castling move
        let castling_move = Move::new((4, 0), (6, 0));
        assert!(game.is_valid_move(&castling_move));
        game.make_move(castling_move);
        assert_eq!(game.board.get_piece_at(6, 0).unwrap().get_color(), Color::White);
        assert_eq!(game.board.get_piece_at(5, 0).unwrap().get_color(), Color::White);
    }

    #[test]
    fn test_invalid_castling() {
        let mut game = Game::new();
        let board = Board::new();

        // Place the king and rook at their initial positions
        let king = King::new(Color::White);
        let rook = Rook::new(Color::White);
        board.set_piece_at(4, 0, Some(Box::new(king)));
        board.set_piece_at(7, 0, Some(Box::new(rook)));

        // Move the king, invalidating castling
        let king_move = Move::new((4, 0), (4, 1));
        game.make_move(king_move);

        // Test castling move
        let castling_move = Move::new((4, 1), (6, 1));
        assert!(!game.is_valid_move(&castling_move));
    }
}
```