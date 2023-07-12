use crate::{ PieceType, Position, PMove };
use std::ops::Index;

#[derive(Debug, PartialEq, Clone)]
pub struct Moves(Vec<PMove>);

impl Moves {
    pub fn new() -> Self {
        Moves {
            0: Vec::with_capacity(32),
        }
    }

    pub fn last(&self) -> Option<&PMove> {
        self.0.last()
    }

    pub fn push(&mut self, item: PMove) {
        self.0.push(item);
    }

    pub fn get_with_type_and_color(&self, index: PieceType, color: u8) -> Vec<&PMove> {
        let mut moves = Vec::new();

        for item in self.0
            .iter()
            .skip(color as usize)
            .step_by(2) {
            if item.piece_type() == index {
                moves.push(item);
            }
        }

        moves
    }

    pub fn piece_moved(&self, initial_position: Position) -> bool {
        self.0
            .iter()
            .any(|x| {
                x.start_position() == initial_position ||
                    (x.end_position() == initial_position && x.is_capture())
            })
    }

    pub fn last_50(&self) -> Vec<&PMove> {
        self.0.iter().rev().take(50).collect()
    }
}

impl Index<i32> for Moves {
    type Output = PMove;

    fn index(&self, index: i32) -> &Self::Output {
        // If the index is negative index from the end
        if index < 0 {
            return &self.0[((self.0.len() as i32) + index) as usize];
        } else {
            return &self.0[index as usize];
        }
    }
}
