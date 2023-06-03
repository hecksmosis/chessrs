use crate::*;
pub use inverse_moves::*;
pub use piece_move::*;

mod inverse_moves;
mod piece_move;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct Piece {
    pub byte: u8,
    pub coords: Coords,
}

impl Piece {
    pub fn new(byte: u8, coords: (usize, usize)) -> Self {
        Piece {
            byte,
            coords: coords.into(),
        }
    }

    pub fn from_coords(byte: u8, coords: Coords) -> Self {
        Piece { byte, coords }
    }

    pub fn is_empty(&self) -> bool {
        self.byte == 0
    }

    pub fn color(&self) -> u8 {
        ((self.byte & 0b1000) != 0) as u8
    }

    pub fn piece_type(&self) -> u8 {
        self.byte & 0b0111
    }

    pub fn is_home_row(&self) -> bool {
        (self.coords.y == 1 && self.byte == 0b0001) || (self.coords.y == 6 && self.byte == 0b1001)
    }

    pub fn empty() -> Self {
        Piece {
            byte: 0,
            coords: Coords::default(),
        }
    }
}

impl Display for Piece {
    fn fmt(&self, f: &mut Formatter) -> fmtResult {
        let piece = match self.byte {
            0b0000 => ' ',
            0b0001 => '♟',
            0b0010 => '♜',
            0b0011 => '♞',
            0b0100 => '♝',
            0b0101 => '♛',
            0b0110 => '♚',
            0b1001 => '♙',
            0b1010 => '♖',
            0b1011 => '♘',
            0b1100 => '♗',
            0b1101 => '♕',
            0b1110 => '♔',
            _ => unreachable!(),
        };

        write!(f, "{}", piece)
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum PieceType {
    Pawn = 0b1,
    Rook = 0b10,
    Knight = 0b11,
    Bishop = 0b100,
    Queen = 0b101,
    King = 0b110,
    None = 0,
}

impl PieceType {
    pub fn iter() -> impl Iterator<Item = PieceType> {
        // Define the order in which the pieces should be iterated
        static PIECE_TYPES: [PieceType; 6] = [
            PieceType::Pawn,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Rook,
            PieceType::Queen,
            PieceType::King,
        ];

        PIECE_TYPES.iter().cloned()
    }
}
