```python
from chess_game.chess_board import ChessBoard
from chess_game.chess_piece import ChessPiece

def can_castle(king: ChessPiece, rook: ChessPiece, board: ChessBoard) -> bool:
    if king.has_moved or rook.has_moved:
        return False

    start, end = sorted([king.position, rook.position])

    for i in range(start + 1, end):
        if board.get_piece_at(i) is not None:
            return False

    return True

def perform_castle(king: ChessPiece, rook: ChessPiece, board: ChessBoard):
    if not can_castle(king, rook, board):
        raise ValueError("Cannot castle")

    king_position, rook_position = king.position, rook.position

    if king_position < rook_position:
        king.position = king_position + 2
        rook.position = rook_position - 2
    else:
        king.position = king_position - 2
        rook.position = rook_position + 3

    king.has_moved = True
    rook.has_moved = True
```