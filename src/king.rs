```rust
use crate::piece::Piece;
use crate::board::Board;
use crate::castling::can_castle;

#[derive(Clone, Debug)]
pub struct King {
    pub piece: Piece,
    pub has_moved: bool,
}

impl King {
    pub fn new(piece: Piece) -> Self {
        King {
            piece,
            has_moved: false,
        }
    }

    pub fn valid_move(&self, board: &Board, end: (usize, usize)) -> bool {
        let start = self.piece.position;
        let dx = (end.0 as i32 - start.0 as i32).abs();
        let dy = (end.1 as i32 - start.1 as i32).abs();

        dx <= 1 && dy <= 1 || can_castle(board, &self.piece, end)
    }

    pub fn move_piece(&mut self, end: (usize, usize)) {
        self.piece.position = end;
        self.has_moved = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::Color;

    #[test]
    fn test_valid_move() {
        let king = King::new(Piece::new((4, 4), Color::White, "King"));
        let board = Board::new();

        assert!(king.valid_move(&board, (5, 5)));
        assert!(king.valid_move(&board, (4, 5)));
        assert!(king.valid_move(&board, (3, 3)));
        assert!(!king.valid_move(&board, (6, 6)));
    }

    #[test]
    fn test_move_piece() {
        let mut king = King::new(Piece::new((4, 4), Color::White, "King"));

        king.move_piece((5, 5));
        assert_eq!(king.piece.position, (5, 5));
        assert!(king.has_moved);
    }
}
```