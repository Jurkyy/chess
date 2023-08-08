```rust
use crate::chess::board::Board;
use crate::chess::piece::Piece;
use crate::chess::game::Game;

pub struct Pawn {
    position: (usize, usize),
    color: String,
    first_move: bool,
}

impl Pawn {
    pub fn new(position: (usize, usize), color: String) -> Self {
        Pawn {
            position,
            color,
            first_move: true,
        }
    }

    pub fn move_piece(&mut self, new_position: (usize, usize), board: &mut Board) -> Result<(), &'static str> {
        if self.is_valid_move(new_position, board) {
            self.position = new_position;
            self.first_move = false;
            Ok(())
        } else {
            Err("InvalidMoveError")
        }
    }

    pub fn is_valid_move(&self, new_position: (usize, usize), board: &Board) -> bool {
        let (x, y) = self.position;
        let (new_x, new_y) = new_position;

        if self.color == "white" {
            if self.first_move && new_x == x + 2 && new_y == y {
                return true;
            }
            if new_x == x + 1 && new_y == y {
                return true;
            }
            if new_x == x + 1 && (new_y == y + 1 || new_y == y - 1) {
                if let Some(piece) = board.get_piece(new_position) {
                    if piece.color == "black" {
                        return true;
                    }
                }
            }
        } else {
            if self.first_move && new_x == x - 2 && new_y == y {
                return true;
            }
            if new_x == x - 1 && new_y == y {
                return true;
            }
            if new_x == x - 1 && (new_y == y + 1 || new_y == y - 1) {
                if let Some(piece) = board.get_piece(new_position) {
                    if piece.color == "white" {
                        return true;
                    }
                }
            }
        }

        false
    }

    pub fn get_possible_moves(&self, board: &Board) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        let (x, y) = self.position;

        if self.color == "white" {
            if x < 7 {
                if board.get_piece((x + 1, y)).is_none() {
                    moves.push((x + 1, y));
                }
                if y < 7 {
                    if let Some(piece) = board.get_piece((x + 1, y + 1)) {
                        if piece.color == "black" {
                            moves.push((x + 1, y + 1));
                        }
                    }
                }
                if y > 0 {
                    if let Some(piece) = board.get_piece((x + 1, y - 1)) {
                        if piece.color == "black" {
                            moves.push((x + 1, y - 1));
                        }
                    }
                }
            }
        } else {
            if x > 0 {
                if board.get_piece((x - 1, y)).is_none() {
                    moves.push((x - 1, y));
                }
                if y < 7 {
                    if let Some(piece) = board.get_piece((x - 1, y + 1)) {
                        if piece.color == "white" {
                            moves.push((x - 1, y + 1));
                        }
                    }
                }
                if y > 0 {
                    if let Some(piece) = board.get_piece((x - 1, y - 1)) {
                        if piece.color == "white" {
                            moves.push((x - 1, y - 1));
                        }
                    }
                }
            }
        }

        moves
    }
}
```