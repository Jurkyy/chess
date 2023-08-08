```rust
use crate::piece::Piece;
use crate::board::Board;

pub fn is_castling_possible(board: &Board, king: &Piece, rook: &Piece) -> bool {
    if king.has_moved || rook.has_moved {
        return false;
    }

    let (king_x, king_y) = king.position;
    let (rook_x, _) = rook.position;

    let min_x = king_x.min(rook_x);
    let max_x = king_x.max(rook_x);

    for x in (min_x + 1)..max_x {
        if board.get_piece_at(x, king_y).is_some() {
            return false;
        }
    }

    true
}

pub fn perform_castling(board: &mut Board, king: &mut Piece, rook: &mut Piece) {
    if !is_castling_possible(board, king, rook) {
        panic!("Castling is not possible");
    }

    let (king_x, king_y) = king.position;
    let (rook_x, _) = rook.position;

    if king_x < rook_x {
        king.position = (king_x + 2, king_y);
        rook.position = (king_x + 1, king_y);
    } else {
        king.position = (king_x - 2, king_y);
        rook.position = (king_x - 1, king_y);
    }

    king.has_moved = true;
    rook.has_moved = true;
}
```