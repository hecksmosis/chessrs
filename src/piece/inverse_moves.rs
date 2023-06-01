use crate::*;

pub fn get_pawn_moves(coords: &Coords, move_dir: isize) -> [(usize, usize); 2] {
    if !coords.is_capture {
        [
            (coords.x, (coords.y as isize + move_dir) as usize),
            (coords.x, (coords.y as isize + (move_dir * 2)) as usize),
        ]
    } else {
        [
            (
                (coords.x as isize - 1) as usize,
                (coords.y as isize + move_dir) as usize,
            ),
            (
                (coords.x as isize + 1) as usize,
                (coords.y as isize + move_dir) as usize,
            ),
        ]
    }
}

pub fn get_rook_moves(coords: &Coords) -> Vec<(usize, usize)> {
    let mut possible_previous_positions = Vec::new();
    for i in 0..=7 {
        if 7 - i != coords.y {
            possible_previous_positions.push((coords.x, 7 - i));
        }
        if i != coords.x {
            possible_previous_positions.push((i, coords.y));
        }
    }
    possible_previous_positions
}

pub fn get_knight_moves() -> Vec<(i32, i32)> {
    vec![
        (1, 2),
        (1, -2),
        (-1, 2),
        (-1, -2),
        (2, 1),
        (2, -1),
        (-2, 1),
        (-2, -1),
    ]
}

pub fn get_bishop_moves(coords: &Coords) -> Vec<(usize, usize)> {
    [(1, 1), (1, -1), (-1, 1), (-1, -1)]
        .iter()
        .flat_map(|&(dx, dy)| {
            (1..=8)
                .map(move |i| (coords.x as i32 + dx * i, coords.y as i32 + dy * i))
                .filter(|&(x, y)| x >= 0 && x < 8 && y >= 0 && y <= 8)
                .map(|(x, y)| (x as usize, y as usize))
        })
        .collect::<Vec<(usize, usize)>>()
}
