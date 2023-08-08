```rust
use crate::board::Board;
use crate::pieces::Piece;

pub struct King {
    pub position: (usize, usize),
    pub color: String,
    pub has_moved: bool,
}

impl King {
    pub fn new(position: (usize, usize), color: String) -> Self {
        King {
            position,
            color,
            has_moved: false,
        }
    }

    pub fn valid_move(&self, board: &Board, end: (usize, usize)) -> bool {
        let dx = (self.position.0 as i32 - end.0 as i32).abs();
        let dy = (self.position.1 as i32 - end.1 as i32).abs();

        dx <= 1 && dy <= 1
    }

    pub fn can_castle(&self, board: &Board, rook: &Piece) -> bool {
        if self.has_moved || rook.has_moved() {
            return false;
        }

        let (start, end) = if self.position.0 < rook.position().0 {
            (self.position.0, rook.position().0)
        } else {
            (rook.position().0, self.position.0)
        };

        for i in start + 1..end {
            if board.piece_at((i, self.position.1)).is_some() {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_move() {
        let board = Board::new();
        let king = King::new((4, 4), "White".to_string());

        assert!(king.valid_move(&board, (5, 5)));
        assert!(king.valid_move(&board, (4, 5)));
        assert!(king.valid_move(&board, (3, 3)));
        assert!(!king.valid_move(&board, (6, 6)));
    }

    #[test]
    fn test_can_castle() {
        let mut board = Board::new();
        let king = King::new((4, 0), "White".to_string());
        let rook = Piece::Rook(Rook::new((7, 0), "White".to_string()));

        board.set_piece(king.position, Some(Piece::King(king)));
        board.set_piece(rook.position(), Some(rook.clone()));

        assert!(king.can_castle(&board, &rook));
        assert!(rook.can_castle(&board, &Piece::King(king)));
    }
}
```