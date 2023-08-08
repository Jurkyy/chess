```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::{Chess, Player};
    use crate::piece::Piece;
    use crate::board::Board;
    use crate::pawn::Pawn;
    use crate::knight::Knight;
    use crate::bishop::Bishop;
    use crate::rook::Rook;
    use crate::queen::Queen;
    use crate::king::King;
    use crate::castling::Castling;

    #[test]
    fn test_pawn_movement() {
        let mut chess = Chess::new();
        assert_eq!(chess.move_piece((1, 0), (2, 0)), Ok(()));
        assert_eq!(chess.move_piece((1, 0), (3, 0)), Ok(()));
        assert_eq!(chess.move_piece((1, 0), (4, 0)), Err("Invalid move"));
    }

    #[test]
    fn test_knight_movement() {
        let mut chess = Chess::new();
        assert_eq!(chess.move_piece((0, 1), (2, 2)), Ok(()));
        assert_eq!(chess.move_piece((0, 1), (2, 0)), Ok(()));
        assert_eq!(chess.move_piece((0, 1), (1, 3)), Err("Invalid move"));
    }

    #[test]
    fn test_bishop_movement() {
        let mut chess = Chess::new();
        assert_eq!(chess.move_piece((0, 2), (2, 0)), Ok(()));
        assert_eq!(chess.move_piece((0, 2), (2, 4)), Ok(()));
        assert_eq!(chess.move_piece((0, 2), (1, 4)), Err("Invalid move"));
    }

    #[test]
    fn test_rook_movement() {
        let mut chess = Chess::new();
        assert_eq!(chess.move_piece((0, 0), (0, 3)), Ok(()));
        assert_eq!(chess.move_piece((0, 0), (3, 0)), Ok(()));
        assert_eq!(chess.move_piece((0, 0), (3, 3)), Err("Invalid move"));
    }

    #[test]
    fn test_queen_movement() {
        let mut chess = Chess::new();
        assert_eq!(chess.move_piece((0, 3), (0, 6)), Ok(()));
        assert_eq!(chess.move_piece((0, 3), (3, 0)), Ok(()));
        assert_eq!(chess.move_piece((0, 3), (3, 6)), Ok(()));
        assert_eq!(chess.move_piece((0, 3), (2, 5)), Err("Invalid move"));
    }

    #[test]
    fn test_king_movement() {
        let mut chess = Chess::new();
        assert_eq!(chess.move_piece((0, 4), (0, 5)), Ok(()));
        assert_eq!(chess.move_piece((0, 4), (1, 4)), Ok(()));
        assert_eq!(chess.move_piece((0, 4), (1, 5)), Ok(()));
        assert_eq!(chess.move_piece((0, 4), (2, 5)), Err("Invalid move"));
    }

    #[test]
    fn test_castling() {
        let mut chess = Chess::new();
        assert_eq!(chess.castling(Player::White, Castling::KingSide), Ok(()));
        assert_eq!(chess.castling(Player::White, Castling::QueenSide), Ok(()));
        assert_eq!(chess.castling(Player::Black, Castling::KingSide), Ok(()));
        assert_eq!(chess.castling(Player::Black, Castling::QueenSide), Ok(()));
    }
}
```