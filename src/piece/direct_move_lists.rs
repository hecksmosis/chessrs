use crate::*;

macro_rules! new_move {
    ($coords:expr, $checker:expr) => {
        PossibleMove::new($coords, $checker)
    };
}

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

pub const POSSIBLE_KNIGHT_MOVES: [PossibleMove; 8] = [
    PossibleMove::new((1, 2), |_, _, _| true),
    PossibleMove::new((2, 1), |_, _, _| true),
    PossibleMove::new((2, -1), |_, _, _| true),
    PossibleMove::new((1, -2), |_, _, _| true),
    PossibleMove::new((-1, -2), |_, _, _| true),
    PossibleMove::new((-2, -1), |_, _, _| true),
    PossibleMove::new((-2, 1), |_, _, _| true),
    PossibleMove::new((-1, 2), |_, _, _| true),
];

pub const POSSIBLE_KING_MOVES: [PossibleMove; 8] = [
    PossibleMove::new((1, 1), |_, _, _| true),
    PossibleMove::new((1, 0), |_, _, _| true),
    PossibleMove::new((1, -1), |_, _, _| true),
    PossibleMove::new((0, 1), |_, _, _| true),
    PossibleMove::new((0, -1), |_, _, _| true),
    PossibleMove::new((-1, 1), |_, _, _| true),
    PossibleMove::new((-1, 0), |_, _, _| true),
    PossibleMove::new((-1, -1), |_, _, _| true),
];

pub const DEFAULT_MOVE: PossibleMove = PossibleMove::new((0, 0), |_, _, _| true);

macro_rules! generate_moves {
    ($name:ident, $diff:expr, $checker:expr) => {
        pub const $name: [PossibleMove; 28] = {
            let mut arr = [DEFAULT_MOVE; 28];
            let mut index = 0;
            let mut i = -7;
            while i <= 7 {
                if i == 0 {
                    i+=1;
                    continue;
                }
                arr[index] = new_move!((i * $diff.0, i * $diff.1), $checker);
                arr[index + 14] = new_move!((-i * $diff.0, i * $diff.1), $checker);
                index += 1;
                i+=1;
            }
            arr
        };
    };
}

generate_moves!(POSSIBLE_ROOK_MOVES, (1, 0), rook_checker);
generate_moves!(POSSIBLE_BISHOP_MOVES, (1, 1), rook_checker);

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
