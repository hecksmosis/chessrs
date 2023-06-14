use crate::*;
use std::ops::{Index, IndexMut};

impl Index<Coords> for Game {
    type Output = Piece;

    fn index(&self, coords: Coords) -> &Self::Output {
        &self.board[7 - coords.y][coords.x]
    }
}

impl IndexMut<Coords> for Game {
    fn index_mut(&mut self, coords: Coords) -> &mut Self::Output {
        &mut self.board[7 - coords.y][coords.x]
    }
}

impl Index<(usize, usize)> for Game {
    type Output = Piece;

    fn index(&self, coords: (usize, usize)) -> &Self::Output {
        &self.board[7 - coords.1][coords.0]
    }
}

impl IndexMut<(usize, usize)> for Game {
    fn index_mut(&mut self, coords: (usize, usize)) -> &mut Self::Output {
        &mut self.board[7 - coords.0][coords.0]
    }
}

impl Index<usize> for Game {
    type Output = [Piece; 8];

    fn index(&self, index: usize) -> &Self::Output {
        &self.board[7 - index]
    }
}

impl IndexMut<usize> for Game {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.board[7 - index]
    }
}
