use crate::*;

#[derive(Debug, PartialEq, Clone)]
pub struct PieceMove {
    pub piece: Piece,
    pub end_position: Position,
    pub is_capture: bool,
    pub en_passant: bool,
    pub castling: Option<(Position, Position)>,
}

impl Display for PieceMove {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        write!(f, "{} -> {}", self.piece, self.end_position)
    }
}

impl PieceMove {
    pub fn new(
        piece: Piece,
        end_position: Position,
        is_capture: bool,
        en_passant: bool,
        castling: Option<(Position, Position)>,
    ) -> Self {
        PieceMove {
            piece,
            end_position,
            is_capture,
            en_passant,
            castling,
        }
    }

    pub fn build(
        piece: Piece,
        end_position: Position,
        is_capture: bool,
        en_passant: bool,
        castling: Option<(Position, Position)>,
        game: &Game,
    ) -> Option<Self> {
        let p_move = PieceMove {
            piece,
            end_position,
            is_capture,
            en_passant,
            castling,
        };
        if !Game::in_bounds(end_position) {
            return None;
        }
        if p_move.is_valid(game, piece.piece_type().into()) {
            Some(p_move)
        } else {
            None
        }
    }

    pub fn from_piece(piece: Piece, game: &Game) -> Vec<Self> {
        let mut moves = Vec::new();
        moves.extend(match piece.piece_type().into() {
            PieceType::Pawn => for_pawn(game.turn)
                .iter()
                .map(|&(x, y, capture)| {
                    (
                        piece.position + (x, y),
                        capture,
                        game.moves.last().is_some()
                            && game.moves.last().unwrap().piece.piece_type()
                                == PieceType::Pawn as u8
                            && game.moves.last().unwrap().piece.position.x as isize
                                + if game.turn == 1 { 1 } else { -1 }
                                == piece.position.x as isize
                            && game.moves.last().unwrap().piece.position.y as isize
                                + if game.turn == 1 { 2 } else { -2 }
                                == game.moves.last().unwrap().end_position.y as isize
                            && game.moves.last().unwrap().end_position.y as isize
                                == piece.position.y as isize
                            && capture,
                    )
                })
                .filter_map(|(pos, capture, en_passant)| {
                    PieceMove::build(piece, pos, capture, en_passant, None, game)
                })
                .collect(),
            PieceType::Knight => for_knight()
                .into_iter()
                .map(|diff| piece.position + diff)
                .filter_map(|pos| {
                    PieceMove::build(piece, pos, !game[pos].is_empty(), false, None, game)
                })
                .collect(),
            PieceType::Bishop => for_bishop(piece.position)
                .into_iter()
                .filter_map(|pos| {
                    PieceMove::build(piece, pos, !game[pos].is_empty(), false, None, game)
                })
                .collect(),
            PieceType::Rook => for_rook(piece.position)
                .into_iter()
                .filter_map(|pos| {
                    PieceMove::build(piece, pos, !game[pos].is_empty(), false, None, game)
                })
                .collect(),
            PieceType::Queen => for_queen(piece.position)
                .into_iter()
                .filter_map(|pos| {
                    PieceMove::build(piece, pos, !game[pos].is_empty(), false, None, game)
                })
                .collect(),
            PieceType::King => for_king(piece.position)
                .into_iter()
                .filter_map(|pos| {
                    PieceMove::build(piece, pos, !game[pos].is_empty(), false, None, game)
                })
                .collect(),
            _ => Vec::new(),
        });
        moves
    }

    pub fn from_piece_type(
        input: Input,
        game: &mut Game,
        skip_check: bool,
        turn_to_use: u8,
    ) -> Result<Self, String> {
        match input.piece_type {
            PieceType::Pawn => {
                match inv_for_pawn(&input, if turn_to_use == 0 { -1 } else { 1 }, game)
                    .iter()
                    .find(|(x, y, can_en_croissant)| {
                        if y > &7 || x > &7 {
                            return false;
                        }
                        println!("{:?}", (x, y, can_en_croissant));

                        let piece = game[(*x, *y)];
                        (piece.piece_type() == (input.piece_type as u8))
                            && ((piece.color() == turn_to_use && !skip_check)
                                || (skip_check && piece.color() != turn_to_use))
                    }) {
                    Some(x) => Ok(PieceMove::new(
                        game[(x.0, x.1)],
                        input.end_position,
                        input.is_capture,
                        x.2,
                        None,
                    )),
                    None => {
                        return Err("Pawn: Invalid move (0 possible positions)".to_string());
                    }
                }
            }
            PieceType::Rook => {
                let moves = generate_rook_moves(input, game, skip_check, turn_to_use);
                match moves.len() {
                    0 => Err("Rook: Invalid move (0 possible positions)".to_string()),
                    1 => Ok(moves[0].clone()),
                    _ => Err("Rook: Invalid move (ambiguous)".to_string()),
                }
            }
            PieceType::Knight => {
                let moves = for_knight()
                    .into_iter()
                    .map(|diff| input.end_position + diff)
                    .filter(|&pos| {
                        if !Game::in_bounds(pos) {
                            return false;
                        }
                        let piece = game[pos];
                        piece.piece_type() == (input.piece_type as u8)
                            && ((piece.color() == turn_to_use && !skip_check)
                                || (skip_check && piece.color() != turn_to_use))
                    })
                    .map(|pos| {
                        PieceMove::new(game[pos], input.end_position, input.is_capture, false, None)
                    })
                    .collect::<Vec<PieceMove>>();

                match moves.len() {
                    0 => Err("Knight: Invalid move (0 possible positions)".to_string()),
                    1 => Ok(moves[0].clone()),
                    _ => Err("Knight: Invalid move (ambiguous)".to_string()),
                }
            }
            PieceType::Bishop => {
                let moves = generate_bishop_moves(input, game, skip_check, turn_to_use);
                match moves.len() {
                    0 => Err("Bishop: Invalid move (0 possible positions)".to_string()),
                    1 => Ok(moves[0].clone()),
                    _ => Err("Bishop: Invalid move (ambiguous)".to_string()),
                }
            }
            PieceType::Queen => {
                let moves_as_bishop = generate_bishop_moves(input, game, skip_check, turn_to_use);
                if moves_as_bishop.len() == 1 {
                    if moves_as_bishop[0].is_valid(game, PieceType::Bishop) {
                        return Ok(moves_as_bishop[0].clone());
                    }
                }

                let moves_as_rook = generate_rook_moves(input, game, skip_check, turn_to_use);
                if moves_as_rook.len() == 1 {
                    if moves_as_rook[0].is_valid(game, PieceType::Rook) {
                        return Ok(moves_as_rook[0].clone());
                    }
                }

                Err("Queen: Invalid move (0 possible positions)".to_string())
            }
            PieceType::King => {
                // Handle castling
                if input.castling != 0 {
                    if game
                        .moves
                        .get_with_type_and_color(PieceType::King, game.turn)
                        .len()
                        > 0
                    {
                        return Err("Invalid move: cannot castle".to_string());
                    }

                    let king_pos = if game.turn == 0 {
                        (4usize, 0).into()
                    } else {
                        (4usize, 7).into()
                    };

                    let rook_pos = match (game.turn, input.castling) {
                        (0, 1) => (7usize, 0).into(),
                        (0, 2) => (0usize, 0).into(),
                        (1, 1) => (7usize, 7).into(),
                        (1, 2) => (0usize, 7).into(),
                        (_, _) => unreachable!(),
                    };

                    let (end_king_pos, end_rook_pos): (Position, Position) =
                        match (game.turn, input.castling) {
                            (0, 1) => ((6, 0).into(), (5, 0).into()),
                            (0, 2) => ((2, 0).into(), (3, 0).into()),
                            (1, 1) => ((6, 7).into(), (5, 7).into()),
                            (1, 2) => ((2, 7).into(), (3, 7).into()),
                            (_, _) => unreachable!(),
                        };
                    if input.castling == 1 {
                        if game.moves.piece_moved(if game.turn == 0 {
                            (0usize, 0).into()
                        } else {
                            (0usize, 7).into()
                        }) {
                            return Err("Invalid move: rook has already moved".to_string());
                        }
                    } else {
                        if game.moves.piece_moved(if game.turn == 0 {
                            (7usize, 0).into()
                        } else {
                            (7usize, 7).into()
                        }) {
                            return Err("Invalid move: rook has already moved".to_string());
                        }
                    }

                    if !game.is_path(PiecePath::Straight, king_pos, rook_pos) {
                        return Err("Invalid move: path is blocked".to_string());
                    }

                    return Ok(PieceMove::new(
                        game[king_pos],
                        end_king_pos,
                        false,
                        false,
                        Some((end_king_pos, end_rook_pos)),
                    ));
                }

                let moves = get_king_moves()
                    .iter()
                    .map(|&diff| input.end_position + diff)
                    .filter(|&pos| {
                        if !Game::in_bounds(pos) {
                            return false;
                        }
                        let piece = game[pos];
                        piece.piece_type() == (input.piece_type as u8)
                            && ((piece.color() == turn_to_use && !skip_check)
                                || (skip_check && piece.color() != turn_to_use))
                            && PieceMove::new(
                                piece,
                                input.end_position,
                                input.is_capture,
                                false,
                                None,
                            )
                            .is_valid(game, input.piece_type)
                    })
                    .map(|pos| {
                        PieceMove::new(game[pos], input.end_position, input.is_capture, false, None)
                    })
                    .collect::<Vec<PieceMove>>();

                match moves.len() {
                    0 => Err("King: Invalid move (0 possible positions)".to_string()),
                    1 => Ok(moves[0].clone()),
                    _ => unreachable!(), // There cannot be two kings of the same color
                }
            }
            _ => unimplemented!(),
        }
    }

    pub fn is_valid(&self, game: &Game, piece_type: PieceType) -> bool {
        match piece_type {
            PieceType::Pawn => {
                let y_move: isize = if game.turn == 0 { 1 } else { -1 };

                (!self.is_capture &&
                    (self.end_position.y == (((self.piece.position.y as isize) + y_move) as usize) ||
                        (self.piece.is_home_row() &&
                            game.is_path(PiecePath::Straight, self.piece.position, self.end_position) &&
                            self.end_position.y ==
                                (((self.piece.position.y as isize) + y_move * 2) as usize))) &&
                    self.end_position.x == self.piece.position.x &&
                    game[self.end_position].is_empty()) || // Capture
                    (self.is_capture &&
                        self.end_position.y == (((self.piece.position.y as isize) + y_move) as usize) &&
                        (self.en_passant || (((self.end_position.x as isize) - (self.piece.position.x as isize)).abs() ==
                        1 && !game[self.end_position].is_empty() && // en croissant
                        game[self.end_position].color() != self.piece.color())))
            }
            PieceType::Rook => {
                ((self.end_position.y != self.piece.position.y
                    && self.end_position.x == self.piece.position.x)
                    || (self.end_position.y == self.piece.position.y
                        && self.end_position.x != self.piece.position.x))
                    && game.is_path(PiecePath::Straight, self.piece.position, self.end_position)
                    && ((game[self.end_position].is_empty() && !self.is_capture)
                        || (!game[self.end_position].is_empty()
                            && self.is_capture
                            && game[self.end_position].color() != self.piece.color()))
            }
            PieceType::Knight => {
                (!self.is_capture && game[self.end_position].is_empty())
                    || (self.is_capture && game[self.end_position].color() != game.turn)
            } // The check for the knight is done in the move purging
            PieceType::Bishop => {
                game.is_path(PiecePath::Diagonal, self.piece.position, self.end_position)
                    && ((!game[self.end_position].is_empty()
                        && self.is_capture
                        && game[self.end_position].color() != self.piece.color())
                        || (game[self.end_position].is_empty() && !self.is_capture))
            }
            PieceType::Queen => {
                self.is_valid(game, PieceType::Bishop) || self.is_valid(game, PieceType::Rook)
            }
            PieceType::King => {
                (game[self.end_position].is_empty() && !self.is_capture)
                    || (!game[self.end_position].is_empty()
                        && self.is_capture
                        && game[self.end_position].color() != self.piece.color())
                    || self.castling.is_some()
            }
            _ => unreachable!(),
        }
    }
}

pub enum PiecePath {
    Straight,
    Diagonal,
}

fn generate_bishop_moves(
    input: Input,
    game: &Game,
    skip_check: bool,
    turn_to_use: u8,
) -> Vec<PieceMove> {
    inv_for_bishop(&input)
        .into_iter()
        .filter(|&position| {
            if position.y > 7 || position.x > 7 {
                return false;
            }
            let piece = game[position];
            piece.piece_type() == (input.piece_type as u8)
                && ((piece.color() == turn_to_use && !skip_check)
                    || (skip_check && piece.color() != turn_to_use))
                && PieceMove::new(
                    game[position],
                    input.end_position,
                    input.is_capture,
                    false,
                    None,
                )
                .is_valid(game, PieceType::Bishop)
        })
        .map(|position| {
            PieceMove::new(
                game[position],
                input.end_position,
                input.is_capture,
                false,
                None,
            )
        })
        .collect::<Vec<PieceMove>>()
}

fn generate_rook_moves(
    input: Input,
    game: &Game,
    skip_check: bool,
    turn_to_use: u8,
) -> Vec<PieceMove> {
    inv_for_rook(&input)
        .iter()
        .filter_map(|&position| {
            let piece = game[position];
            if piece.piece_type() == (input.piece_type as u8)
                && ((piece.color() == turn_to_use && !skip_check)
                    || (skip_check && piece.color() != turn_to_use))
                && PieceMove::new(piece, input.end_position, input.is_capture, false, None)
                    .is_valid(game, input.piece_type)
            {
                Some(PieceMove::new(
                    piece,
                    input.end_position,
                    input.is_capture,
                    false,
                    None,
                ))
            } else {
                None
            }
        })
        .collect::<Vec<PieceMove>>()
}
