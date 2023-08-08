```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::color::Color;
    use crate::chess::piece::Piece;
    use crate::chess::board::Board;
    use crate::chess::move::Move;
    use crate::chess::game::Game;

    #[test]
    fn test_valid_move() {
        let mut game = Game::new();
        let move = Move::new(1, 2, 3, 4);
        assert!(game.is_valid_move(&move));
    }

    #[test]
    fn test_invalid_move() {
        let mut game = Game::new();
        let move = Move::new(1, 2, 8, 9);
        assert!(!game.is_valid_move(&move));
    }

    #[test]
    fn test_castling_move() {
        let mut game = Game::new();
        let move = Move::new(4, 0, 6, 0);
        assert!(game.is_valid_move(&move));
        game.make_move(&move);
        assert_eq!(game.board[6][0], Some(Piece::King(Color::White)));
        assert_eq!(game.board[5][0], Some(Piece::Rook(Color::White)));
    }

    #[test]
    fn test_en_passant_move() {
        let mut game = Game::new();
        game.make_move(&Move::new(1, 4, 3, 4));
        game.make_move(&Move::new(6, 5, 4, 5));
        let move = Move::new(3, 4, 4, 5);
        assert!(game.is_valid_move(&move));
        game.make_move(&move);
        assert_eq!(game.board[4][5], Some(Piece::Pawn(Color::White)));
        assert_eq!(game.board[4][4], None);
    }

    #[test]
    fn test_two_tile_move() {
        let mut game = Game::new();
        let move = Move::new(1, 3, 3, 3);
        assert!(game.is_valid_move(&move));
        game.make_move(&move);
        assert_eq!(game.board[3][3], Some(Piece::Pawn(Color::White)));
        assert_eq!(game.board[1][3], None);
    }
}
```