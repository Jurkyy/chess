```rust
use crate::chess::board::Board;
use crate::chess::king::King;
use crate::chess::rook::Rook;
use crate::chess::piece::Piece;
use crate::chess::piece::Color;

pub fn can_castle(board: &Board, king: &King, rook: &Rook) -> bool {
    if king.has_moved || rook.has_moved {
        return false;
    }

    let (king_x, king_y) = king.position;
    let (rook_x, _rook_y) = rook.position;

    let min_x = king_x.min(rook_x);
    let max_x = king_x.max(rook_x);

    for x in (min_x + 1)..max_x {
        if let Some(piece) = board.get_piece(x, king_y) {
            return false;
        }
    }

    true
}

pub fn perform_castling(board: &mut Board, king: &mut King, rook: &mut Rook) {
    if !can_castle(board, king, rook) {
        panic!("Invalid castling move");
    }

    let (king_x, king_y) = king.position;
    let (rook_x, rook_y) = rook.position;

    if rook_x > king_x {
        king.position = (king_x + 2, king_y);
        rook.position = (king_x + 1, rook_y);
    } else {
        king.position = (king_x - 2, king_y);
        rook.position = (king_x - 1, rook_y);
    }

    king.has_moved = true;
    rook.has_moved = true;
}
```