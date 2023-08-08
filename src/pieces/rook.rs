use crate::board::Board;
use crate::pieces::Piece;

pub struct Rook {
    pub position: (usize, usize),
    pub color: String,
    pub has_moved: bool,
}

impl Rook {
    pub fn new(position: (usize, usize), color: String) -> Self {
        Rook {
            position,
            color,
            has_moved: false,
        }
    }

    pub fn valid_moves(&self, board: &Board) -> Vec<(usize, usize)> {
        let mut moves: Vec<(usize, usize)> = Vec::new();

        // Horizontal and vertical moves
        for i in 0..8 {
            moves.push((self.position.0, i));
            moves.push((i, self.position.1));
        }

        // Filter out invalid moves
        moves.retain(|&pos| board.is_valid_move(self.position, pos));

        moves
    }

    pub fn can_castle(&self, board: &Board) -> bool {
        !self.has_moved && self.clear_path_to_king(board)
    }

    fn clear_path_to_king(&self, board: &Board) -> bool {
        let king_position = board.get_king_position(&self.color);

        // Check if there are no pieces between the rook and the king
        for i in std::cmp::min(self.position.1, king_position.1) + 1
            ..std::cmp::max(self.position.1, king_position.1)
        {
            if board.get_piece((self.position.0, i)).is_some() {
                return false;
            }
        }

        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_valid_moves() {
        let board = Board::new();
        let rook = Rook::new((0, 0), "White".to_string());

        assert_eq!(rook.valid_moves(&board).len(), 14);
    }

    #[test]
    fn test_can_castle() {
        let mut board = Board::new();
        let rook = Rook::new((0, 0), "White".to_string());

        assert!(!rook.can_castle(&board));

        board.remove_piece((0, 1));
        board.remove_piece((0, 2));
        board.remove_piece((0, 3));

        assert!(rook.can_castle(&board));
    }
}
