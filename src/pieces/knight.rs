use crate::board::Board;
use crate::pieces::Piece;

pub struct Knight {
    pub position: (usize, usize),
    pub color: String,
}

impl Knight {
    pub fn new(position: (usize, usize), color: String) -> Self {
        Knight { position, color }
    }

    pub fn valid_moves(&self, board: &Board) -> Vec<(usize, usize)> {
        let mut moves = Vec::new();
        let possible_moves = [
            (-2, -1),
            (-2, 1),
            (-1, -2),
            (-1, 2),
            (1, -2),
            (1, 2),
            (2, -1),
            (2, 1),
        ];

        for &(dx, dy) in possible_moves.iter() {
            let new_position = (self.position.0 as i32 + dx, self.position.1 as i32 + dy);
            if new_position.0 >= 0
                && new_position.0 < 8
                && new_position.1 >= 0
                && new_position.1 < 8
            {
                let new_position = (new_position.0 as usize, new_position.1 as usize);
                if let Some(piece) = board.get_piece(new_position) {
                    if piece.color != self.color {
                        moves.push(new_position);
                    }
                } else {
                    moves.push(new_position);
                }
            }
        }

        moves
    }
}

impl Piece for Knight {
    fn move_piece(&mut self, new_position: (usize, usize)) {
        self.position = new_position;
    }

    fn get_position(&self) -> (usize, usize) {
        self.position
    }

    fn get_color(&self) -> String {
        self.color.clone()
    }
}
