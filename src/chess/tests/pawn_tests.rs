```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::pawn::Pawn;
    use crate::chess::board::Board;
    use crate::chess::game::Game;
    use crate::chess::PIECES;

    #[test]
    fn test_pawn_move_forward() {
        let mut game = Game::new();
        let pawn = Pawn::new(PIECES::Pawn, "white", (1, 1));
        game.board.set_piece_at((1, 1), Some(Box::new(pawn)));
        assert!(game.move_piece((1, 1), (1, 2)).is_ok());
    }

    #[test]
    fn test_pawn_capture_diagonal() {
        let mut game = Game::new();
        let pawn_white = Pawn::new(PIECES::Pawn, "white", (1, 1));
        let pawn_black = Pawn::new(PIECES::Pawn, "black", (2, 2));
        game.board.set_piece_at((1, 1), Some(Box::new(pawn_white)));
        game.board.set_piece_at((2, 2), Some(Box::new(pawn_black)));
        assert!(game.move_piece((1, 1), (2, 2)).is_ok());
    }

    #[test]
    fn test_pawn_first_move_two_squares() {
        let mut game = Game::new();
        let pawn = Pawn::new(PIECES::Pawn, "white", (1, 1));
        game.board.set_piece_at((1, 1), Some(Box::new(pawn)));
        assert!(game.move_piece((1, 1), (1, 3)).is_ok());
    }

    #[test]
    fn test_pawn_en_passant() {
        let mut game = Game::new();
        let pawn_white = Pawn::new(PIECES::Pawn, "white", (4, 4));
        let pawn_black = Pawn::new(PIECES::Pawn, "black", (5, 6));
        game.board.set_piece_at((4, 4), Some(Box::new(pawn_white)));
        game.board.set_piece_at((5, 6), Some(Box::new(pawn_black)));
        game.move_piece((5, 6), (5, 4)).unwrap();
        assert!(game.move_piece((4, 4), (5, 5)).is_ok());
    }

    #[test]
    fn test_pawn_promotion() {
        let mut game = Game::new();
        let pawn = Pawn::new(PIECES::Pawn, "white", (1, 7));
        game.board.set_piece_at((1, 7), Some(Box::new(pawn)));
        assert!(game.move_piece((1, 7), (1, 8)).is_ok());
        assert_eq!(game.board.get_piece_at((1, 8)).unwrap().get_type(), PIECES::Queen);
    }
}
```