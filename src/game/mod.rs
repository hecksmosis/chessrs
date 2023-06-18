use crate::*;

pub mod checks;
mod default;
mod index_extensions;

#[derive(Debug)]
pub struct Game {
    board: [[Piece; 8]; 8],
    pub turn: u8,
    pub check: Checks,
    pub king_positions: [(usize, usize); 2],
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        for row in self.board.iter() {
            for piece in row.iter() {
                write!(f, "{}", piece)?;
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

type MoveResult = Result<(), String>;

impl Game {
    pub fn make_move(&mut self, coords: Coords) -> MoveResult {
        let previous_position = match PieceMove::from_piece_type(coords, self, false, self.turn) {
            Ok(x) => x,
            Err(e) => {
                return Err(e);
            }
        };

        if !previous_position.is_valid(self, None) {
            return Err("Invalid move".to_string());
        }

        let piece = self.move_piece(&previous_position, &coords);
        if self.is_king_in_check()[self.turn] {
            self.revert_move(&previous_position, &coords, piece);
            return Err("Invalid move, king in check".to_string());
        }

        Ok(())
    }

    fn move_piece(&mut self, previous_position: &PieceMove, coords: &Coords) -> Piece {
        let color_mask = if self.turn == 1 { 0b1000 } else { 0 };
        let piece = Piece::from_coords((coords.piece as u8) ^ color_mask, *coords);

        self[previous_position.piece.coords.y][previous_position.piece.coords.x] = Piece::empty();
        self[coords.y][coords.x] = piece;

        if piece.piece_type() == (PieceType::King as u8) {
            self.king_positions[self.turn as usize] = (coords.x, coords.y);
        }

        piece
    }

    fn revert_move(&mut self, previous_position: &PieceMove, coords: &Coords, piece: Piece) {
        self[previous_position.piece.coords.y][previous_position.piece.coords.x] =
            previous_position.piece;
        self[coords.y][coords.x] = Piece::empty();

        if piece.piece_type() == PieceType::King as u8 {
            self.king_positions[self.turn as usize] = (
                previous_position.piece.coords.x,
                previous_position.piece.coords.y,
            );
        }
    }

    pub fn is_path(&self, path_type: PiecePath, start_coords: Coords, end_coords: Coords) -> bool {
        match path_type {
            PiecePath::Straight => {
                if start_coords.x == end_coords.x {
                    let range = if start_coords.y < end_coords.y {
                        start_coords.y + 1..end_coords.y
                    } else {
                        end_coords.y + 1..start_coords.y
                    };
                    return range.step_by(1).all(|y| self[y][start_coords.x].is_empty());
                } else if start_coords.y == end_coords.y {
                    let range = if start_coords.x < end_coords.x {
                        start_coords.x + 1..end_coords.x
                    } else {
                        end_coords.x + 1..start_coords.x
                    };
                    return range.step_by(1).all(|x| self[start_coords.y][x].is_empty());
                }
                false
            }
            PiecePath::Diagonal => {
                if start_coords.x == end_coords.x || start_coords.y == end_coords.y {
                    return false;
                }
                let (step_x, step_y) = (
                    if start_coords.x < end_coords.x { 1 } else { -1 },
                    if start_coords.y < end_coords.y { 1 } else { -1 },
                );
                let (mut x, mut y) = (
                    start_coords.x as isize + step_x,
                    start_coords.y as isize + step_y,
                );
                while x != end_coords.x as isize
                    && y != end_coords.y as isize
                    && Game::in_bounds(x, y)
                {
                    if !self[y as usize][x as usize].is_empty() {
                        return false;
                    }
                    x += step_x;
                    y += step_y;
                }
                true
            }
        }
    }

    pub fn is_king_in_check(&mut self) -> Checks {
        [0, 1]
            .iter()
            .map(|color| {
                PieceType::iter().any(|piece_type| {
                    self.is_attacked(self.get_check_move_coords(piece_type, *color), *color)
                })
            })
            .collect::<Checks>()
    }

    fn get_check_move_coords(&self, piece_type: PieceType, color: u8) -> Coords {
        let (x, y) = if color == 0 {
            self.king_positions[0]
        } else {
            self.king_positions[1]
        };
        Coords {
            piece: piece_type,
            x,
            y,
            is_capture: true,
        }
    }

    fn is_attacked(&self, piece_coords: Coords, color: u8) -> bool {
        PieceMove::from_piece_type(piece_coords, self, true, color)
            .map(|attacker_move| attacker_move.is_valid(self, None))
            .unwrap_or(false)
    }

    fn in_bounds<T>(x: T, y: T) -> bool
    where
        T: Into<isize> + Copy,
    {
        x.into() >= 0 && x.into() <= 7 && y.into() >= 0 && y.into() <= 7
    }
}
