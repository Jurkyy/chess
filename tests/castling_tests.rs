```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::{Board, Game, Piece, Position, Color, Castling};

    #[test]
    fn test_can_castle() {
        let mut game = Game::new();
        assert_eq!(game.can_castle(&Castling::new(Color::White, true)), false);
        assert_eq!(game.can_castle(&Castling::new(Color::Black, true)), false);

        game.board.set_piece_at(Position::new(0, 4), Some(Piece::King(Color::White)));
        game.board.set_piece_at(Position::new(0, 0), Some(Piece::Rook(Color::White)));
        game.board.set_piece_at(Position::new(7, 4), Some(Piece::King(Color::Black)));
        game.board.set_piece_at(Position::new(7, 0), Some(Piece::Rook(Color::Black)));

        assert_eq!(game.can_castle(&Castling::new(Color::White, true)), true);
        assert_eq!(game.can_castle(&Castling::new(Color::Black, true)), true);
    }

    #[test]
    fn test_castle() {
        let mut game = Game::new();
        game.board.set_piece_at(Position::new(0, 4), Some(Piece::King(Color::White)));
        game.board.set_piece_at(Position::new(0, 0), Some(Piece::Rook(Color::White)));
        game.board.set_piece_at(Position::new(7, 4), Some(Piece::King(Color::Black)));
        game.board.set_piece_at(Position::new(7, 0), Some(Piece::Rook(Color::Black)));

        game.castle(&Castling::new(Color::White, true));
        assert_eq!(game.board.get_piece_at(&Position::new(0, 2)), Some(Piece::King(Color::White)));
        assert_eq!(game.board.get_piece_at(&Position::new(0, 3)), Some(Piece::Rook(Color::White)));

        game.castle(&Castling::new(Color::Black, true));
        assert_eq!(game.board.get_piece_at(&Position::new(7, 2)), Some(Piece::King(Color::Black)));
        assert_eq!(game.board.get_piece_at(&Position::new(7, 3)), Some(Piece::Rook(Color::Black)));
    }
}
```