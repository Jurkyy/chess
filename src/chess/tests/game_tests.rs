```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::{Game, Piece, PIECES};

    #[test]
    fn test_game_initialization() {
        let game = Game::new();
        assert_eq!(game.current_player, PIECES::WHITE);
        assert_eq!(game.status, GameStatus::Ongoing);
    }

    #[test]
    fn test_valid_move() {
        let mut game = Game::new();
        let pawn = Piece::new(PIECES::PAWN, PIECES::WHITE, (1, 1));
        game.board[1][1] = Some(pawn);
        assert!(game.move_piece((1, 1), (1, 2)).is_ok());
    }

    #[test]
    fn test_invalid_move() {
        let mut game = Game::new();
        let pawn = Piece::new(PIECES::PAWN, PIECES::WHITE, (1, 1));
        game.board[1][1] = Some(pawn);
        assert!(game.move_piece((1, 1), (2, 2)).is_err());
    }

    #[test]
    fn test_check() {
        let mut game = Game::new();
        let king = Piece::new(PIECES::KING, PIECES::WHITE, (0, 0));
        let rook = Piece::new(PIECES::ROOK, PIECES::BLACK, (0, 7));
        game.board[0][0] = Some(king);
        game.board[0][7] = Some(rook);
        assert!(game.is_in_check(PIECES::WHITE));
    }

    #[test]
    fn test_checkmate() {
        let mut game = Game::new();
        let king = Piece::new(PIECES::KING, PIECES::WHITE, (0, 0));
        let rook1 = Piece::new(PIECES::ROOK, PIECES::BLACK, (0, 7));
        let rook2 = Piece::new(PIECES::ROOK, PIECES::BLACK, (1, 7));
        game.board[0][0] = Some(king);
        game.board[0][7] = Some(rook1);
        game.board[1][7] = Some(rook2);
        assert!(game.is_in_checkmate(PIECES::WHITE));
    }

    #[test]
    fn test_castling() {
        let mut game = Game::new();
        let king = Piece::new(PIECES::KING, PIECES::WHITE, (4, 0));
        let rook = Piece::new(PIECES::ROOK, PIECES::WHITE, (7, 0));
        game.board[4][0] = Some(king);
        game.board[7][0] = Some(rook);
        assert!(game.can_castle(PIECES::WHITE, CastlingSide::Kingside).is_ok());
        assert!(game.perform_castling(PIECES::WHITE, CastlingSide::Kingside).is_ok());
    }
}
```