```rust
#[cfg(test)]
mod tests {
    use super::super::rook::Rook;
    use super::super::board::Board;
    use super::super::piece::Piece;
    use super::super::piece::Color;

    #[test]
    fn test_rook_movement() {
        let mut board = Board::init();
        let rook = Rook::new(Color::White, (0, 0));
        board.set_piece(Box::new(rook), (0, 0));

        // Test vertical movement
        assert!(board.move_piece((0, 0), (0, 3)).is_ok());

        // Test horizontal movement
        assert!(board.move_piece((0, 3), (3, 3)).is_ok());

        // Test invalid movement
        assert!(board.move_piece((3, 3), (4, 5)).is_err());
    }

    #[test]
    fn test_rook_capture() {
        let mut board = Board::init();
        let rook = Rook::new(Color::White, (0, 0));
        let enemy = Piece::new(Color::Black, (0, 3));
        board.set_piece(Box::new(rook), (0, 0));
        board.set_piece(Box::new(enemy), (0, 3));

        // Test capture
        assert!(board.move_piece((0, 0), (0, 3)).is_ok());
        assert!(board.get_piece((0, 3)).unwrap().get_color() == Color::White);
    }

    #[test]
    fn test_rook_castling() {
        let mut board = Board::init();
        let rook = Rook::new(Color::White, (0, 0));
        let king = Piece::new(Color::White, (0, 4));
        board.set_piece(Box::new(rook), (0, 0));
        board.set_piece(Box::new(king), (0, 4));

        // Test castling
        assert!(board.castling(Color::White, true).is_ok());
        assert!(board.get_piece((0, 2)).unwrap().get_color() == Color::White);
        assert!(board.get_piece((0, 3)).unwrap().get_color() == Color::White);
    }
}
```