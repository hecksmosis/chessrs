use crate::*;

pub struct PossibleMove {
    pub diff: (i8, i8),
    pub capture: bool,
    pub checker: fn(Position, Position, &Game) -> bool,
}

impl PossibleMove {
    pub const fn with_capture(
        diff: (i8, i8),
        capture: bool,
        checker: fn(Position, Position, &Game) -> bool
    ) -> Self {
        Self { diff, capture, checker }
    }

    pub const fn new(diff: (i8, i8), checker: fn(Position, Position, &Game) -> bool) -> Self {
        Self { diff, capture: false, checker }
    }

    pub fn check(&self, start_position: Position, game: &Game) -> bool {
        let end_position = start_position + self.diff;
        let to_move = game[start_position];
        Game::in_bounds(end_position) &&
            (self.checker)(start_position, end_position, game) &&
            !(
                game.position_attacked(game.king_positions[game.turn as usize]) ||
                (to_move.piece_type() == (PieceType::King as u8) &&
                    game.position_attacked(end_position))
            )
    }
}

pub const POSSIBLE_WHITE_PAWN_MOVES: [PossibleMove; 4] = [
    PossibleMove::with_capture((0, 1), false, |_, end, game| game[end].is_empty()),
    PossibleMove::with_capture((0, 2), false, |start, end, game| {
        game[end].is_empty() &&
            game.is_path(PiecePath::Straight, start, end) &&
            game[start].is_home_row()
    }),
    PossibleMove::with_capture((1, 1), true, pawn_capture_checker),
    PossibleMove::with_capture((-1, 1), true, pawn_capture_checker),
];

pub const POSSIBLE_BLACK_PAWN_MOVES: [PossibleMove; 4] = [
    PossibleMove::with_capture((0, -1), false, |_, end, game| game[end].is_empty()),
    PossibleMove::with_capture((0, -2), false, |start, end, game| {
        game[end].is_empty() &&
            game.is_path(PiecePath::Straight, start, end) &&
            game[start].is_home_row()
    }),
    PossibleMove::with_capture((1, -1), true, pawn_capture_checker),
    PossibleMove::with_capture((-1, -1), true, pawn_capture_checker),
];

pub const POSSIBLE_ROOK_MOVES: [PossibleMove; 28] = [
    PossibleMove::new((-7, 0), rook_checker),
    PossibleMove::new((-6, 0), rook_checker),
    PossibleMove::new((-5, 0), rook_checker),
    PossibleMove::new((-4, 0), rook_checker),
    PossibleMove::new((-3, 0), rook_checker),
    PossibleMove::new((-2, 0), rook_checker),
    PossibleMove::new((-1, 0), rook_checker),
    PossibleMove::new((1, 0), rook_checker),
    PossibleMove::new((2, 0), rook_checker),
    PossibleMove::new((3, 0), rook_checker),
    PossibleMove::new((4, 0), rook_checker),
    PossibleMove::new((5, 0), rook_checker),
    PossibleMove::new((6, 0), rook_checker),
    PossibleMove::new((7, 0), rook_checker),
    PossibleMove::new((0, -7), rook_checker),
    PossibleMove::new((0, -6), rook_checker),
    PossibleMove::new((0, -5), rook_checker),
    PossibleMove::new((0, -4), rook_checker),
    PossibleMove::new((0, -3), rook_checker),
    PossibleMove::new((0, -2), rook_checker),
    PossibleMove::new((0, -1), rook_checker),
    PossibleMove::new((0, 1), rook_checker),
    PossibleMove::new((0, 2), rook_checker),
    PossibleMove::new((0, 3), rook_checker),
    PossibleMove::new((0, 4), rook_checker),
    PossibleMove::new((0, 5), rook_checker),
    PossibleMove::new((0, 6), rook_checker),
    PossibleMove::new((0, 7), rook_checker),
];

fn pawn_capture_checker(start: Position, end: Position, game: &Game) -> bool {
    let end_piece = game[end];
    let last_move = game.moves.last();
    (!end_piece.is_empty() && end_piece.color() != game[start].color()) ||
        (end_piece.is_empty() &&
            last_move.is_some() &&
            last_move.unwrap().piece_type() == PieceType::Pawn &&
            last_move.unwrap().start_position().x == end.x &&
            (last_move.unwrap().start_position().y as isize) +
                (if game.turn == 1 { 2 } else { -2 }) ==
                (last_move.unwrap().end_position().y as isize))
}

fn rook_checker(start: Position, end: Position, game: &Game) -> bool {
    game.is_path(PiecePath::Straight, start, end)
}
