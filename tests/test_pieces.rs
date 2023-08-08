#[cfg(test)]
mod test_pieces {
    use crate::*;

    #[test]
    fn test_pawn_move() {
        let mut board = Board::new();
        let pawn = Piece::new(PieceType::Pawn, Color::White, (1, 1));
        board.place_piece(pawn);
        assert!(board.move_piece((1, 1), (1, 2)));
    }

    #[test]
    fn test_pawn_capture() {
        let mut board = Board::new();
        let pawn = Piece::new(PieceType::Pawn, Color::White, (1, 1));
        let enemy_pawn = Piece::new(PieceType::Pawn, Color::Black, (2, 2));
        board.place_piece(pawn);
        board.place_piece(enemy_pawn);
        assert!(board.move_piece((1, 1), (2, 2)));
    }

    #[test]
    fn test_pawn_promotion() {
        let mut board = Board::new();
        let pawn = Piece::new(PieceType::Pawn, Color::White, (1, 7));
        board.place_piece(pawn);
        assert!(board.move_piece((1, 7), (1, 8)));
        assert_eq!(
            board.get_piece((1, 8)).unwrap().piece_type,
            PieceType::Queen
        );
    }

    #[test]
    fn test_knight_move() {
        let mut board = Board::new();
        let knight = Piece::new(PieceType::Knight, Color::White, (1, 1));
        board.place_piece(knight);
        assert!(board.move_piece((1, 1), (3, 2)));
    }

    #[test]
    fn test_bishop_move() {
        let mut board = Board::new();
        let bishop = Piece::new(PieceType::Bishop, Color::White, (1, 1));
        board.place_piece(bishop);
        assert!(board.move_piece((1, 1), (3, 3)));
    }

    #[test]
    fn test_rook_move() {
        let mut board = Board::new();
        let rook = Piece::new(PieceType::Rook, Color::White, (1, 1));
        board.place_piece(rook);
        assert!(board.move_piece((1, 1), (1, 3)));
    }

    #[test]
    fn test_queen_move() {
        let mut board = Board::new();
        let queen = Piece::new(PieceType::Queen, Color::White, (1, 1));
        board.place_piece(queen);
        assert!(board.move_piece((1, 1), (1, 3)));
        assert!(board.move_piece((1, 3), (3, 3)));
    }

    #[test]
    fn test_king_move() {
        let mut board = Board::new();
        let king = Piece::new(PieceType::King, Color::White, (1, 1));
        board.place_piece(king);
        assert!(board.move_piece((1, 1), (1, 2)));
    }

    #[test]
    fn test_castling() {
        let mut board = Board::new();
        let king = Piece::new(PieceType::King, Color::White, (5, 1));
        let rook = Piece::new(PieceType::Rook, Color::White, (8, 1));
        board.place_piece(king);
        board.place_piece(rook);
        assert!(board.castling(Color::White, CastlingSide::Kingside));
    }
}
