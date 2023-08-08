```rust
#[cfg(test)]
mod tests {
    use super::Board;
    use super::Piece;
    use super::Color;
    use super::Position;

    #[test]
    fn test_new() {
        let board = Board::new();
        assert_eq!(board.get_piece(Position::new(0, 0)), Some(Piece::new(Color::White, PieceType::Rook)));
        assert_eq!(board.get_piece(Position::new(7, 7)), Some(Piece::new(Color::Black, PieceType::Rook)));
        assert_eq!(board.get_piece(Position::new(3, 3)), None);
    }

    #[test]
    fn test_move_piece() {
        let mut board = Board::new();
        assert!(board.move_piece(Position::new(1, 0), Position::new(3, 0)).is_ok());
        assert_eq!(board.get_piece(Position::new(3, 0)), Some(Piece::new(Color::White, PieceType::Pawn)));
        assert_eq!(board.get_piece(Position::new(1, 0)), None);
    }

    #[test]
    fn test_is_valid_move() {
        let board = Board::new();
        assert!(board.is_valid_move(Position::new(1, 0), Position::new(3, 0)));
        assert!(!board.is_valid_move(Position::new(0, 0), Position::new(3, 0)));
    }

    #[test]
    fn test_get_piece() {
        let board = Board::new();
        assert_eq!(board.get_piece(Position::new(0, 0)), Some(Piece::new(Color::White, PieceType::Rook)));
        assert_eq!(board.get_piece(Position::new(7, 7)), Some(Piece::new(Color::Black, PieceType::Rook)));
        assert_eq!(board.get_piece(Position::new(3, 3)), None);
    }
}
```