use std::any::Any;
use std::rc::Rc;

use super::chessboard::{Chessboard, Color};

// Helper trait for downcasting
pub trait Downcast {
    fn as_any(&self) -> &dyn Any;
}

impl<T> Downcast for T
where
    T: ChessPiece + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }
}

pub trait ChessPiece: Downcast {
    fn color(&self) -> Color;
    fn ascii_char(&self) -> char;
    fn valid_moves(&self, start: (usize, usize), chessboard: &Chessboard) -> Vec<(usize, usize)>;
}

// Implement the ChessPiece trait
impl<T: ChessPiece + 'static> Downcast for Rc<T> {
    fn as_any(&self) -> &dyn Any {
        self
    }
}
