```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::{Game, Piece, PIECES};

    #[test]
    fn test_can_castle() {
        let mut game = Game::new();
        assert_eq!(game.board.can_castle(&game.white_king, &game.white_rook_left), true);
        assert_eq!(game.board.can_castle(&game.white_king, &game.white_rook_right), true);
        assert_eq!(game.board.can_castle(&game.black_king, &game.black_rook_left), true);
        assert_eq!(game.board.can_castle(&game.black_king, &game.black_rook_right), true);
    }

    #[test]
    fn test_cannot_castle_if_king_has_moved() {
        let mut game = Game::new();
        game.move_piece(&game.white_king, (4, 2));
        assert_eq!(game.board.can_castle(&game.white_king, &game.white_rook_left), false);
        assert_eq!(game.board.can_castle(&game.white_king, &game.white_rook_right), false);
    }

    #[test]
    fn test_cannot_castle_if_rook_has_moved() {
        let mut game = Game::new();
        game.move_piece(&game.white_rook_left, (0, 2));
        assert_eq!(game.board.can_castle(&game.white_king, &game.white_rook_left), false);
        game.move_piece(&game.white_rook_right, (7, 2));
        assert_eq!(game.board.can_castle(&game.white_king, &game.white_rook_right), false);
    }

    #[test]
    fn test_cannot_castle_if_pieces_in_between() {
        let mut game = Game::new();
        game.board.pieces[0][3] = Some(Piece::new(PIECES::Bishop, true, (0, 3)));
        assert_eq!(game.board.can_castle(&game.white_king, &game.white_rook_left), false);
        game.board.pieces[7][5] = Some(Piece::new(PIECES::Bishop, true, (7, 5)));
        assert_eq!(game.board.can_castle(&game.white_king, &game.white_rook_right), false);
    }

    #[test]
    fn test_perform_castling() {
        let mut game = Game::new();
        game.board.perform_castling(&game.white_king, &game.white_rook_left);
        assert_eq!(game.white_king.position, (2, 0));
        assert_eq!(game.white_rook_left.position, (3, 0));
        game.board.perform_castling(&game.black_king, &game.black_rook_right);
        assert_eq!(game.black_king.position, (6, 7));
        assert_eq!(game.black_rook_right.position, (5, 7));
    }
}
```