```rust
#[cfg(test)]
mod tests {
    use super::super::queen::Queen;
    use super::super::piece::Piece;
    use super::super::board::Board;

    #[test]
    fn test_queen_movement() {
        let mut board = Board::init();
        let queen = Queen::new(4, 4, "White");
        board.set_piece(4, 4, Box::new(queen));

        // Test queen's horizontal movement
        assert_eq!(board.move_piece(4, 4, 4, 6), Ok(()));
        assert_eq!(board.get_piece(4, 6).unwrap().get_type(), "Queen");

        // Test queen's vertical movement
        assert_eq!(board.move_piece(4, 6, 2, 6), Ok(()));
        assert_eq!(board.get_piece(2, 6).unwrap().get_type(), "Queen");

        // Test queen's diagonal movement
        assert_eq!(board.move_piece(2, 6, 4, 8), Ok(()));
        assert_eq!(board.get_piece(4, 8).unwrap().get_type(), "Queen");
    }

    #[test]
    fn test_queen_invalid_movement() {
        let mut board = Board::init();
        let queen = Queen::new(4, 4, "White");
        board.set_piece(4, 4, Box::new(queen));

        // Test queen's invalid movement
        assert_eq!(board.move_piece(4, 4, 5, 6), Err("Invalid move"));
    }

    #[test]
    fn test_queen_capture() {
        let mut board = Board::init();
        let queen = Queen::new(4, 4, "White");
        let pawn = Piece::new(4, 5, "Pawn", "Black");
        board.set_piece(4, 4, Box::new(queen));
        board.set_piece(4, 5, Box::new(pawn));

        // Test queen's capture
        assert_eq!(board.move_piece(4, 4, 4, 5), Ok(()));
        assert_eq!(board.get_piece(4, 5).unwrap().get_type(), "Queen");
    }
}
```