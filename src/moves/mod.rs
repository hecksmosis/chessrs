use crate::{PieceMove, PieceType, Position};
use std::ops::Index;

#[derive(Debug, PartialEq, Clone)]
pub struct Moves(Vec<PieceMove>);

impl Moves {
    pub fn new() -> Self {
        Moves {
            0: Vec::with_capacity(32),
        }
    }

    pub fn last(&self) -> Option<&PieceMove> {
        self.0.last()
    }

    pub fn push(&mut self, item: PieceMove) {
        self.0.push(item);
    }

    pub fn get_with_type_and_color(&self, index: PieceType, color: u8) -> Vec<&PieceMove> {
        let mut moves = Vec::new();

        for item in self.0.iter() {
            if item.piece.piece_type() == index as u8 && item.piece.color() == color {
                moves.push(item);
            }
        }

        moves
    }

    pub fn piece_moved(&self, initial_position: Position) -> bool {
        self.0.iter().any(|x| {
            x.piece.position == initial_position
                || (x.end_position == initial_position && x.is_capture)
        })
    }
}

impl Index<i32> for Moves {
    type Output = PieceMove;

    fn index(&self, index: i32) -> &Self::Output {
        // If the index is negative index from the end
        if index < 0 {
            return &self.0[(self.0.len() as i32 + index) as usize];
        } else {
            return &self.0[index as usize];
        }
    }
}
