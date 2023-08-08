```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;
    use crate::piece::{Piece, PieceType, Color};
    use crate::castling::can_castle;

    #[test]
    fn test_castling_conditions() {
        let mut board = Board::init_board();
        let white_king = Piece::new(PieceType::King, Color::White, (4, 0));
        let white_rook = Piece::new(PieceType::Rook, Color::White, (7, 0));
        board.set_piece(white_king);
        board.set_piece(white_rook);

        assert_eq!(can_castle(&board, &white_king, &white_rook), false);

        board.move_piece((4, 0), (4, 2));
        board.move_piece((7, 0), (7, 2));

        assert_eq!(can_castle(&board, &white_king, &white_rook), true);
    }

    #[test]
    fn test_castling_move() {
        let mut board = Board::init_board();
        let white_king = Piece::new(PieceType::King, Color::White, (4, 0));
        let white_rook = Piece::new(PieceType::Rook, Color::White, (7, 0));
        board.set_piece(white_king);
        board.set_piece(white_rook);

        board.move_piece((4, 0), (4, 2));
        board.move_piece((7, 0), (7, 2));

        assert_eq!(board.get_piece((4, 2)).unwrap().piece_type, PieceType::King);
        assert_eq!(board.get_piece((7, 2)).unwrap().piece_type, PieceType::Rook);
    }
}
```