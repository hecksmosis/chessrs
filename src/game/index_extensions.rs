use crate::*;
use std::ops::{Index, IndexMut};

impl Index<Position> for Game {
    type Output = Piece;

    fn index(&self, position: Position) -> &Self::Output {
        if !Game::in_bounds(position) {
            return &self.board[0][0]; // This index will be wrong but it will be discarded by Game::in_bounds
        }
        &self.board[7 - position.y][position.x]
    }
}

impl IndexMut<Position> for Game {
    fn index_mut(&mut self, position: Position) -> &mut Self::Output {
        if !Game::in_bounds(position) {
            return &mut self.board[0][0];
        }
        &mut self.board[7 - position.y][position.x]
    }
}

impl<T> Index<(T, T)> for Game
where
    T: Into<usize> + Copy + Clone,
{
    type Output = Piece;

    fn index(&self, (x, y): (T, T)) -> &Self::Output {
        &self[Into::<Position>::into((x.into(), y.into()))]
    }
}

impl<T> IndexMut<(T, T)> for Game
where
    T: Into<usize> + Copy + Clone,
{
    fn index_mut(&mut self, (x, y): (T, T)) -> &mut Self::Output {
        &mut self[Into::<Position>::into((x.into(), y.into()))]
    }
}
