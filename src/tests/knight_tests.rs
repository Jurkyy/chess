```rust
#[cfg(test)]
mod tests {
    use super::super::knight::Knight;
    use super::super::piece::Piece;
    use super::super::board::Board;

    #[test]
    fn test_knight_moves() {
        let mut board = Board::init();
        let knight = Knight::new(1, 2, "white");
        board.set_piece(Box::new(knight), 1, 2);

        let moves = board.get_piece(1, 2).unwrap().get_moves(&board);
        assert_eq!(moves.len(), 3);
        assert!(moves.contains(&(2, 4)));
        assert!(moves.contains(&(3, 3)));
        assert!(moves.contains(&(3, 1)));
    }

    #[test]
    fn test_knight_jump_over() {
        let mut board = Board::init();
        let knight = Knight::new(1, 2, "white");
        board.set_piece(Box::new(knight), 1, 2);

        let pawn = Piece::new(2, 2, "white", "Pawn");
        board.set_piece(Box::new(pawn), 2, 2);

        let moves = board.get_piece(1, 2).unwrap().get_moves(&board);
        assert_eq!(moves.len(), 3);
        assert!(moves.contains(&(2, 4)));
        assert!(moves.contains(&(3, 3)));
        assert!(moves.contains(&(3, 1)));
    }
}
```