```python
class ChessPlayer:
    def __init__(self, color):
        self.color = color
        self.score = 0
        self.pieces = self.initialize_pieces()

    def initialize_pieces(self):
        pieces = []
        for i in range(8):  # pawns
            pieces.append(ChessPiece('Pawn', self.color))
        pieces.append(ChessPiece('Rook', self.color))  # rooks
        pieces.append(ChessPiece('Rook', self.color))
        pieces.append(ChessPiece('Knight', self.color))  # knights
        pieces.append(ChessPiece('Knight', self.color))
        pieces.append(ChessPiece('Bishop', self.color))  # bishops
        pieces.append(ChessPiece('Bishop', self.color))
        pieces.append(ChessPiece('Queen', self.color))  # queen
        pieces.append(ChessPiece('King', self.color))  # king
        return pieces

    def get_king(self):
        for piece in self.pieces:
            if piece.type == 'King':
                return piece

    def is_checkmate(self):
        return self.get_king().is_checkmate

    def has_castling_rights(self):
        return self.get_king().has_castling_rights
```