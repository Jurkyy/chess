```rust
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_king_color() {
        let king = King::new(Color::White);
        assert_eq!(king.color, Color::White);
    }

    #[test]
    fn test_rook_color() {
        let rook = Rook::new(Color::Black);
        assert_eq!(rook.color, Color::Black);
    }

    #[test]
    fn test_pawn_en_passant() {
        let mut board = Board::new();
        let pawn = Pawn::new(Color::White);
        board.set_piece_at(4, 4, Some(Box::new(pawn)));
        let move = Move::new(4, 4, 5, 5);
        assert!(board.is_valid_move(&move));
    }

    #[test]
    fn test_rook_move() {
        let mut board = Board::new();
        let rook = Rook::new(Color::White);
        board.set_piece_at(0, 0, Some(Box::new(rook)));
        let move = Move::new(0, 0, 0, 7);
        assert!(board.is_valid_move(&move));
    }

    #[test]
    fn test_bishop_color() {
        let bishop = Bishop::new(Color::White);
        assert_eq!(bishop.color, Color::White);
    }

    #[test]
    fn test_knight_move() {
        let mut board = Board::new();
        let knight = Knight::new(Color::White);
        board.set_piece_at(0, 1, Some(Box::new(knight)));
        let move = Move::new(0, 1, 2, 2);
        assert!(board.is_valid_move(&move));
    }

    #[test]
    fn test_pawn_two_tile_move() {
        let mut board = Board::new();
        let pawn = Pawn::new(Color::White);
        board.set_piece_at(1, 1, Some(Box::new(pawn)));
        let move = Move::new(1, 1, 1, 3);
        assert!(board.is_valid_move(&move));
    }

    #[test]
    fn test_castling() {
        let mut game = Game::new();
        let move = Move::new(4, 0, 6, 0);
        assert!(game.is_valid_move(&move));
    }
}
```