pub mod pawn;
pub mod knight;
pub mod bishop;
pub mod rook;
pub mod queen;
pub mod king;

pub enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

pub struct Piece {
    piece_type: PieceType,
    position: (usize, usize),
    color: String,
    has_moved: bool,
}

impl Piece {
    pub fn new(piece_type: PieceType, position: (usize, usize), color: String) -> Self {
        Self {
            piece_type,
            position,
            color,
            has_moved: false,
        }
    }

    pub fn move_piece(&mut self, new_position: (usize, usize)) {
        self.position = new_position;
        self.has_moved = true;
    }
}