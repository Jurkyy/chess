```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::board::Board;
    use crate::chess::pieces::king::King;
    use crate::chess::pieces::rook::Rook;
    use crate::chess::move::Move;
    use crate::chess::game::Game;

    #[test]
    fn test_is_valid_move_castling() {
        let mut board = Board::new();
        let king = King::new(Color::White);
        let rook = Rook::new(Color::White);
        board.set_piece_at(4, 0, Piece::King(king));
        board.set_piece_at(7, 0, Piece::Rook(rook));
        let move = Move::new((4, 0), (6, 0));
        assert!(king.is_valid_move(&move, &board));
    }

    #[test]
    fn test_is_castling_move() {
        let king = King::new(Color::White);
        let move = Move::new((4, 0), (6, 0));
        assert!(king.is_castling_move(&move));
    }

    #[test]
    fn test_perform_castling() {
        let mut game = Game::new();
        game.board.set_piece_at(4, 0, Piece::King(King::new(Color::White)));
        game.board.set_piece_at(7, 0, Piece::Rook(Rook::new(Color::White)));
        let move = Move::new((4, 0), (6, 0));
        game.make_move(&move);
        assert_eq!(game.board.get_piece_at(6, 0), Some(Piece::King(King::new(Color::White))));
        assert_eq!(game.board.get_piece_at(5, 0), Some(Piece::Rook(Rook::new(Color::White))));
    }
}
```