```rust
use crate::game_components::chess_piece::{ChessPiece, Color};
use crate::game_components::bishop::Bishop;
use crate::game_components::knight::Knight;
use crate::game_components::queen::Queen;
use crate::game_components::king::King;
use crate::game_components::rook::Rook;
use crate::game_components::pawn::Pawn;
use crate::game_components::chessboard::{Chessboard, Square};
use crate::player_management::player::Player;

pub fn initialize_game() -> (Chessboard, Player, Player) {
    let mut board = Chessboard {
        squares: [[[Square::Empty; 8]; 8]; 8],
    };

    let player1 = Player {
        name: String::from("Player 1"),
        color: Color::White,
    };

    let player2 = Player {
        name: String::from("Player 2"),
        color: Color::Black,
    };

    // Place pieces on the starting positions
    for i in 0..8 {
        board.squares[i][1] = Square::Occupied(Box::new(Pawn { color: Color::White }));
        board.squares[i][6] = Square::Occupied(Box::new(Pawn { color: Color::Black }));
    }

    board.squares[0][0] = Square::Occupied(Box::new(Rook { color: Color::White }));
    board.squares[7][0] = Square::Occupied(Box::new(Rook { color: Color::White }));
    board.squares[0][7] = Square::Occupied(Box::new(Rook { color: Color::Black }));
    board.squares[7][7] = Square::Occupied(Box::new(Rook { color: Color::Black }));

    board.squares[1][0] = Square::Occupied(Box::new(Knight { color: Color::White }));
    board.squares[6][0] = Square::Occupied(Box::new(Knight { color: Color::White }));
    board.squares[1][7] = Square::Occupied(Box::new(Knight { color: Color::Black }));
    board.squares[6][7] = Square::Occupied(Box::new(Knight { color: Color::Black }));

    board.squares[2][0] = Square::Occupied(Box::new(Bishop { color: Color::White }));
    board.squares[5][0] = Square::Occupied(Box::new(Bishop { color: Color::White }));
    board.squares[2][7] = Square::Occupied(Box::new(Bishop { color: Color::Black }));
    board.squares[5][7] = Square::Occupied(Box::new(Bishop { color: Color::Black }));

    board.squares[3][0] = Square::Occupied(Box::new(Queen { color: Color::White }));
    board.squares[3][7] = Square::Occupied(Box::new(Queen { color: Color::Black }));

    board.squares[4][0] = Square::Occupied(Box::new(King { color: Color::White }));
    board.squares[4][7] = Square::Occupied(Box::new(King { color: Color::Black }));

    (board, player1, player2)
}
```