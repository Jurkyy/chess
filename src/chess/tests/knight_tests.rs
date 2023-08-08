```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::knight::Knight;
    use crate::chess::board::Board;
    use crate::chess::game::Game;
    use crate::chess::piece::Piece;
    use crate::chess::PIECES;

    #[test]
    fn test_knight_moves() {
        let mut game = Game::new();
        let knight = Knight::new(PIECES::Knight, 1, 0, 1);
        game.board.set_piece_at(0, 1, Piece::Knight(knight));
        let moves = game.get_possible_moves(0, 1);
        assert_eq!(moves.len(), 2);
        assert!(moves.contains(&(2, 0)));
        assert!(moves.contains(&(2, 2)));
    }

    #[test]
    fn test_knight_jump_over_pieces() {
        let mut game = Game::new();
        let knight = Knight::new(PIECES::Knight, 1, 0, 1);
        game.board.set_piece_at(0, 1, Piece::Knight(knight));
        game.board.set_piece_at(1, 0, Piece::Pawn(Pawn::new(PIECES::Pawn, 0, 1, 0)));
        let moves = game.get_possible_moves(0, 1);
        assert_eq!(moves.len(), 2);
        assert!(moves.contains(&(2, 0)));
        assert!(moves.contains(&(2, 2)));
    }

    #[test]
    fn test_knight_capture() {
        let mut game = Game::new();
        let knight = Knight::new(PIECES::Knight, 1, 0, 1);
        game.board.set_piece_at(0, 1, Piece::Knight(knight));
        game.board.set_piece_at(2, 0, Piece::Pawn(Pawn::new(PIECES::Pawn, 0, 2, 0)));
        let moves = game.get_possible_moves(0, 1);
        assert_eq!(moves.len(), 2);
        assert!(moves.contains(&(2, 0)));
        game.move_piece(0, 1, 2, 0);
        assert_eq!(game.board.get_piece_at(2, 0), Some(Piece::Knight(knight)));
        assert_eq!(game.board.get_piece_at(0, 1), None);
    }
}
```