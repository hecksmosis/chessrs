use crate::*;
use std::ops::{Index, IndexMut};

impl Index<Coords> for Game {
    type Output = Piece;

    fn index(&self, coords: Coords) -> &Self::Output {
        println!(
            "Immutable access to coords: x: {}, y: {}",
            coords.x, coords.y
        );
        &self.board[7 - coords.y][coords.x]
    }
}

impl IndexMut<Coords> for Game {
    fn index_mut(&mut self, coords: Coords) -> &mut Self::Output {
        println!("Mutable access to coords: x: {}, y: {}", coords.x, coords.y);
        &mut self.board[7 - coords.y][coords.x]
    }
}

impl Index<(usize, usize)> for Game {
    type Output = Piece;

    fn index(&self, coords: (usize, usize)) -> &Self::Output {
        println!(
            "Immutable access to coords: x: {}, y: {}",
            coords.0, coords.1
        );

        &self.board[7 - coords.1][coords.0]
    }
}

impl IndexMut<(usize, usize)> for Game {
    fn index_mut(&mut self, coords: (usize, usize)) -> &mut Self::Output {
        println!("Mutable access to coords: x: {}, y: {}", coords.0, coords.1);
        &mut self.board[7 - coords.0][coords.0]
    }
}

impl Index<usize> for Game {
    type Output = [Piece; 8];

    fn index(&self, index: usize) -> &Self::Output {
        println!(
            "Accessing y coordinate: {}, corresponding to {}",
            index,
            7 - index
        );
        &self.board[7 - index]
    }
}

impl IndexMut<usize> for Game {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.board[7 - index]
    }
}
