```rust
#[cfg(test)]
mod pieces_tests {
    use super::super::src::chess::board::Board;
    use super::super::src::chess::pieces::{King, Rook, Pawn, Knight, Bishop, Queen};
    use super::super::src::chess::utils::Position;

    #[test]
    fn test_move() {
        let mut board = Board::new();
        let pawn = Pawn::new(Position::new(1, 1), true);
        assert!(pawn.is_valid_move(&board, &Position::new(1, 2)));
        assert!(!pawn.is_valid_move(&board, &Position::new(1, 3)));
    }

    #[test]
    fn test_castle() {
        let mut board = Board::new();
        let king = King::new(Position::new(4, 0), true);
        let rook = Rook::new(Position::new(7, 0), true);
        board.set_piece(Box::new(king));
        board.set_piece(Box::new(rook));
        assert!(board.can_castle(true));
    }

    #[test]
    fn test_check() {
        let mut board = Board::new();
        let king = King::new(Position::new(4, 0), true);
        let queen = Queen::new(Position::new(4, 7), false);
        board.set_piece(Box::new(king));
        board.set_piece(Box::new(queen));
        assert!(board.is_in_check(true));
    }

    #[test]
    fn test_checkmate() {
        let mut board = Board::new();
        let king = King::new(Position::new(4, 0), true);
        let queen = Queen::new(Position::new(4, 7), false);
        let bishop = Bishop::new(Position::new(5, 6), false);
        board.set_piece(Box::new(king));
        board.set_piece(Box::new(queen));
        board.set_piece(Box::new(bishop));
        assert!(board.is_in_checkmate(true));
    }
}
```