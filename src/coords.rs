use std::fmt::Display;

use crate::PieceType;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Coords {
    pub piece: PieceType,
    pub is_capture: bool,
    pub x: usize,
    pub y: usize,
}

impl Display for Coords {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let x = (self.x as u8 + 'a' as u8) as char;
        let y = (self.y as u8 + '1' as u8) as char;

        write!(f, "{}{}", x, y)
    }
}

impl Into<(usize, usize)> for Coords {
    fn into(self) -> (usize, usize) {
        (self.x, self.y)
    }
}

impl From<(usize, usize)> for Coords {
    fn from(tuple: (usize, usize)) -> Self {
        Coords {
            piece: PieceType::None,
            is_capture: false,
            x: tuple.0,
            y: 7 - tuple.1,
        }
    }
}

impl Coords {
    pub fn new(x: usize, y: usize) -> Self {
        Coords {
            piece: PieceType::None,
            is_capture: false,
            x,
            y: 7 - y,
        }
    }
    pub fn from_raw(x: usize, y: usize) -> Self {
        Coords {
            piece: PieceType::None,
            is_capture: false,
            x,
            y,
        }
    }

    pub fn build(s: &str) -> Result<Coords, String> {
        if s.len() > 4 || s.len() < 2 {
            return Err("Invalid input".to_string());
        }

        let mut chars = s.chars();
        let piece = if s.len() == 2 {
            PieceType::Pawn
        } else {
            match chars.next().unwrap() {
                'p' => PieceType::Pawn,
                'r' => PieceType::Rook,
                'n' => PieceType::Knight,
                'b' => PieceType::Bishop,
                'q' => PieceType::Queen,
                'k' => PieceType::King,
                _ => return Err("Invalid piece".to_string()),
            }
        };
        let is_capture = if s.len() > 3 {
            match chars.next().unwrap() {
                'x' => true,
                _ => false,
            }
        } else {
            false
        };
        let x = match chars.next().unwrap() {
            x @ 'a'..='h' => x as usize - 'a' as usize,
            _ => return Err("Invalid x coordinate".to_string()),
        };
        let y = match chars.next().unwrap() {
            y @ '1'..='8' => y as usize - '1' as usize,
            _ => return Err("Invalid y coordinate".to_string()),
        };

        println!("x: {}, y: {}", x, y);

        Ok(Coords {
            piece,
            is_capture,
            x,
            y,
        })
    }

    pub fn default() -> Self {
        Coords {
            piece: PieceType::None,
            is_capture: false,
            x: 9,
            y: 9,
        }
    }
}
