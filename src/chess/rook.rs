```rust
use crate::chess::board::Board;
use crate::chess::piece::Piece;
use crate::chess::game::GameStatus;
use crate::chess::castling::can_castle;

pub struct Rook {
    position: (usize, usize),
    color: String,
    has_moved: bool,
}

impl Rook {
    pub fn new(position: (usize, usize), color: String) -> Rook {
        Rook {
            position,
            color,
            has_moved: false,
        }
    }

    pub fn move_piece(&mut self, new_position: (usize, usize), board: &mut Board) -> Result<(), &'static str> {
        if self.is_valid_move(new_position, board) {
            self.position = new_position;
            self.has_moved = true;
            Ok(())
        } else {
            Err("InvalidMoveError")
        }
    }

    pub fn is_valid_move(&self, new_position: (usize, usize), board: &Board) -> bool {
        let (new_x, new_y) = new_position;
        let (old_x, old_y) = self.position;

        if new_x == old_x || new_y == old_y {
            if board.is_path_clear(self.position, new_position) {
                return true;
            }
        }
        false
    }

    pub fn get_possible_moves(&self, board: &Board) -> Vec<(usize, usize)> {
        let mut moves: Vec<(usize, usize)> = Vec::new();
        let directions = [(0, 1), (1, 0), (0, -1), (-1, 0)];

        for direction in directions.iter() {
            let mut x = (self.position.0 as i32) + direction.0;
            let mut y = (self.position.1 as i32) + direction.1;

            while x >= 0 && x < 8 && y >= 0 && y < 8 {
                if let Some(piece) = board.get_piece((x as usize, y as usize)) {
                    if piece.color != self.color {
                        moves.push((x as usize, y as usize));
                    }
                    break;
                } else {
                    moves.push((x as usize, y as usize));
                }
                x += direction.0;
                y += direction.1;
            }
        }
        moves
    }

    pub fn can_castle(&self, board: &Board) -> bool {
        if self.has_moved {
            return false;
        }
        can_castle(self.position, self.color.clone(), board)
    }
}
```