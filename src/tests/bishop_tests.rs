```rust
#[cfg(test)]
mod tests {
    use super::super::bishop::Bishop;
    use super::super::piece::Piece;
    use super::super::board::Board;

    #[test]
    fn test_bishop_moves() {
        let mut board = Board::init();
        let bishop = Bishop::new(2, 0, "White");
        board.set_piece(2, 0, Box::new(bishop));
        let moves = board.get_piece(2, 0).unwrap().get_moves(&board);
        assert_eq!(moves.len(), 7);
    }

    #[test]
    fn test_bishop_captures() {
        let mut board = Board::init();
        let bishop = Bishop::new(2, 0, "White");
        board.set_piece(2, 0, Box::new(bishop));
        let opponent_bishop = Bishop::new(4, 2, "Black");
        board.set_piece(4, 2, Box::new(opponent_bishop));
        let moves = board.get_piece(2, 0).unwrap().get_moves(&board);
        assert!(moves.contains(&(4, 2)));
    }

    #[test]
    fn test_bishop_blocked() {
        let mut board = Board::init();
        let bishop = Bishop::new(2, 0, "White");
        board.set_piece(2, 0, Box::new(bishop));
        let blocking_bishop = Bishop::new(3, 1, "White");
        board.set_piece(3, 1, Box::new(blocking_bishop));
        let moves = board.get_piece(2, 0).unwrap().get_moves(&board);
        assert_eq!(moves.len(), 1);
    }
}
```