use crate::board::Board;
use crate::pieces::piece::Piece;

pub struct Rook {
    pub position: (usize, usize),
    pub color: bool,
    pub has_moved: bool,
}

impl Piece for Rook {
    fn new(position: (usize, usize), color: bool) -> Self {
        Rook {
            position,
            color,
            has_moved: false,
        }
    }

    fn get_position(&self) -> (usize, usize) {
        self.position
    }

    fn set_position(&mut self, position: (usize, usize)) {
        self.position = position;
    }

    fn get_color(&self) -> bool {
        self.color
    }

    fn get_valid_moves(&self, board: &Board) -> Vec<Move> {
        let mut moves = Vec::new();

        // Horizontal and vertical moves
        for direction in &[(0, 1), (1, 0), (0, -1), (-1, 0)] {
            let mut current_position = self.position;

            loop {
                current_position.0 = (current_position.0 as isize + direction.0) as usize;
                current_position.1 = (current_position.1 as isize + direction.1) as usize;

                if !board.is_valid_position(current_position) {
                    break;
                }

                if let Some(piece) = board.get_piece(current_position) {
                    if piece.get_color() == self.color {
                        break;
                    } else {
                        moves.push(Move {
                            start: self.position,
                            end: current_position,
                        });
                        break;
                    }
                } else {
                    moves.push(Move {
                        start: self.position,
                        end: current_position,
                    });
                }
            }
        }

        moves
    }

    fn has_moved(&self) -> bool {
        self.has_moved
    }

    fn set_has_moved(&mut self, has_moved: bool) {
        self.has_moved = has_moved;
    }
}
