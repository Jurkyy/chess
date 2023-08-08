```rust
use super::board::Board;
use super::piece::Piece;
use super::PIECES;

pub struct Queen {
    pub position: (usize, usize),
    pub color: String,
}

impl Queen {
    pub fn new(position: (usize, usize), color: String) -> Self {
        Queen { position, color }
    }

    pub fn is_valid_move(&self, board: &Board, end: (usize, usize)) -> bool {
        let dx = (end.0 as i32 - self.position.0 as i32).abs();
        let dy = (end.1 as i32 - self.position.1 as i32).abs();

        if dx == dy || dx == 0 || dy == 0 {
            return board.is_path_clear(self.position, end);
        }

        false
    }

    pub fn get_possible_moves(&self, board: &Board) -> Vec<(usize, usize)> {
        let mut moves: Vec<(usize, usize)> = Vec::new();

        for i in 0..8 {
            for j in 0..8 {
                if self.is_valid_move(board, (i, j)) {
                    moves.push((i, j));
                }
            }
        }

        moves
    }

    pub fn move_piece(&mut self, board: &mut Board, end: (usize, usize)) {
        if self.is_valid_move(board, end) {
            let piece = board.get_piece(self.position).unwrap();
            board.set_piece(end, piece);
            board.remove_piece(self.position);
            self.position = end;
        } else {
            panic!("InvalidMoveError");
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_valid_move() {
        let board = Board::new();
        let queen = Queen::new((3, 3), "White".to_string());

        assert_eq!(queen.is_valid_move(&board, (5, 5)), true);
        assert_eq!(queen.is_valid_move(&board, (1, 1)), true);
        assert_eq!(queen.is_valid_move(&board, (3, 7)), true);
        assert_eq!(queen.is_valid_move(&board, (3, 0)), true);
        assert_eq!(queen.is_valid_move(&board, (0, 3)), true);
        assert_eq!(queen.is_valid_move(&board, (7, 3)), true);
        assert_eq!(queen.is_valid_move(&board, (4, 2)), false);
    }

    #[test]
    fn test_get_possible_moves() {
        let board = Board::new();
        let queen = Queen::new((3, 3), "White".to_string());

        assert_eq!(queen.get_possible_moves(&board).len(), 27);
    }

    #[test]
    #[should_panic(expected = "InvalidMoveError")]
    fn test_move_piece_invalid() {
        let mut board = Board::new();
        let mut queen = Queen::new((3, 3), "White".to_string());

        queen.move_piece(&mut board, (4, 2));
    }
}
```