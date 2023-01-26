enum Piece {
    None,
    Pawn,
    Rook,
    Knight,
    Bishop,
    Queen,
    King,
}

struct ChessBoard {
    squares: [[Piece; 8]; 8],
}

impl ChessBoard {
    fn new() -> ChessBoard {
        let mut board = ChessBoard {
            squares: [[Piece::None; 8]; 8],
        };

        // Initialize the pawns
        for i in 0..8 {
            board.squares[1][i] = Piece::Pawn;
            board.squares[6][i] = Piece::Pawn;
        }

        // Initialize the rooks
        board.squares[0][0] = Piece::Rook;
        board.squares[0][7] = Piece::Rook;
        board.squares[7][0] = Piece::Rook;
        board.squares[7][7] = Piece::Rook;

        // Initialize the knights
        board.squares[0][1] = Piece::Knight;
        board.squares[0][6] = Piece::Knight;
        board.squares[7][1] = Piece::Knight;
        board.squares[7][6] = Piece::Knight;

        // Initialize the bishops
        board.squares[0][2] = Piece::Bishop;
        board.squares[0][5] = Piece::Bishop;
        board.squares[7][2] = Piece::Bishop;
        board.squares[7][5] = Piece::Bishop;

        // Initialize the queen and king
        board.squares[0][3] = Piece::Queen;
        board.squares[0][4] = Piece::King;
        board.squares[7][3] = Piece::Queen;
        board.squares[7][4] = Piece::King;

        board
    }
}
