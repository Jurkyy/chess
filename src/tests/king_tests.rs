```rust
#[cfg(test)]
mod tests {
    use super::super::king::King;
    use super::super::board::Board;
    use super::super::piece::Piece;
    use super::super::piece::Color;

    #[test]
    fn test_king_moves() {
        let mut board = Board::init();
        let king = Piece::King(King::new(Color::White, (4, 1)));

        board.set_piece(king.clone(), (4, 1));
        assert_eq!(king.get_valid_moves(&board), vec![(3, 1), (5, 1), (4, 2), (3, 2), (5, 2)]);

        board.set_piece(Piece::Pawn(Pawn::new(Color::Black, (3, 2))), (3, 2));
        assert_eq!(king.get_valid_moves(&board), vec![(3, 1), (5, 1), (4, 2), (5, 2)]);
    }

    #[test]
    fn test_king_castling() {
        let mut board = Board::init();
        let king = Piece::King(King::new(Color::White, (4, 1)));
        let rook = Piece::Rook(Rook::new(Color::White, (7, 1)));

        board.set_piece(king.clone(), (4, 1));
        board.set_piece(rook.clone(), (7, 1));
        assert_eq!(king.get_valid_moves(&board), vec![(3, 1), (5, 1), (4, 2), (3, 2), (5, 2), (6, 1)]);

        board.move_piece((4, 1), (5, 1));
        assert_eq!(king.get_valid_moves(&board), vec![(4, 1), (6, 1), (5, 2), (4, 2), (6, 2)]);
    }
}
```