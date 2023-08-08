```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::board::Board;
    use crate::chess::pieces::{King, Rook};
    use crate::chess::game::Game;
    use crate::chess::move::Move;

    #[test]
    fn test_castling_move() {
        let mut game = Game::new();
        let king = King::new(Color::White);
        let rook = Rook::new(Color::White);

        game.board.set_piece_at(4, 0, Piece::King(king));
        game.board.set_piece_at(7, 0, Piece::Rook(rook));

        let castling_move = Move::new((4, 0), (6, 0));
        assert!(game.is_valid_move(&castling_move));
        game.make_move(&castling_move);

        match game.board.get_piece_at(6, 0) {
            Some(Piece::King(_)) => assert!(true),
            _ => assert!(false),
        }

        match game.board.get_piece_at(5, 0) {
            Some(Piece::Rook(_)) => assert!(true),
            _ => assert!(false),
        }
    }

    #[test]
    fn test_castling_in_check() {
        let mut game = Game::new();
        let king = King::new(Color::White);
        let rook = Rook::new(Color::White);

        game.board.set_piece_at(4, 0, Piece::King(king));
        game.board.set_piece_at(7, 0, Piece::Rook(rook));
        game.board.set_piece_at(4, 7, Piece::Rook(Rook::new(Color::Black)));

        let castling_move = Move::new((4, 0), (6, 0));
        assert!(!game.is_valid_move(&castling_move));
    }
}
```