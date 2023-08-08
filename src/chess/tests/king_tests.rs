```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::king::King;
    use crate::chess::board::Board;
    use crate::chess::game::Game;
    use crate::chess::piece::Piece;
    use crate::chess::PIECES;

    #[test]
    fn test_king_moves() {
        let mut game = Game::new();
        let king = King::new(PIECES::King, 4, 4, true);
        game.board.set_piece_at(4, 4, Piece::King(king));
        let moves = game.board.get_piece_at(4, 4).unwrap().get_possible_moves(&game.board);
        assert_eq!(moves.len(), 8);
    }

    #[test]
    fn test_king_in_check() {
        let mut game = Game::new();
        let king = King::new(PIECES::King, 4, 4, true);
        game.board.set_piece_at(4, 4, Piece::King(king));
        game.board.set_piece_at(5, 5, Piece::Queen(Queen::new(PIECES::Queen, 5, 5, false)));
        assert!(game.is_in_check(true));
    }

    #[test]
    fn test_king_in_checkmate() {
        let mut game = Game::new();
        let king = King::new(PIECES::King, 4, 4, true);
        game.board.set_piece_at(4, 4, Piece::King(king));
        game.board.set_piece_at(5, 5, Piece::Queen(Queen::new(PIECES::Queen, 5, 5, false)));
        game.board.set_piece_at(3, 5, Piece::Rook(Rook::new(PIECES::Rook, 3, 5, false)));
        assert!(game.is_in_checkmate(true));
    }

    #[test]
    fn test_king_castling() {
        let mut game = Game::new();
        let king = King::new(PIECES::King, 4, 0, true);
        let rook = Rook::new(PIECES::Rook, 7, 0, true);
        game.board.set_piece_at(4, 0, Piece::King(king));
        game.board.set_piece_at(7, 0, Piece::Rook(rook));
        assert!(game.can_castle(true, true));
        game.perform_castling(true, true);
        assert_eq!(game.board.get_piece_at(6, 0).unwrap().get_piece_type(), PIECES::King);
        assert_eq!(game.board.get_piece_at(5, 0).unwrap().get_piece_type(), PIECES::Rook);
    }
}
```