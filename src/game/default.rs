use crate::*;
use std::array;

fn create_pawn_row(color: bool) -> [Piece; 8] {
    array::from_fn(|i| {
        Piece::new(
            if color { 0b1001 } else { 0b0001 },
            (i, if color { 1 } else { 6 }).into(),
        )
    })
}

fn create_start_row(color: bool) -> [Piece; 8] {
    (0b10..=0b110).chain((0b10..=0b100).rev()).enumerate().fold(
        [Piece::empty(); 8],
        |mut pieces, (index, piece)| {
            pieces[index] = Piece::new(
                ((color as u8) << 3) + piece,
                (index, if !color { 7 } else { 0 }).into(),
            );
            pieces
        },
    )
}

fn create_empty_row() -> [Piece; 8] {
    [Piece::empty(); 8]
}

impl Default for Game {
    fn default() -> Self {
        Game {
            board: [
                create_start_row(true),
                create_pawn_row(true),
                create_empty_row(),
                create_empty_row(),
                create_empty_row(),
                create_empty_row(),
                create_pawn_row(false),
                create_start_row(false),
            ],
            turn: 0,
        }
    }
}
