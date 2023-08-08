```python
class ChessPiece:
    def __init__(self, piece_type, color, position):
        self.piece_type = piece_type
        self.color = color
        self.position = position
        self.has_moved = False

    def move(self, new_position):
        self.position = new_position
        self.has_moved = True

    def can_castle(self):
        return self.piece_type == 'king' and not self.has_moved
```