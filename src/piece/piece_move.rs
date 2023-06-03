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

    pub fn from_piece_type(coords: Coords, game: &Game) -> Result<Self, String> {
        match coords.piece {
            PieceType::Pawn => match get_pawn_moves(&coords, if game.turn == 0 { -1 } else { 1 })
                .iter()
                .find(|(x, y)| {
                    println!("{}, {}", x, y);
                    if y > &7 || x > &7 {
                        return false;
                    }
                    let piece = game[*y][*x];
                    println!("make_move > previous_positions: {:?} ", piece);
                    piece.piece_type() == (coords.piece as u8) && piece.color() == game.turn
                }) {
                Some(x) => Ok(PieceMove::new(game[x.1][x.0], coords.is_capture, coords)),
                None => {
                    return Err("AAA: Invalid move".to_string());
                }
            },
            PieceType::Rook => {
                let result = get_rook_moves(&coords)
                    .iter()
                    .map(|(x, y)| (*x, *y))
                    .filter_map(|(x, y)| {
                        let piece = game[y][x];
                        if piece.piece_type() == (coords.piece as u8)
                            && piece.color() == game.turn
                            && PieceMove::new(piece, coords.is_capture, coords).is_valid(game, None)
                        {
                            Some(PieceMove::new(piece, coords.is_capture, coords))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<PieceMove>>();

                match result.len() {
                    0 => Err("BBB: Invalid move".to_string()),
                    1 => Ok(result[0].clone()),
                    _ => Err("CCC: Invalid move".to_string()),
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
                            && piece.color() == game.turn
                            && ((!coords.is_capture && game[coords].is_empty())
                                || (coords.is_capture && game[coords].color() != game.turn))
                    })
                    .map(|(x, y)| PieceMove::new(game[y][x], coords.is_capture, coords))
                    .collect::<Vec<PieceMove>>();

                match moves.len() {
                    0 => Err("DDD: Invalid move".to_string()),
                    1 => Ok(moves[0].clone()),
                    _ => Err("EEE: Invalid move".to_string()),
                }
            }
            PieceType::Bishop => {
                let moves = get_bishop_moves(&coords)
                    .into_iter()
                    .filter(|&(x, y)| {
                        println!("x: {}, y: {}", x, y);
                        if y > 7 || x > 7 {
                            return false;
                        }
                        let piece = game[y][x];
                        piece.piece_type() == (coords.piece as u8)
                            && piece.color() == game.turn
                            && PieceMove::new(game[y][x], coords.is_capture, coords)
                                .is_valid(game, None)
                    })
                    .map(|(x, y)| PieceMove::new(game[y][x], coords.is_capture, coords))
                    .collect::<Vec<PieceMove>>();

                match moves.len() {
                    0 => Err("FFF: Invalid move".to_string()),
                    1 => Ok(moves[0].clone()),
                    _ => Err("GGG: Invalid move".to_string()),
                }
            }
            PieceType::Queen => {
                let moves_as_bishop = get_bishop_moves(&coords)
                    .into_iter()
                    .filter(|&(x, y)| {
                        println!("x: {}, y: {}", x, y);
                        if y > 7 || x > 7 {
                            return false;
                        }
                        let piece = game[y][x];
                        piece.piece_type() == (coords.piece as u8)
                            && piece.color() == game.turn
                            && PieceMove::new(game[y][x], coords.is_capture, coords)
                                .is_valid(game, None)
                    })
                    .map(|(x, y)| PieceMove::new(game[y][x], coords.is_capture, coords))
                    .collect::<Vec<PieceMove>>();
                if moves_as_bishop.len() == 1 {
                    if moves_as_bishop[0].is_valid(game, Some(PieceType::Bishop)) {
                        return Ok(moves_as_bishop[0].clone());
                    }
                }

                let moves_as_rook = get_rook_moves(&coords)
                    .iter()
                    .map(|(x, y)| (*x, *y))
                    .filter_map(|(x, y)| {
                        let piece = game[y][x];
                        if piece.piece_type() == (coords.piece as u8)
                            && piece.color() == game.turn
                            && PieceMove::new(piece, coords.is_capture, coords).is_valid(game, None)
                        {
                            Some(PieceMove::new(piece, coords.is_capture, coords))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<PieceMove>>();

                if moves_as_rook.len() == 1 {
                    if moves_as_rook[0].is_valid(game, Some(PieceType::Rook)) {
                        return Ok(moves_as_rook[0].clone());
                    }
                }

                Err("HHH: Invalid move".to_string())
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
                            && piece.color() == game.turn
                            && PieceMove::new(game[y][x], coords.is_capture, coords)
                                .is_valid(game, None)
                    })
                    .map(|(x, y)| PieceMove::new(game[y][x], coords.is_capture, coords))
                    .collect::<Vec<PieceMove>>();

                match moves.len() {
                    0 => Err("III: Invalid move".to_string()),
                    1 => Ok(moves[0].clone()),
                    _ => unreachable!(),
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
                        game[self.end_coords].color() != game.turn)
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
                game[self.end_coords].is_empty()
                    && game.is_path(PiecePath::Diagonal, self.piece.coords, self.end_coords)
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
