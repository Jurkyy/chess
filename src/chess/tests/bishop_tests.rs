```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::bishop::Bishop;
    use crate::chess::board::Board;
    use crate::chess::game::Game;
    use crate::chess::piece::Piece;
    use crate::chess::PIECES;

    #[test]
    fn test_bishop_valid_moves() {
        let mut game = Game::new();
        let bishop = Piece::new(PIECES::Bishop, true, (3, 3));
        game.board.set_piece(bishop);
        let valid_moves = bishop.get_possible_moves(&game.board);
        assert_eq!(valid_moves.len(), 13);
    }

    #[test]
    fn test_bishop_invalid_moves() {
        let mut game = Game::new();
        let bishop = Piece::new(PIECES::Bishop, true, (3, 3));
        game.board.set_piece(bishop);
        assert!(!bishop.is_valid_move((3, 3), (4, 3), &game.board));
    }

    #[test]
    fn test_bishop_capture() {
        let mut game = Game::new();
        let bishop = Piece::new(PIECES::Bishop, true, (3, 3));
        let enemy_pawn = Piece::new(PIECES::Pawn, false, (5, 5));
        game.board.set_piece(bishop);
        game.board.set_piece(enemy_pawn);
        assert!(bishop.is_valid_move((3, 3), (5, 5), &game.board));
    }

    #[test]
    fn test_bishop_blocked() {
        let mut game = Game::new();
        let bishop = Piece::new(PIECES::Bishop, true, (3, 3));
        let blocking_pawn = Piece::new(PIECES::Pawn, true, (4, 4));
        game.board.set_piece(bishop);
        game.board.set_piece(blocking_pawn);
        assert!(!bishop.is_valid_move((3, 3), (5, 5), &game.board));
    }
}
```