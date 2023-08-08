use crate::pieces::piece::Piece;
use crate::board::Board;
use crate::pieces::piece::Color;

pub struct Bishop {
    pub color: Color,
    pub position: (usize, usize),
}

impl Piece for Bishop {
    fn new(color: Color, position: (usize, usize)) -> Self {
        Bishop { color, position }
    }

    fn color(&self) -> Color {
        self.color
    }

    fn position(&self) -> (usize, usize) {
        self.position
    }

    fn move_piece(&mut self, to: (usize, usize)) {
        self.position = to;
    }

    fn valid_move(&self, board: &Board, to: (usize, usize)) -> bool {
        let dx = (self.position.0 as i32 - to.0 as i32).abs();
        let dy = (self.position.1 as i32 - to.1 as i32).abs();

        if dx == dy {
            let x_step = if to.0 > self.position.0 { 1 } else { -1 };
            let y_step = if to.1 > self.position.1 { 1 } else { -1 };
            let mut x = self.position.0 as i32 + x_step;
            let mut y = self.position.1 as i32 + y_step;

            while x != to.0 as i32 && y != to.1 as i32 {
                if let Some(piece) = board.get_piece((x as usize, y as usize)) {
                    return false;
                }
                x += x_step;
                y += y_step;
            }
            return true;
        }
        false
    }
}