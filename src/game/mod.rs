use crate::*;

mod default;
mod index_extensions;

#[derive(Debug)]
pub struct Game {
    pub board: [[Piece; 8]; 8],
    pub turn: u8,
    pub check: (bool, bool),
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

impl Game {
    pub fn make_move(&mut self, coords: Coords) -> Result<(), String> {
        let previous_position = match PieceMove::from_piece_type(coords, self, false, self.turn) {
            Ok(x) => x,
            Err(e) => {
                return Err(e);
            }
        };

        if !previous_position.is_valid(self, None) {
            return Err("Invalid move".to_string());
        }

        let color_mask = if self.turn == 1 { 0b1000 } else { 0 };
        let piece = Piece::from_coords((coords.piece as u8) ^ color_mask, coords);

        self[previous_position.piece.coords.y][previous_position.piece.coords.x] = Piece::empty();
        self[coords.y][coords.x] = piece;

        if piece.piece_type() == (PieceType::King as u8) {
            self.king_positions[self.turn as usize] = (coords.x, coords.y);
        }

        Ok(())
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
                    for y in range.step_by(1) {
                        if !self[y][start_coords.x].is_empty() {
                            return false;
                        }
                    }
                    return true;
                } else if start_coords.y == end_coords.y {
                    let range = if start_coords.x < end_coords.x {
                        start_coords.x + 1..end_coords.x
                    } else {
                        end_coords.x + 1..start_coords.x
                    };
                    for x in range.step_by(1) {
                        if !self[start_coords.y][x].is_empty() {
                            return false;
                        }
                    }
                    return true;
                }
                false
            }
            PiecePath::Diagonal => {
                if start_coords.x != end_coords.x && start_coords.y != end_coords.y {
                    let (mut x, mut y) =
                        match (start_coords.x < end_coords.x, start_coords.y < end_coords.y) {
                            (true, true) => (start_coords.x + 1, start_coords.y + 1),
                            (true, false) => (
                                start_coords.x + 1,
                                start_coords.y.checked_sub(1).unwrap_or(9),
                            ),
                            (false, true) => (
                                start_coords.x.checked_sub(1).unwrap_or(9),
                                start_coords.y + 1,
                            ),
                            (false, false) => (
                                start_coords.x.checked_sub(1).unwrap_or(9),
                                start_coords.y.checked_sub(1).unwrap_or(9),
                            ),
                        };
                    while x != end_coords.x && y != end_coords.y {
                        if x >= 8 || y >= 8 {
                            return false;
                        }
                        if !self[y][x].is_empty() {
                            return false;
                        }
                        x = match (start_coords.x < end_coords.x, x < end_coords.x) {
                            (true, true) => x + 1,
                            (true, false) => x.checked_sub(1).unwrap_or(9),
                            (false, true) => x + 1,
                            (false, false) => x.checked_sub(1).unwrap_or(9),
                        };
                        y = match (start_coords.y < end_coords.y, y < end_coords.y) {
                            (true, true) => y + 1,
                            (true, false) => y.checked_sub(1).unwrap_or(9),
                            (false, true) => y.checked_sub(1).unwrap_or(9),
                            (false, false) => y.checked_sub(1).unwrap_or(9),
                        };
                    }
                    return true;
                }
                false
            }
        }
    }

    pub fn is_king_in_check(&mut self) -> Vec<bool> {
        [0, 1]
            .iter()
            .map(|color| {
                PieceType::iter().any(|piece_type| {
                    self.get_check_move_coords(piece_type)
                        .iter()
                        .any(|coords| self.is_attacked(*coords, *color))
                })
            })
            .collect::<Vec<bool>>()
    }

    fn get_check_move_coords(&self, piece_type: PieceType) -> Vec<Coords> {
        vec![
            Coords {
                piece: piece_type,
                x: self.king_positions[0].0,
                y: self.king_positions[0].1,
                is_capture: true,
            },
            Coords {
                piece: piece_type,
                x: self.king_positions[1].0,
                y: self.king_positions[1].1,
                is_capture: true,
            },
        ]
    }

    fn is_attacked(&self, piece_coords: Coords, color: u8) -> bool {
        println!("piece_coords: {:?}", piece_coords);
        let attacker_move = match PieceMove::from_piece_type(piece_coords, self, true, color) {
            Ok(x) => x,
            Err(e) => {
                println!("{}", e);
                return false;
            }
        };
        println!("attacker_move: {:?}", attacker_move);
        if attacker_move.is_valid(self, None) {
            return true;
        }
        false
    }
}
