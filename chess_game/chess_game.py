```python
from .chess_board import ChessBoard
from .chess_player import ChessPlayer
from .castling import Castling

class ChessGame:
    def __init__(self):
        self.board = ChessBoard()
        self.players = [ChessPlayer('white'), ChessPlayer('black')]
        self.current_player = None
        self.castling = Castling(self.board)

    def start_game(self):
        self.current_player = self.players[0]

    def change_turn(self):
        self.current_player = self.players[1] if self.current_player == self.players[0] else self.players[0]

    def move_piece(self, piece, new_position):
        if self.castling.is_castling_move(piece, new_position):
            self.castling.perform_castling(piece, new_position)
        else:
            self.board.move_piece(piece, new_position)

        self.change_turn()

    def is_checkmate(self):
        return self.board.is_checkmate(self.current_player.color)

    def is_stalemate(self):
        return self.board.is_stalemate(self.current_player.color)
```