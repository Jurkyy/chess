use crate::board::Board;
use crate::pieces::bishop::Bishop;
use crate::pieces::piece::Piece;
use crate::pieces::rook::Rook;

pub struct Queen {
    pub position: (usize, usize),
    pub color: bool,
}

impl Piece for Queen {
    fn new(position: (usize, usize), color: bool) -> Self {
        Queen { position, color }
    }

    fn get_color(&self) -> String {
        self.color.clone()
    }

    fn get_position(&self) -> (usize, usize) {
        self.position
    }

    fn set_position(&mut self, position: (usize, usize)) {
        self.position = position;
    }

    fn can_move(&self, board: &Board, mov: &Move) -> bool {
        let rook = Rook::new(self.position, self.color.clone());
        let bishop = Bishop::new(self.position, self.color.clone());

        rook.can_move(board, mov) || bishop.can_move(board, mov)
    }
}
