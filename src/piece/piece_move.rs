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
        piece_type: PieceType,
        coords: Coords,
        game: &Game,
    ) -> Result<Self, String> {
        match piece_type {
            PieceType::Pawn => match get_pawn_moves(&coords, if game.turn == 0 { -1 } else { 1 })
                .iter()
                .find(|(x, y)| {
                    println!("{}, {}", x, y);
                    if y > &7 || x > &7 {
                        return false;
                    }
                    let piece = game[*y][*x];
                    println!("make_move > previous_positions: {:?} ", piece);
                    piece.piece_type() == coords.piece as u8 && piece.color() == game.turn
                }) {
                Some(x) => Ok(PieceMove::new(game[x.1][x.0], coords.is_capture, coords)),
                None => return Err("AAA: Invalid move".to_string()),
            },
            PieceType::Rook => {
                let result = get_rook_moves(&coords)
                    .iter()
                    .map(|(x, y)| (*x, *y))
                    .filter_map(|(x, y)| {
                        let piece = game[y][x];
                        if piece.piece_type() == coords.piece as u8
                            && piece.color() == game.turn
                            && PieceMove::new(piece, coords.is_capture, coords).is_valid(game)
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
                            (coords.x as i32 + dx) as usize,
                            (coords.y as i32 + dy) as usize,
                        )
                    })
                    .filter(|&(x, y)| {
                        if x > 7 || y > 7 {
                            return false;
                        }
                        let piece = game[y][x];
                        x < 8
                            && y < 8
                            && piece.piece_type() == coords.piece as u8
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
                        piece.piece_type() == coords.piece as u8
                            && piece.color() == game.turn
                            && PieceMove::new(game[y][x], coords.is_capture, coords).is_valid(game)
                    })
                    .map(|(x, y)| PieceMove::new(game[y][x], coords.is_capture, coords))
                    .collect::<Vec<PieceMove>>();

                match moves.len() {
                    0 => Err("FFF: Invalid move".to_string()),
                    1 => Ok(moves[0].clone()),
                    _ => Err("GGG: Invalid move".to_string()),
                }
            }

            _ => unimplemented!(),
        }
    }

    pub fn is_valid(&self, game: &Game) -> bool {
        match self.piece.piece_type() {
            0b1 => {
                let y_move: isize = if game.turn == 0 { 1 } else { -1 };
                println!(
                    "y_move: {}, color: {}, end_coords: {}, piece coords: {}, home_row: {}, y = y+2: {}, x maintained: {}, at end: {}, empty: {}",
                    y_move,
                    self.piece.color(),
                    self.end_coords.y,
                    self.piece.coords,
                    self.piece.is_home_row(),
                    self.end_coords.y
                            == (self.piece.coords.y as isize + (y_move * 2)) as usize,
                    self.end_coords.x == self.piece.coords.x,
                    game[self.end_coords],
                    game[self.end_coords].is_empty()
                );

                println!(
                    "{:?}, {:?}, {:?}, {:?}",
                    (self.end_coords.y == (self.piece.coords.y as isize + y_move) as usize),
                    (self.end_coords.x as isize - self.piece.coords.x as isize).abs() == 1,
                    !game[self.end_coords].is_empty(),
                    game[self.end_coords].color() != game.turn,
                );

                (!self.is_capture && ((self.end_coords.y == (self.piece.coords.y as isize + y_move) as usize
                    || (self.piece.is_home_row()
                        && self.end_coords.y
                            == (self.piece.coords.y as isize + (y_move * 2)) as usize))
                    && self.end_coords.x == self.piece.coords.x
                    && game[self.end_coords].is_empty())) || // Capture
                    (self.is_capture && (self.end_coords.y == (self.piece.coords.y as isize + y_move) as usize
                        && (self.end_coords.x as isize
                            - self.piece.coords.x as isize)
                            .abs()
                            == 1
                        && !game[self.end_coords].is_empty() && game[self.end_coords].color() != game.turn))
            }
            0b10 => {
                println!("Rook");
                println!(
                    "vertical: {}, horizontal: {}, empty: {}, is_path: {}",
                    (self.end_coords.y != self.piece.coords.y
                        && self.end_coords.x == self.piece.coords.x),
                    (self.end_coords.y == self.piece.coords.y
                        && self.end_coords.x != self.piece.coords.x),
                    game[self.end_coords].is_empty(),
                    game.is_path(
                        PiecePath::Straight,
                        self.piece.coords.into(),
                        self.end_coords,
                    )
                );

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
            0b11 => true, // The check for the knight is doen in the move purging
            0b100 => {
                println!("Bishop");
                println!(
                    "empty: {}, is_path: {}",
                    game[self.end_coords].is_empty(),
                    game.is_path(
                        PiecePath::Diagonal,
                        self.piece.coords.into(),
                        self.end_coords,
                    )
                );

                game[self.end_coords].is_empty()
                    && game.is_path(PiecePath::Diagonal, self.piece.coords, self.end_coords)
            }
            _ => unreachable!(),
        }
    }
}

pub enum PiecePath {
    Straight,
    Diagonal,
}
