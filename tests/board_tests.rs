```rust
#[cfg(test)]
mod board_tests {
    use super::super::src::chess::board::Board;
    use super::super::src::chess::pieces::{Piece, King, Rook};
    use super::super::src::chess::utils::Position;

    #[test]
    fn test_move_piece() {
        let mut board = Board::new();
        let king = King::new(Position::new(4, 0), true);
        let rook = Rook::new(Position::new(7, 0), true);
        board.set_piece(Box::new(king));
        board.set_piece(Box::new(rook));

        assert!(board.move_piece(&Position::new(4, 0), &Position::new(4, 1)).is_ok());
        assert!(board.get_piece(&Position::new(4, 1)).unwrap().get_position() == &Position::new(4, 1));
    }

    #[test]
    fn test_invalid_move() {
        let mut board = Board::new();
        let king = King::new(Position::new(4, 0), true);
        board.set_piece(Box::new(king));

        assert!(board.move_piece(&Position::new(4, 0), &Position::new(5, 2)).is_err());
    }

    #[test]
    fn test_castling() {
        let mut board = Board::new();
        let king = King::new(Position::new(4, 0), true);
        let rook = Rook::new(Position::new(7, 0), true);
        board.set_piece(Box::new(king));
        board.set_piece(Box::new(rook));

        assert!(board.move_piece(&Position::new(4, 0), &Position::new(6, 0)).is_ok());
        assert!(board.get_piece(&Position::new(6, 0)).unwrap().get_position() == &Position::new(6, 0));
        assert!(board.get_piece(&Position::new(5, 0)).unwrap().get_position() == &Position::new(5, 0));
    }
}
```