use std::vec;

use crate::*;

pub fn inv_for_pawn(input: &Input, move_dir: isize, game: &mut Game) -> [(usize, usize, bool); 2] {
    if !input.is_capture {
        [
            (input.end_position.x, ((input.end_position.y as isize) + move_dir) as usize, false),
            (
                input.end_position.x,
                ((input.end_position.y as isize) + move_dir * 2) as usize,
                false,
            ),
        ]
    } else if
        game.moves.last().is_some() &&
        game.moves.last().unwrap().piece.piece_type() == (PieceType::Pawn as u8) &&
        game.moves.last().unwrap().piece.position.x == input.end_position.x &&
        (game.moves.last().unwrap().piece.position.y as isize) +
            (if game.turn == 1 { 2 } else { -2 }) ==
            (game.moves.last().unwrap().end_position.y as isize)
    {
        [
            (
                ((input.end_position.x as isize) - 1) as usize,
                ((input.end_position.y as isize) + move_dir) as usize,
                true,
            ),
            (
                ((input.end_position.x as isize) + 1) as usize,
                ((input.end_position.y as isize) + move_dir) as usize,
                true,
            ),
        ]
    } else {
        [
            (
                ((input.end_position.x as isize) - 1) as usize,
                ((input.end_position.y as isize) + move_dir) as usize,
                false,
            ),
            (
                ((input.end_position.x as isize) + 1) as usize,
                ((input.end_position.y as isize) + move_dir) as usize,
                false,
            ),
        ]
    }
}

pub fn inv_for_rook(input: &Input) -> Vec<Position> {
    let mut possible_previous_positions = Vec::new();
    for i in 0..=7 {
        if 7 - i != input.end_position.y {
            possible_previous_positions.push(input.end_position.with_y(7 - i));
        }
        if i != input.end_position.x {
            possible_previous_positions.push(input.end_position.with_x(i));
        }
    }
    possible_previous_positions
}

pub fn inv_for_bishop(input: &Input) -> Vec<Position> {
    [
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ]
        .iter()
        .flat_map(|&(dx, dy)| {
            (1..=8)
                .map(move |i| input.end_position + (dx * i, dy * i))
                .filter(|&pos| Game::in_bounds(pos))
        })
        .collect::<Vec<_>>()
}

pub fn get_king_moves() -> Vec<(i32, i32)> {
    vec![(1, 1), (1, -1), (-1, 1), (-1, -1), (1, 0), (-1, 0), (0, 1), (0, -1)]
}
