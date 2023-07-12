use crate::*;
pub use inverse_moves::*;
pub use moves::*;
pub use piece_move::*;
pub use bin_move::*;
pub use move_lists::*;
pub use direct_move_lists::*;

mod inverse_moves;
mod moves;
mod piece_move;
mod move_lists;
mod direct_move_lists;
mod bin_move;

#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd)]
pub struct Piece {
    pub byte: u8,
    pub position: Position,
}

impl Piece {
    pub fn new(byte: u8, position: (usize, usize)) -> Self {
        Piece {
            byte,
            position: position.into(),
        }
    }

    pub fn from_position(byte: u8, position: Position) -> Self {
        Piece { byte, position }
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
        (self.position.y == 1 && self.byte == 0b0001) ||
            (self.position.y == 6 && self.byte == 0b1001)
    }

    pub fn empty(position: Position) -> Self {
        Piece { byte: 0, position }
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

impl Value for Piece {
    fn value(&self) -> i32 {
        match self.byte {
            0b0001 => 1,
            0b0010 => 5,
            0b0011 => 3,
            0b0100 => 3,
            0b0101 => 9,
            0b0110 => 1000,
            0b1001 => -1,
            0b1010 => -5,
            0b1011 => -3,
            0b1100 => -3,
            0b1101 => -9,
            0b1110 => -1000,
            _ => 0,
        }
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

impl From<u8> for PieceType {
    fn from(byte: u8) -> Self {
        match byte & 0b0111 {
            0b0001 => PieceType::Pawn,
            0b0010 => PieceType::Rook,
            0b0011 => PieceType::Knight,
            0b0100 => PieceType::Bishop,
            0b0101 => PieceType::Queen,
            0b0110 => PieceType::King,
            _ => PieceType::None,
        }
    }
}
