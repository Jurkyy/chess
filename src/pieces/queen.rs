```rust
use crate::board::Board;
use crate::pieces::Piece;

pub struct Queen {
    position: (usize, usize),
    color: String,
}

impl Queen {
    pub fn new(position: (usize, usize), color: String) -> Self {
        Queen { position, color }
    }

    pub fn move_piece(&mut self, new_position: (usize, usize), board: &mut Board) {
        let valid_moves = self.valid_moves(board);
        if valid_moves.contains(&new_position) {
            self.position = new_position;
        } else {
            panic!("Invalid move for Queen");
        }
    }

    pub fn valid_moves(&self, board: &Board) -> Vec<(usize, usize)> {
        let mut moves: Vec<(usize, usize)> = Vec::new();

        // Horizontal and vertical moves
        for i in 0..8 {
            moves.push((self.position.0, i));
            moves.push((i, self.position.1));
        }

        // Diagonal moves
        let directions = [(-1, -1), (-1, 1), (1, -1), (1, 1)];
        for direction in directions.iter() {
            let mut x = self.position.0 as i32;
            let mut y = self.position.1 as i32;
            while let Some(piece) = board.get_piece((x as usize, y as usize)) {
                if piece.color != self.color {
                    moves.push((x as usize, y as usize));
                }
                x += direction.0;
                y += direction.1;
            }
        }

        moves
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queen_moves() {
        let mut board = Board::new();
        let queen = Queen::new((3, 3), "White".to_string());
        board.add_piece(Box::new(queen));
        let queen = board.get_piece((3, 3)).unwrap();
        let valid_moves = queen.valid_moves(&board);
        assert_eq!(valid_moves.len(), 27);
    }
}
```