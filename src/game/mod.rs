use crate::*;

mod default;
mod index_extensions;

#[derive(Debug)]
pub struct Game {
    pub board: [[Piece; 8]; 8],
    pub turn: u8,
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
        // First, we must infer the starting piece position from the end position
        match coords.piece {
            PieceType::Pawn => {
                let previous_position =
                    match PieceMove::from_piece_type(PieceType::Pawn, coords, self) {
                        Ok(x) => x,
                        Err(e) => return Err(e),
                    };

                if !previous_position.is_valid(self) {
                    return Err("AAB: Invalid move".to_string());
                }

                self[previous_position.piece.coords.y][previous_position.piece.coords.x] =
                    Piece::empty();
                println!("turn {}", self.turn);
                let color_mask = if self.turn == 1 { 0b1000 } else { 0 };
                self[coords.y][coords.x] =
                    Piece::from_coords(coords.piece as u8 ^ color_mask, coords);
            }
            PieceType::Rook => {
                let previous_position =
                    match PieceMove::from_piece_type(PieceType::Rook, coords, self) {
                        Ok(x) => x,
                        Err(e) => return Err(e),
                    };

                self[previous_position.piece.coords.y][previous_position.piece.coords.x] =
                    Piece::empty();
                println!("turn {}", self.turn);
                let color_mask = if self.turn == 1 { 0b1000 } else { 0 };
                self[coords.y][coords.x] =
                    Piece::from_coords(coords.piece as u8 ^ color_mask, coords);
            }
            PieceType::Knight => {
                let previous_position =
                    match PieceMove::from_piece_type(PieceType::Knight, coords, self) {
                        Ok(x) => x,
                        Err(e) => return Err(e),
                    };

                self[previous_position.piece.coords.x][previous_position.piece.coords.y] =
                    Piece::empty();
                println!("turn {}", self.turn);
                let color_mask = if self.turn == 1 { 0b1000 } else { 0 };
                self[coords.y][coords.x] =
                    Piece::from_coords(coords.piece as u8 ^ color_mask, coords);
            }
            PieceType::Bishop => {
                // Can move any number of squares diagonally
                // Check if there are any pieces in the way
                let previous_position =
                    match PieceMove::from_piece_type(PieceType::Bishop, coords, self) {
                        Ok(x) => x,
                        Err(e) => return Err(e),
                    };

                self[previous_position.piece.coords.x][previous_position.piece.coords.y] =
                    Piece::empty();
                println!("turn {}", self.turn);
                let color_mask = if self.turn == 1 { 0b1000 } else { 0 };
                self[coords.y][coords.x] =
                    Piece::from_coords(coords.piece as u8 ^ color_mask, coords);
            }
            _ => todo!(),
        }
        Ok(())
    }

    pub fn is_path(&self, path_type: PiecePath, start_coords: Coords, end_coords: Coords) -> bool {
        match path_type {
            PiecePath::Straight => {
                println!("{:?} -> {:?}", start_coords, end_coords);
                if start_coords.x == end_coords.x {
                    println!("moving in y");
                    println!("{}", self);
                    let mut y = if start_coords.y < end_coords.y {
                        start_coords.y + 1
                    } else {
                        start_coords.y - 1
                    };
                    println!("y: {}", y);
                    while y != end_coords.y {
                        println!(
                            "Piece: {}, at ({}, {})",
                            self[y][start_coords.x], start_coords.x, y
                        );
                        if !self[y][start_coords.x].is_empty() {
                            return false;
                        }
                        y = if start_coords.y < end_coords.y {
                            y + 1
                        } else {
                            y - 1
                        };
                    }
                    return true;
                } else if start_coords.y == end_coords.y {
                    println!("moving in x");
                    println!("{}", self);
                    let mut x = if start_coords.x < end_coords.x {
                        start_coords.x + 1
                    } else {
                        start_coords.x - 1
                    };
                    println!("y: {}", x);
                    while x != end_coords.y {
                        println!(
                            "Piece: {}, at ({}, {})",
                            self[start_coords.y][x], x, start_coords.y
                        );
                        if !self[start_coords.y][x].is_empty() {
                            return false;
                        }
                        x = if start_coords.x < end_coords.x {
                            x + 1
                        } else {
                            x - 1
                        };
                    }
                    return true;
                }
                false
            }
            PiecePath::Diagonal => {
                println!("{:?} -> {:?}", start_coords, end_coords);
                if start_coords.x == end_coords.x {
                    println!("moving in xy");
                    println!("{}", self);
                    let mut y = if start_coords.y < end_coords.y {
                        start_coords.y + 1
                    } else {
                        start_coords.y - 1
                    };
                    let mut x = if start_coords.y < end_coords.y {
                        start_coords.y + 1
                    } else {
                        start_coords.y - 1
                    };
                    println!("y: {}", y);
                    while y != end_coords.y {
                        println!("Piece: {}, at ({}, {})", self[y][x], x, y);
                        if !self[y][x].is_empty() {
                            return false;
                        }
                        y = if start_coords.y < end_coords.y {
                            y + 1
                        } else {
                            y - 1
                        };
                        x = if start_coords.y < end_coords.y {
                            y + 1
                        } else {
                            y - 1
                        };
                    }
                    return true;
                }
                false
            }
        }
    }
}