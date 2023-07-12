use crate::*;
use std::array;

fn create_pawn_row(color: bool) -> [Piece; 8] {
    array::from_fn(|i| {
        Piece::new(if color { 0b1001 } else { 0b0001 }, (i, if color { 6 } else { 1 }).into())
    })
}

fn create_start_row(color: bool) -> [Piece; 8] {
    (0b10..=0b110)
        .chain((0b10..=0b100).rev())
        .enumerate()
        .fold([Piece::empty((0, 0).into()); 8], |mut pieces, (index, piece)| {
            pieces[index] = Piece::new(
                ((color as u8) << 3) + piece,
                (index, if !color { 0 } else { 7 }).into()
            );
            pieces
        })
}

fn create_empty_row(y: usize) -> [Piece; 8] {
    array::from_fn(|i| Piece::new(0, (i, y).into()))
}

impl Default for Game {
    fn default() -> Self {
        Game {
            board: create_board(),
            turn: 0,
            check: Checks::default(),
            king_positions: [(4, 0).into(), (4, 7).into()],
            moves: Moves::new(),
            hash_history: vec![],
        }
    }
}

pub fn create_board() -> [[Piece; 8]; 8] {
    [
        create_start_row(true),
        create_pawn_row(true),
        create_empty_row(5),
        create_empty_row(4),
        create_empty_row(3),
        create_empty_row(2),
        create_pawn_row(false),
        create_start_row(false),
    ]
}
