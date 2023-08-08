```rust
#[cfg(test)]
mod tests {
    use super::*;
    use crate::chess::{Piece, Board, PIECES};

    #[test]
    fn test_board_initialization() {
        let board = Board::new();
        assert_eq!(board.size, BOARD_SIZE);
        assert_eq!(board.pieces.len(), BOARD_SIZE * BOARD_SIZE);
    }

    #[test]
    fn test_board_setup() {
        let board = Board::new();
        for (i, piece) in board.pieces.iter().enumerate() {
            let expected_piece = match i {
                0 | 7 => Some(Piece::new(PIECES::Rook, "black")),
                1 | 6 => Some(Piece::new(PIECES::Knight, "black")),
                2 | 5 => Some(Piece::new(PIECES::Bishop, "black")),
                3 => Some(Piece::new(PIECES::Queen, "black")),
                4 => Some(Piece::new(PIECES::King, "black")),
                8..=15 => Some(Piece::new(PIECES::Pawn, "black")),
                48..=55 => Some(Piece::new(PIECES::Pawn, "white")),
                56 | 63 => Some(Piece::new(PIECES::Rook, "white")),
                57 | 62 => Some(Piece::new(PIECES::Knight, "white")),
                58 | 61 => Some(Piece::new(PIECES::Bishop, "white")),
                59 => Some(Piece::new(PIECES::Queen, "white")),
                60 => Some(Piece::new(PIECES::King, "white")),
                _ => None,
            };
            assert_eq!(piece, &expected_piece);
        }
    }

    #[test]
    fn test_move_piece() {
        let mut board = Board::new();
        let pawn = Piece::new(PIECES::Pawn, "white");
        let initial_position = (6, 4);
        let target_position = (4, 4);
        board.pieces[initial_position.0][initial_position.1] = Some(pawn);
        board.move_piece(initial_position, target_position).unwrap();
        assert_eq!(board.pieces[target_position.0][target_position.1], Some(pawn));
        assert_eq!(board.pieces[initial_position.0][initial_position.1], None);
    }

    #[test]
    #[should_panic(expected = "InvalidMoveError")]
    fn test_invalid_move() {
        let mut board = Board::new();
        let pawn = Piece::new(PIECES::Pawn, "white");
        let initial_position = (6, 4);
        let target_position = (4, 5);
        board.pieces[initial_position.0][initial_position.1] = Some(pawn);
        board.move_piece(initial_position, target_position).unwrap();
    }
}
```