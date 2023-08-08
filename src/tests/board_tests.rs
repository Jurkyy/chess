```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::board::Board;
    use crate::chess::piece::Piece;
    use crate::chess::color::Color;
    use crate::chess::king::King;
    use crate::chess::rook::Rook;

    #[test]
    fn test_board_initialization() {
        let board = Board::new();
        assert_eq!(board.get_piece_at(0, 0), Some(Piece::Rook(Rook::new(Color::White))));
        assert_eq!(board.get_piece_at(7, 7), Some(Piece::Rook(Rook::new(Color::Black))));
        assert_eq!(board.get_piece_at(4, 0), Some(Piece::King(King::new(Color::White))));
        assert_eq!(board.get_piece_at(4, 7), Some(Piece::King(King::new(Color::Black))));
    }

    #[test]
    fn test_board_move_piece() {
        let mut board = Board::new();
        assert!(board.move_piece((1, 0), (3, 0)).is_ok());
        assert_eq!(board.get_piece_at(1, 0), None);
        assert_eq!(board.get_piece_at(3, 0), Some(Piece::Pawn(Pawn::new(Color::White))));
    }

    #[test]
    fn test_board_invalid_move() {
        let mut board = Board::new();
        assert!(board.move_piece((0, 0), (5, 0)).is_err());
    }

    #[test]
    fn test_board_castling() {
        let mut board = Board::new();
        assert!(board.move_piece((4, 0), (6, 0)).is_err()); // Castling not possible yet
        assert!(board.move_piece((1, 0), (3, 0)).is_ok()); // Move pawn out of the way
        assert!(board.move_piece((6, 0), (5, 0)).is_ok()); // Move knight out of the way
        assert!(board.move_piece((4, 0), (6, 0)).is_ok()); // Castling should be possible now
        assert_eq!(board.get_piece_at(4, 0), None);
        assert_eq!(board.get_piece_at(5, 0), Some(Piece::Rook(Rook::new(Color::White))));
        assert_eq!(board.get_piece_at(6, 0), Some(Piece::King(King::new(Color::White))));
    }
}
```