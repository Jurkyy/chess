```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::board::Board;
    use crate::chess::pieces::{King, Rook};
    use crate::chess::move::Move;

    #[test]
    fn test_is_castling_move() {
        let board = Board::new();
        let king = King::new(Color::White, Position::new(4, 0));
        let rook = Rook::new(Color::White, Position::new(7, 0));
        let move = Move::new(Position::new(4, 0), Position::new(6, 0));

        assert_eq!(is_castling_move(&board, &king, &move), true);
    }

    #[test]
    fn test_perform_castling() {
        let mut board = Board::new();
        let king = King::new(Color::White, Position::new(4, 0));
        let rook = Rook::new(Color::White, Position::new(7, 0));
        let move = Move::new(Position::new(4, 0), Position::new(6, 0));

        perform_castling(&mut board, &king, &rook, &move);

        assert_eq!(board.get_piece(Position::new(6, 0)), Some(Piece::King(king)));
        assert_eq!(board.get_piece(Position::new(5, 0)), Some(Piece::Rook(rook)));
    }
}
```