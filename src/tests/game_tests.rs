```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::game::Game;
    use crate::chess::color::Color;
    use crate::chess::piece::Piece;
    use crate::chess::board::Board;
    use crate::chess::move::Move;
    use crate::chess::castling::Castling;

    #[test]
    fn test_game_creation() {
        let game = Game::new();
        assert_eq!(game.turn, Color::White);
        assert_eq!(game.board, Board::new());
    }

    #[test]
    fn test_valid_move() {
        let mut game = Game::new();
        let move = Move::new((0, 1), (0, 3));
        assert!(game.is_valid_move(&move));
    }

    #[test]
    fn test_invalid_move() {
        let mut game = Game::new();
        let move = Move::new((0, 0), (0, 3));
        assert!(!game.is_valid_move(&move));
    }

    #[test]
    fn test_castling() {
        let mut game = Game::new();
        let castling = Castling::new((0, 4), (0, 7));
        assert!(game.is_valid_castling(&castling));
    }

    #[test]
    fn test_invalid_castling() {
        let mut game = Game::new();
        let castling = Castling::new((0, 4), (0, 6));
        assert!(!game.is_valid_castling(&castling));
    }
}
```