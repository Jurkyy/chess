```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::board::Board;
    use crate::chess::piece::Piece;
    use crate::chess::game::Game;
    use crate::chess::move::Move;
    use crate::chess::position::Position;
    use crate::chess::color::Color;

    #[test]
    fn test_move_piece() {
        let mut game = Game::new();
        let move = Move::new(Position::new(1, 0), Position::new(3, 0));
        assert!(game.move_piece(&move));
        assert_eq!(game.board.get_piece(Position::new(3, 0)), Some(Piece::Pawn(Color::White)));
    }

    #[test]
    fn test_is_valid_move() {
        let game = Game::new();
        let valid_move = Move::new(Position::new(1, 0), Position::new(3, 0));
        let invalid_move = Move::new(Position::new(0, 0), Position::new(3, 0));
        assert!(game.is_valid_move(&valid_move));
        assert!(!game.is_valid_move(&invalid_move));
    }

    #[test]
    fn test_move_out_of_bounds() {
        let mut game = Game::new();
        let move = Move::new(Position::new(1, 0), Position::new(8, 0));
        assert!(!game.move_piece(&move));
    }

    #[test]
    fn test_move_into_check() {
        let mut game = Game::new();
        game.board.set_piece(Position::new(4, 4), Some(Piece::Queen(Color::Black)));
        let move = Move::new(Position::new(6, 3), Position::new(5, 3));
        assert!(!game.move_piece(&move));
    }
}
```