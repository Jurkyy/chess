```rust
#[cfg(test)]
mod tests {
    use super::Board;

    #[test]
    fn test_init_board() {
        let board = Board::init();
        assert_eq!(board.pieces.len(), 32);
    }

    #[test]
    fn test_move_piece() {
        let mut board = Board::init();
        let pawn = board.pieces[0].clone();
        let result = board.move_piece(&pawn, (2, 0));
        assert!(result.is_ok());
    }

    #[test]
    fn test_capture_piece() {
        let mut board = Board::init();
        let pawn = board.pieces[0].clone();
        board.move_piece(&pawn, (2, 0)).unwrap();
        let opponent_pawn = board.pieces[16].clone();
        let result = board.move_piece(&opponent_pawn, (2, 0));
        assert!(result.is_err());
    }

    #[test]
    fn test_promote_pawn() {
        let mut board = Board::init();
        let pawn = board.pieces[0].clone();
        board.move_piece(&pawn, (7, 0)).unwrap();
        assert_eq!(pawn.piece_type, PieceType::Queen);
    }

    #[test]
    fn test_castling() {
        let mut board = Board::init();
        let king = board.pieces[4].clone();
        let rook = board.pieces[0].clone();
        let result = board.castling(&king, &rook);
        assert!(result.is_ok());
    }
}
```