#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::Board;
    use crate::pieces::{Bishop, King, Knight, Pawn, Piece, Queen, Rook};

    #[test]
    fn test_board_new() {
        let board = Board::new();
        assert_eq!(board.is_empty(), false);
    }

    #[test]
    fn test_pawn_movement() {
        let mut board = Board::new();
        let pawn = Pawn::new(Color::White, Position::new(1, 1));
        board.set_piece(pawn);
        assert_eq!(
            board.can_move(&Position::new(1, 1), &Position::new(1, 2)),
            true
        );
    }

    #[test]
    fn test_knight_movement() {
        let mut board = Board::new();
        let knight = Knight::new(Color::White, Position::new(1, 1));
        board.set_piece(knight);
        assert_eq!(
            board.can_move(&Position::new(1, 1), &Position::new(2, 3)),
            true
        );
    }

    #[test]
    fn test_bishop_movement() {
        let mut board = Board::new();
        let bishop = Bishop::new(Color::White, Position::new(1, 1));
        board.set_piece(bishop);
        assert_eq!(
            board.can_move(&Position::new(1, 1), &Position::new(3, 3)),
            true
        );
    }

    #[test]
    fn test_rook_movement() {
        let mut board = Board::new();
        let rook = Rook::new(Color::White, Position::new(1, 1));
        board.set_piece(rook);
        assert_eq!(
            board.can_move(&Position::new(1, 1), &Position::new(1, 3)),
            true
        );
    }

    #[test]
    fn test_queen_movement() {
        let mut board = Board::new();
        let queen = Queen::new(Color::White, Position::new(1, 1));
        board.set_piece(queen);
        assert_eq!(
            board.can_move(&Position::new(1, 1), &Position::new(3, 3)),
            true
        );
        assert_eq!(
            board.can_move(&Position::new(1, 1), &Position::new(1, 3)),
            true
        );
    }

    #[test]
    fn test_king_movement() {
        let mut board = Board::new();
        let king = King::new(Color::White, Position::new(1, 1));
        board.set_piece(king);
        assert_eq!(
            board.can_move(&Position::new(1, 1), &Position::new(1, 2)),
            true
        );
    }

    #[test]
    fn test_castling() {
        let mut board = Board::new();
        let king = King::new(Color::White, Position::new(1, 1));
        let rook = Rook::new(Color::White, Position::new(1, 8));
        board.set_piece(king);
        board.set_piece(rook);
        assert_eq!(board.can_castle(Color::White), true);
    }

    #[test]
    fn test_pawn_promotion() {
        let mut board = Board::new();
        let pawn = Pawn::new(Color::White, Position::new(1, 7));
        board.set_piece(pawn);
        board.move_piece(&Position::new(1, 7), &Position::new(1, 8));
        assert_eq!(
            board.get_piece(&Position::new(1, 8)).unwrap().get_type(),
            PieceType::Queen
        );
    }
}
