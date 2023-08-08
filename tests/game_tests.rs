```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::game::Game;
    use crate::chess::board::Board;
    use crate::chess::piece::Piece;
    use crate::chess::move::Move;
    use crate::chess::castling::Castling;
    use crate::chess::piece::Color;

    #[test]
    fn test_new() {
        let game = Game::new();
        assert_eq!(game.board, Board::new());
        assert_eq!(game.current_color, Color::White);
    }

    #[test]
    fn test_move_piece() {
        let mut game = Game::new();
        let move = Move::new(Position::new(1, 0), Position::new(3, 0));
        assert!(game.move_piece(&move));
        assert_eq!(game.board.get_piece_at(&move.to), Some(Piece::new(Color::White, PieceType::Pawn)));
    }

    #[test]
    fn test_is_valid_move() {
        let game = Game::new();
        let move = Move::new(Position::new(1, 0), Position::new(3, 0));
        assert!(game.is_valid_move(&move));
    }

    #[test]
    fn test_can_castle() {
        let mut game = Game::new();
        game.board.clear();
        game.board.set_piece_at(&Position::new(0, 4), Some(Piece::new(Color::White, PieceType::King)));
        game.board.set_piece_at(&Position::new(0, 7), Some(Piece::new(Color::White, PieceType::Rook)));
        assert!(game.can_castle(&Castling::new(Color::White, CastlingSide::KingSide)));
    }
}
```