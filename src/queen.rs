```rust
use crate::piece::Piece;
use crate::board::Board;

pub struct Queen {
    piece: Piece,
}

impl Queen {
    pub fn new(piece: Piece) -> Self {
        Queen { piece }
    }

    pub fn valid_move(&self, board: &Board, end_x: usize, end_y: usize) -> bool {
        let start_x = self.piece.x;
        let start_y = self.piece.y;

        // Check if the move is in a straight line or diagonal
        if start_x == end_x || start_y == end_y || (start_x as i32 - end_x as i32).abs() == (start_y as i32 - end_y as i32).abs() {
            // Check if the path is clear
            if board.is_path_clear(start_x, start_y, end_x, end_y) {
                return true;
            }
        }
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::{Color, Type};

    #[test]
    fn test_valid_move() {
        let board = Board::new();
        let queen = Queen::new(Piece::new(4, 4, Color::White, Type::Queen));

        assert_eq!(queen.valid_move(&board, 4, 6), true);
        assert_eq!(queen.valid_move(&board, 6, 6), true);
        assert_eq!(queen.valid_move(&board, 2, 2), true);
        assert_eq!(queen.valid_move(&board, 4, 2), false);
    }
}
```