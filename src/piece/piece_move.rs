use crate::*;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct PieceMove {
    pub piece: Piece,
    pub is_capture: bool,
    pub end_coords: Coords,
}

impl Display for PieceMove {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        write!(f, "{} -> {}", self.piece, self.end_coords)
    }
}

impl PieceMove {
    pub fn new(piece: Piece, is_capture: bool, end_coords: Coords) -> Self {
        PieceMove {
            piece,
            is_capture,
            end_coords,
        }
    }

    pub fn from_piece_type(
        coords: Coords,
        game: &Game,
        skip_check: bool,
        turn_to_use: u8,
    ) -> Result<Self, String> {
        match coords.piece {
            PieceType::Pawn => match get_pawn_moves(&coords, if turn_to_use == 0 { -1 } else { 1 })
                .iter()
                .find(|(x, y)| {
                    if y > &7 || x > &7 {
                        return false;
                    }
                    let piece = game[*y][*x];
                    piece.piece_type() == (coords.piece as u8)
                        && ((piece.color() == turn_to_use && !skip_check)
                            || (skip_check && piece.color() != turn_to_use))
                }) {
                Some(x) => Ok(PieceMove::new(game[x.1][x.0], coords.is_capture, coords)),
                None => {
                    return Err("Pawn: Invalid move (0 possible positions)".to_string());
                }
            },
            PieceType::Rook => {
                let moves = generate_rook_moves(coords, game, skip_check, turn_to_use);
                match moves.len() {
                    0 => Err("Rook: Invalid move (0 possible positions)".to_string()),
                    1 => Ok(moves[0].clone()),
                    _ => Err("Rook: Invalid move (ambiguous)".to_string()),
                }
            }
            PieceType::Knight => {
                let moves = get_knight_moves()
                    .into_iter()
                    .map(|(dx, dy)| {
                        (
                            ((coords.x as i32) + dx) as usize,
                            ((coords.y as i32) + dy) as usize,
                        )
                    })
                    .filter(|&(x, y)| {
                        if x > 7 || y > 7 {
                            return false;
                        }
                        let piece = game[y][x];
                        x < 8
                            && y < 8
                            && piece.piece_type() == (coords.piece as u8)
                            && ((piece.color() == turn_to_use && !skip_check)
                                || (skip_check && piece.color() != turn_to_use))
                            && ((!coords.is_capture && game[coords].is_empty())
                                || (coords.is_capture && game[coords].color() != turn_to_use))
                    })
                    .map(|(x, y)| PieceMove::new(game[y][x], coords.is_capture, coords))
                    .collect::<Vec<PieceMove>>();

                match moves.len() {
                    0 => Err("Knight: Invalid move (0 possible positions)".to_string()),
                    1 => Ok(moves[0].clone()),
                    _ => Err("Knight: Invalid move (ambiguous)".to_string()),
                }
            }
            PieceType::Bishop => {
                let moves = generate_bishop_moves(coords, game, skip_check, turn_to_use);
                match moves.len() {
                    0 => Err("Bishop: Invalid move (0 possible positions)".to_string()),
                    1 => Ok(moves[0].clone()),
                    _ => Err("Bishop: Invalid move (ambiguous)".to_string()),
                }
            }
            PieceType::Queen => {
                let moves_as_bishop = generate_bishop_moves(coords, game, skip_check, turn_to_use);
                if moves_as_bishop.len() == 1 {
                    if moves_as_bishop[0].is_valid(game, Some(PieceType::Bishop)) {
                        println!(
                            "Queen moves as bishop (checking with Bishop): {:?}",
                            moves_as_bishop[0]
                        );
                        return Ok(moves_as_bishop[0].clone());
                    }
                }

                let moves_as_rook = generate_rook_moves(coords, game, skip_check, turn_to_use);
                println!("Queen moves as rook: {:?}", moves_as_rook);
                if moves_as_rook.len() == 1 {
                    if moves_as_rook[0].is_valid(game, Some(PieceType::Rook)) {
                        return Ok(moves_as_rook[0].clone());
                    }
                }

                Err("Queen: Invalid move (0 possible positions)".to_string())
            }
            PieceType::King => {
                let moves = get_king_moves()
                    .iter()
                    .map(|(dx, dy)| {
                        (
                            ((coords.x as i32) + dx) as usize,
                            ((coords.y as i32) + dy) as usize,
                        )
                    })
                    .filter(|&(x, y)| {
                        if y > 7 || x > 7 {
                            return false;
                        }
                        let piece = game[y][x];
                        piece.piece_type() == (coords.piece as u8)
                            && ((piece.color() == turn_to_use && !skip_check)
                                || (skip_check && piece.color() != turn_to_use))
                            && PieceMove::new(game[y][x], coords.is_capture, coords)
                                .is_valid(game, None)
                    })
                    .map(|(x, y)| PieceMove::new(game[y][x], coords.is_capture, coords))
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

    pub fn is_valid(&self, game: &Game, piece_type: Option<PieceType>) -> bool {
        match if let Some(p_type) = piece_type {
            p_type
        } else {
            self.end_coords.piece
        } {
            PieceType::Pawn => {
                let y_move: isize = if game.turn == 0 { 1 } else { -1 };

                (!self.is_capture &&
                    (self.end_coords.y == (((self.piece.coords.y as isize) + y_move) as usize) ||
                        (self.piece.is_home_row() &&
                            game.is_path(PiecePath::Straight, self.piece.coords, self.end_coords) &&
                            self.end_coords.y ==
                                (((self.piece.coords.y as isize) + y_move * 2) as usize))) &&
                    self.end_coords.x == self.piece.coords.x &&
                    game[self.end_coords].is_empty()) || // Capture
                    (self.is_capture &&
                        self.end_coords.y == (((self.piece.coords.y as isize) + y_move) as usize) &&
                        ((self.end_coords.x as isize) - (self.piece.coords.x as isize)).abs() ==
                            1 &&
                        !game[self.end_coords].is_empty() &&
                        game[self.end_coords].color() != self.piece.color())
            }
            PieceType::Rook => {
                ((self.end_coords.y != self.piece.coords.y
                    && self.end_coords.x == self.piece.coords.x)
                    || (self.end_coords.y == self.piece.coords.y
                        && self.end_coords.x != self.piece.coords.x))
                    && game.is_path(PiecePath::Straight, self.piece.coords, self.end_coords)
                    && ((game[self.end_coords].is_empty() && !self.is_capture)
                        || (!game[self.end_coords].is_empty()
                            && self.is_capture
                            && game[self.end_coords].color() != self.piece.color()))
            }
            PieceType::Knight => true, // The check for the knight is done in the move purging
            PieceType::Bishop => {
                println!("Bishop: {:?} -> {:?}", self.piece.coords, self.end_coords);
                println!("{:?}", self);
                println!(
                    "empty: {}, capture: {}, colors: {}",
                    !game[self.end_coords].is_empty(),
                    self.is_capture,
                    game[self.end_coords].color() != self.piece.color()
                );
                println!(
                    "is_path: {}",
                    game.is_path(PiecePath::Diagonal, self.piece.coords, self.end_coords)
                );

                game.is_path(PiecePath::Diagonal, self.piece.coords, self.end_coords)
                    && ((!game[self.end_coords].is_empty()
                        && self.is_capture
                        && game[self.end_coords].color() != self.piece.color())
                        || (game[self.end_coords].is_empty() && !self.is_capture))
            }
            PieceType::Queen => true,
            PieceType::King => {
                (game[self.end_coords].is_empty() && !self.is_capture)
                    || (!game[self.end_coords].is_empty()
                        && self.is_capture
                        && game[self.end_coords].color() != self.piece.color())
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
    coords: Coords,
    game: &Game,
    skip_check: bool,
    turn_to_use: u8,
) -> Vec<PieceMove> {
    get_bishop_moves(&coords)
        .into_iter()
        .filter(|&(x, y)| {
            if y > 7 || x > 7 {
                return false;
            }
            let piece = game[y][x];
            piece.piece_type() == (coords.piece as u8)
                && ((piece.color() == turn_to_use && !skip_check)
                    || (skip_check && piece.color() != turn_to_use))
                && PieceMove::new(game[y][x], coords.is_capture, coords)
                    .is_valid(game, Some(PieceType::Bishop))
        })
        .map(|(x, y)| PieceMove::new(game[y][x], coords.is_capture, coords))
        .collect::<Vec<PieceMove>>()
}

fn generate_rook_moves(
    coords: Coords,
    game: &Game,
    skip_check: bool,
    turn_to_use: u8,
) -> Vec<PieceMove> {
    get_rook_moves(&coords)
        .iter()
        .map(|(x, y)| (*x, *y))
        .filter_map(|(x, y)| {
            let piece = game[y][x];
            if piece.piece_type() == (coords.piece as u8)
                && ((piece.color() == turn_to_use && !skip_check)
                    || (skip_check && piece.color() != turn_to_use))
                && PieceMove::new(piece, coords.is_capture, coords).is_valid(game, None)
            {
                Some(PieceMove::new(piece, coords.is_capture, coords))
            } else {
                None
            }
        })
        .collect::<Vec<PieceMove>>()
}
