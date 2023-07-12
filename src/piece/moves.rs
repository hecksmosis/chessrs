use crate::{ Game, Position, PieceType };

pub fn for_pawn(color: u8) -> Vec<(i32, i32, bool, PieceType)> {
    PieceType::iter()
        .flat_map(|piece_type| {
            (
                if color == 0 {
                    vec![(0, 1, false), (0, 2, false), (1, 1, true), (-1, 1, true)]
                } else {
                    vec![(0, -1, false), (0, -2, false), (1, -1, true), (-1, -1, true)]
                }
            )
                .iter()
                .map(|&(dx, dy, is_capture)| (dx, dy, is_capture, piece_type))
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

pub fn for_knight() -> Vec<(i32, i32)> {
    vec![(1, 2), (1, -2), (-1, 2), (-1, -2), (2, 1), (2, -1), (-2, 1), (-2, -1)]
}

pub fn for_bishop(piece_position: Position) -> Vec<Position> {
    [
        (1, 1),
        (1, -1),
        (-1, 1),
        (-1, -1),
    ]
        .iter()
        .flat_map(|&(dx, dy)| {
            (1..=8)
                .map(move |i| piece_position + (dx * i, dy * i))
                .filter(|&pos| Game::in_bounds(pos))
        })
        .collect::<Vec<_>>()
}

pub fn for_rook(piece_position: Position) -> Vec<Position> {
    let mut possible_previous_positions = Vec::new();
    for i in 0..=7 {
        if 7 - i != piece_position.y {
            possible_previous_positions.push(piece_position.with_y(7 - i));
        }
        if i != piece_position.x {
            possible_previous_positions.push(piece_position.with_x(i));
        }
    }
    possible_previous_positions
        .into_iter()
        .filter(|&pos| Game::in_bounds(pos))
        .collect::<Vec<_>>()
}

pub fn for_queen(piece_position: Position) -> Vec<Position> {
    for_bishop(piece_position).into_iter().chain(for_rook(piece_position)).collect::<Vec<_>>()
}

pub fn for_king(king_position: Position) -> Vec<Position> {
    vec![(1, 1), (1, -1), (-1, 1), (-1, -1), (1, 0), (-1, 0), (0, 1), (0, -1)]
        .into_iter()
        .map(|(dx, dy)| king_position + (dx, dy))
        .filter(|&pos| Game::in_bounds(pos))
        .collect::<Vec<_>>()
}
