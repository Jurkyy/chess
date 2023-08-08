```rust
#[cfg(test)]
mod tests {
    use super::super::pieces::{Piece, Pawn, Knight, Bishop, Rook, Queen, King};
    use super::super::board::Board;

    #[test]
    fn test_pawn_moves() {
        let pawn = Pawn::new(true);
        assert_eq!(pawn.valid_moves(), vec![(0, 1), (0, 2)]);
    }

    #[test]
    fn test_knight_moves() {
        let knight = Knight::new(true);
        assert_eq!(knight.valid_moves(), vec![(1, 2), (2, 1), (-1, 2), (-2, 1), (1, -2), (2, -1), (-1, -2), (-2, -1)]);
    }

    #[test]
    fn test_bishop_moves() {
        let bishop = Bishop::new(true);
        let moves = bishop.valid_moves();
        for i in -7..8 {
            assert!(moves.contains(&(i, i)));
            assert!(moves.contains(&(-i, i)));
        }
    }

    #[test]
    fn test_rook_moves() {
        let rook = Rook::new(true);
        let moves = rook.valid_moves();
        for i in -7..8 {
            assert!(moves.contains(&(i, 0)));
            assert!(moves.contains(&(0, i)));
        }
    }

    #[test]
    fn test_queen_moves() {
        let queen = Queen::new(true);
        let moves = queen.valid_moves();
        for i in -7..8 {
            assert!(moves.contains(&(i, 0)));
            assert!(moves.contains(&(0, i)));
            assert!(moves.contains(&(i, i)));
            assert!(moves.contains(&(-i, i)));
        }
    }

    #[test]
    fn test_king_moves() {
        let king = King::new(true);
        assert_eq!(king.valid_moves(), vec![(0, 1), (1, 0), (0, -1), (-1, 0), (1, 1), (-1, -1), (1, -1), (-1, 1)]);
    }

    #[test]
    fn test_board_new() {
        let board = Board::new();
        assert_eq!(board.pieces.len(), 32);
    }
}
```