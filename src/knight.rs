```rust
use crate::piece::Piece;
use crate::board::Board;

pub struct Knight {
    piece: Piece,
}

impl Knight {
    pub fn new(piece: Piece) -> Self {
        Knight { piece }
    }

    pub fn valid_move(&self, board: &Board, end_x: usize, end_y: usize) -> bool {
        let start_x = self.piece.x;
        let start_y = self.piece.y;

        let dx = (start_x as i32 - end_x as i32).abs();
        let dy = (start_y as i32 - end_y as i32).abs();

        (dx == 2 && dy == 1) || (dx == 1 && dy == 2)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::piece::Color;

    #[test]
    fn test_valid_move() {
        let board = Board::new();
        let knight = Knight::new(Piece::new(4, 4, Color::White, "Knight"));

        assert!(knight.valid_move(&board, 6, 5));
        assert!(knight.valid_move(&board, 5, 6));
        assert!(!knight.valid_move(&board, 4, 4));
        assert!(!knight.valid_move(&board, 0, 0));
    }
}
```