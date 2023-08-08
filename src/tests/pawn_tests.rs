```rust
#[cfg(test)]
mod tests {
    use super::super::pawn::Pawn;
    use super::super::piece::Piece;
    use super::super::board::Board;

    #[test]
    fn test_pawn_move_forward() {
        let mut board = Board::init();
        let pawn = Pawn::new(1, 1, "White");
        assert!(pawn.move(&mut board, 1, 2));
        assert_eq!(pawn.position, (1, 2));
    }

    #[test]
    fn test_pawn_capture_diagonal() {
        let mut board = Board::init();
        let pawn = Pawn::new(1, 1, "White");
        let opponent_pawn = Pawn::new(2, 2, "Black");
        board.set_piece(Box::new(opponent_pawn));
        assert!(pawn.capture(&mut board, 2, 2));
        assert_eq!(pawn.position, (2, 2));
    }

    #[test]
    fn test_pawn_first_move_two_squares() {
        let mut board = Board::init();
        let pawn = Pawn::new(1, 1, "White");
        assert!(pawn.move(&mut board, 1, 3));
        assert_eq!(pawn.position, (1, 3));
    }

    #[test]
    fn test_pawn_en_passant() {
        let mut board = Board::init();
        let pawn = Pawn::new(1, 1, "White");
        let opponent_pawn = Pawn::new(2, 3, "Black");
        board.set_piece(Box::new(opponent_pawn));
        assert!(pawn.capture(&mut board, 2, 3));
        assert_eq!(pawn.position, (2, 3));
    }

    #[test]
    fn test_pawn_promotion() {
        let mut board = Board::init();
        let pawn = Pawn::new(1, 7, "White");
        assert!(pawn.move(&mut board, 1, 8));
        assert_eq!(pawn.piece_type, "Queen");
    }
}
```