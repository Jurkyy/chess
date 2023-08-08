```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::piece::{Piece, Color};

    #[test]
    fn test_piece_new() {
        let piece = Piece::new(Color::White, PieceType::Pawn);
        assert_eq!(piece.color, Color::White);
        assert_eq!(piece.piece_type, PieceType::Pawn);
    }

    #[test]
    fn test_piece_move() {
        let mut piece = Piece::new(Color::White, PieceType::Pawn);
        let new_position = Position::new(1, 2);
        piece.move_piece(new_position);
        assert_eq!(piece.position, new_position);
    }

    #[test]
    fn test_piece_capture() {
        let mut piece = Piece::new(Color::White, PieceType::Pawn);
        let opponent_piece = Piece::new(Color::Black, PieceType::Pawn);
        assert!(piece.capture(opponent_piece));
    }

    #[test]
    fn test_piece_promotion() {
        let mut piece = Piece::new(Color::White, PieceType::Pawn);
        piece.promote(PieceType::Queen);
        assert_eq!(piece.piece_type, PieceType::Queen);
    }

    #[test]
    fn test_piece_invalid_move() {
        let mut piece = Piece::new(Color::White, PieceType::Pawn);
        let new_position = Position::new(8, 8);
        assert!(!piece.is_valid_move(new_position));
    }
}
```