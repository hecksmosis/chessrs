use crate::*;

pub struct PossibleBacktrace {
    pub diff: (i8, i8),
    pub checker: fn(Position, &PMove, &Game) -> bool,
}

impl PossibleBacktrace {
    pub const fn new(diff: (i8, i8), checker: fn(Position, &PMove, &Game) -> bool) -> Self {
        Self { diff, checker }
    }

    pub fn check(&self, pmove: &PMove, game: &Game) -> bool {
        let start_position = pmove.end_position() - self.diff;
        let start_piece = game[start_position];
        Game::in_bounds(start_position) &&
            (self.checker)(start_position, pmove, game) &&
            start_piece.byte == pmove.byte(game) &&
            !(
                game.position_attacked(game.king_positions[game.turn as usize]) ||
                (pmove.piece_type() == PieceType::King &&
                    game.position_attacked(pmove.end_position()))
            )
    }
}

pub const WHITE_PAWN_MOVES: [PossibleBacktrace; 4] = [
    PossibleBacktrace::new((0, 1), |_, p_move, _| !p_move.is_capture()),
    PossibleBacktrace::new((0, 2), |start, p_move, game| {
        game.is_path(PiecePath::Straight, start, p_move.end_position()) && game[start].is_home_row()
    }),
    PossibleBacktrace::new((1, 1), pawn_capture_checker),
    PossibleBacktrace::new((-1, 1), pawn_capture_checker),
];

pub const BLACK_PAWN_MOVES: [PossibleBacktrace; 4] = [
    PossibleBacktrace::new((0, -1), |_, p_move, _| !p_move.is_capture()),
    PossibleBacktrace::new((0, -2), |start, p_move, game| {
        game.is_path(PiecePath::Straight, start, p_move.end_position()) && game[start].is_home_row()
    }),
    PossibleBacktrace::new((1, -1), pawn_capture_checker),
    PossibleBacktrace::new((-1, -1), pawn_capture_checker),
];

pub const KNIGHT_MOVES: [PossibleBacktrace; 8] = [
    PossibleBacktrace::new((1, 2), knight_checker),
    PossibleBacktrace::new((2, 1), knight_checker),
    PossibleBacktrace::new((2, -1), knight_checker),
    PossibleBacktrace::new((1, -2), knight_checker),
    PossibleBacktrace::new((-1, -2), knight_checker),
    PossibleBacktrace::new((-2, -1), knight_checker),
    PossibleBacktrace::new((-2, 1), knight_checker),
    PossibleBacktrace::new((-1, 2), knight_checker),
];

pub const ROOK_MOVES: [PossibleBacktrace; 28] = [
    PossibleBacktrace::new((-7, 0), rook_checker),
    PossibleBacktrace::new((-6, 0), rook_checker),
    PossibleBacktrace::new((-5, 0), rook_checker),
    PossibleBacktrace::new((-4, 0), rook_checker),
    PossibleBacktrace::new((-3, 0), rook_checker),
    PossibleBacktrace::new((-2, 0), rook_checker),
    PossibleBacktrace::new((-1, 0), rook_checker),
    PossibleBacktrace::new((1, 0), rook_checker),
    PossibleBacktrace::new((2, 0), rook_checker),
    PossibleBacktrace::new((3, 0), rook_checker),
    PossibleBacktrace::new((4, 0), rook_checker),
    PossibleBacktrace::new((5, 0), rook_checker),
    PossibleBacktrace::new((6, 0), rook_checker),
    PossibleBacktrace::new((7, 0), rook_checker),
    PossibleBacktrace::new((0, -7), rook_checker),
    PossibleBacktrace::new((0, -6), rook_checker),
    PossibleBacktrace::new((0, -5), rook_checker),
    PossibleBacktrace::new((0, -4), rook_checker),
    PossibleBacktrace::new((0, -3), rook_checker),
    PossibleBacktrace::new((0, -2), rook_checker),
    PossibleBacktrace::new((0, -1), rook_checker),
    PossibleBacktrace::new((0, 1), rook_checker),
    PossibleBacktrace::new((0, 2), rook_checker),
    PossibleBacktrace::new((0, 3), rook_checker),
    PossibleBacktrace::new((0, 4), rook_checker),
    PossibleBacktrace::new((0, 5), rook_checker),
    PossibleBacktrace::new((0, 6), rook_checker),
    PossibleBacktrace::new((0, 7), rook_checker),
];

pub const BISHOP_MOVES: [PossibleBacktrace; 28] = [
    PossibleBacktrace::new((1, 1), bishop_checker),
    PossibleBacktrace::new((2, 2), bishop_checker),
    PossibleBacktrace::new((3, 3), bishop_checker),
    PossibleBacktrace::new((4, 4), bishop_checker),
    PossibleBacktrace::new((5, 5), bishop_checker),
    PossibleBacktrace::new((6, 6), bishop_checker),
    PossibleBacktrace::new((7, 7), bishop_checker),
    PossibleBacktrace::new((1, -1), bishop_checker),
    PossibleBacktrace::new((2, -2), bishop_checker),
    PossibleBacktrace::new((3, -3), bishop_checker),
    PossibleBacktrace::new((4, -4), bishop_checker),
    PossibleBacktrace::new((5, -5), bishop_checker),
    PossibleBacktrace::new((6, -6), bishop_checker),
    PossibleBacktrace::new((7, -7), bishop_checker),
    PossibleBacktrace::new((-1, 1), bishop_checker),
    PossibleBacktrace::new((-2, 2), bishop_checker),
    PossibleBacktrace::new((-3, 3), bishop_checker),
    PossibleBacktrace::new((-4, 4), bishop_checker),
    PossibleBacktrace::new((-5, 5), bishop_checker),
    PossibleBacktrace::new((-6, 6), bishop_checker),
    PossibleBacktrace::new((-7, 7), bishop_checker),
    PossibleBacktrace::new((-1, -1), bishop_checker),
    PossibleBacktrace::new((-2, -2), bishop_checker),
    PossibleBacktrace::new((-3, -3), bishop_checker),
    PossibleBacktrace::new((-4, -4), bishop_checker),
    PossibleBacktrace::new((-5, -5), bishop_checker),
    PossibleBacktrace::new((-6, -6), bishop_checker),
    PossibleBacktrace::new((-7, -7), bishop_checker),
];

pub const KING_MOVES: [PossibleBacktrace; 8] = [
    PossibleBacktrace::new((1, 1), king_checker),
    PossibleBacktrace::new((1, 0), king_checker),
    PossibleBacktrace::new((1, -1), king_checker),
    PossibleBacktrace::new((0, 1), king_checker),
    PossibleBacktrace::new((0, -1), king_checker),
    PossibleBacktrace::new((-1, 1), king_checker),
    PossibleBacktrace::new((-1, 0), king_checker),
    PossibleBacktrace::new((-1, -1), king_checker),
];

fn king_checker(_: Position, p_move: &PMove, game: &Game) -> bool {
    check_valid_capture(p_move.is_capture(), &game[p_move.end_position()], game.turn) &&
        !game.position_attacked(p_move.end_position())
}

fn bishop_checker(start: Position, p_move: &PMove, game: &Game) -> bool {
    check_valid_capture(p_move.is_capture(), &game[p_move.end_position()], game.turn) &&
        game.is_path(PiecePath::Diagonal, start, p_move.end_position())
}

fn pawn_capture_checker(start: Position, p_move: &PMove, game: &Game) -> bool {
    let end_piece = game[p_move.end_position()];
    let last_move = game.moves.last();
    p_move.is_capture() &&
        ((!end_piece.is_empty() && end_piece.color() != game[start].color()) ||
            (end_piece.is_empty() &&
                last_move.is_some() &&
                last_move.unwrap().piece_type() == PieceType::Pawn &&
                last_move.unwrap().start_position().x == p_move.end_position().x &&
                (last_move.unwrap().start_position().y as isize) +
                    (if game.turn == 1 { 2 } else { -2 }) ==
                    (last_move.unwrap().end_position().y as isize)))
}

fn check_valid_capture(is_capture: bool, end_piece: &Piece, turn: u8) -> bool {
    (end_piece.is_empty() && is_capture) ||
        (!end_piece.is_empty() && is_capture && end_piece.color() != turn)
}

fn knight_checker(_: Position, p_move: &PMove, game: &Game) -> bool {
    check_valid_capture(p_move.is_capture(), &game[p_move.end_position()], game.turn)
}

fn rook_checker(start: Position, p_move: &PMove, game: &Game) -> bool {
    check_valid_capture(p_move.is_capture(), &game[p_move.end_position()], game.turn) &&
        game.is_path(PiecePath::Straight, start, p_move.end_position())
}
