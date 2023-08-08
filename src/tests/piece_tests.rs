```rust
#[cfg(test)]
mod tests {
    use super::Piece;
    use super::Board;

    #[test]
    fn test_piece_creation() {
        let piece = Piece::new("Pawn", "White", (0, 1));
        assert_eq!(piece.piece_type, "Pawn");
        assert_eq!(piece.color, "White");
        assert_eq!(piece.position, (0, 1));
    }

    #[test]
    fn test_piece_move() {
        let mut piece = Piece::new("Pawn", "White", (0, 1));
        piece.move_piece((0, 2));
        assert_eq!(piece.position, (0, 2));
    }

    #[test]
    fn test_piece_capture() {
        let mut board = Board::new();
        let mut piece = Piece::new("Pawn", "White", (0, 1));
        board.place_piece(piece);
        let opponent_piece = Piece::new("Pawn", "Black", (1, 2));
        board.place_piece(opponent_piece);
        piece.capture((1, 2), &mut board);
        assert_eq!(board.get_piece((1, 2)), None);
    }

    #[test]
    fn test_piece_promotion() {
        let mut board = Board::new();
        let mut piece = Piece::new("Pawn", "White", (0, 7));
        board.place_piece(piece);
        piece.promote("Queen");
        assert_eq!(piece.piece_type, "Queen");
    }

    #[test]
    fn test_piece_castling() {
        let mut board = Board::new();
        let mut king = Piece::new("King", "White", (4, 0));
        let rook = Piece::new("Rook", "White", (7, 0));
        board.place_piece(king);
        board.place_piece(rook);
        king.castling(&mut board);
        assert_eq!(king.position, (6, 0));
        assert_eq!(board.get_piece((5, 0)).unwrap().piece_type, "Rook");
    }
}
```