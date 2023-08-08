```rust
#[cfg(test)]
mod tests {
    use super::super::queen::Queen;
    use super::super::board::Board;
    use super::super::game::Game;
    use super::super::piece::Piece;
    use super::super::PIECES;

    #[test]
    fn test_queen_moves() {
        let mut game = Game::new();
        let queen = Queen::new(PIECES::QUEEN, 4, 4, true);
        game.board.set_piece_at(4, 4, Piece::Queen(queen));

        let moves = game.board.get_piece_at(4, 4).unwrap().get_possible_moves(&game);
        assert_eq!(moves.len(), 27); // Queen should have 27 possible moves in an empty board from position (4,4)
    }

    #[test]
    fn test_queen_capture() {
        let mut game = Game::new();
        let queen = Queen::new(PIECES::QUEEN, 4, 4, true);
        game.board.set_piece_at(4, 4, Piece::Queen(queen));

        let opponent_pawn = Pawn::new(PIECES::PAWN, 5, 5, false);
        game.board.set_piece_at(5, 5, Piece::Pawn(opponent_pawn));

        let moves = game.board.get_piece_at(4, 4).unwrap().get_possible_moves(&game);
        assert!(moves.contains(&(5, 5))); // Queen should be able to capture opponent's pawn
    }

    #[test]
    fn test_queen_invalid_move() {
        let mut game = Game::new();
        let queen = Queen::new(PIECES::QUEEN, 4, 4, true);
        game.board.set_piece_at(4, 4, Piece::Queen(queen));

        let own_pawn = Pawn::new(PIECES::PAWN, 5, 5, true);
        game.board.set_piece_at(5, 5, Piece::Pawn(own_pawn));

        assert!(!game.move_piece(4, 4, 5, 5).is_ok()); // Queen should not be able to move to a square occupied by own piece
    }
}
```