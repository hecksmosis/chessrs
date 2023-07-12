use crate::*;

pub const CASTLING: [Option<((usize, usize), (usize, usize))>; 9] = [
    None,
    None,
    None,
    Some(((6, 0), (5, 0))),
    None,
    Some(((6, 7), (5, 7))),
    Some(((2, 0), (3, 0))),
    None,
    Some(((2, 7), (3, 7))),
];

pub const PROMOTIONS: [PieceType; 5] = [
    PieceType::None,
    PieceType::Rook,
    PieceType::Knight,
    PieceType::Bishop,
    PieceType::Queen,
];

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct PMove(pub u32);

impl PMove {
    pub fn partial(
        start_position: u8,
        end_position: u8,
        piece_type: u8,
        capture: bool,
        castling: u8,
        promotion: u8
    ) -> Self {
        Self(
            (end_position as u32) |
                ((start_position as u32) << 6) |
                ((piece_type as u32) << 12) |
                ((capture as u32) << 15) |
                ((castling as u32) << 16) |
                ((promotion as u32) << 18)
        )
    }

    pub fn for_piece(position: Position, piece: Piece, game: &Game) -> Vec<Self> {
        match piece.piece_type().into() {
            PieceType::Pawn => {
                (
                    if game.turn == 0 {
                        POSSIBLE_WHITE_PAWN_MOVES.iter()
                    } else {
                        POSSIBLE_BLACK_PAWN_MOVES.iter()
                    }
                )
                    .filter(|m| m.check(position, game))
                    .map(|m|
                        PMove::partial(
                            position.to_byte(),
                            (position + m.diff).to_byte(),
                            piece.piece_type(),
                            m.capture,
                            0,
                            101
                        )
                    )
                    .collect()
            }
            PieceType::Rook => {
                POSSIBLE_ROOK_MOVES.iter()
                    .filter(|m| m.check(position, game))
                    .map(|m|
                        PMove::partial(
                            position.to_byte(),
                            (position + m.diff).to_byte(),
                            piece.piece_type(),
                            m.capture,
                            0,
                            0
                        )
                    )
                    .collect()
            }
            _ => Vec::new(),
        }
    }

    pub fn castle(castling: u8) -> Self {
        Self(((PieceType::King as u32) << 12) | ((castling as u32) << 16))
    }

    pub fn end_position(&self) -> Position {
        Position::from_byte((self.0 & 0b111111) as u8)
    }

    pub fn start_position(&self) -> Position {
        Position::from_byte((self.0 & (0b111111 << 6)) as u8)
    }

    pub fn piece_type_raw(&self) -> u8 {
        ((self.0 & (0b111 << 12)) >> 12) as u8
    }

    pub fn piece_type(&self) -> PieceType {
        PieceType::from(self.piece_type_raw())
    }

    pub fn is_capture(&self) -> bool {
        (self.0 & (1 << 15)) != 0
    }

    pub fn castling(&self) -> u8 {
        ((self.0 & (0b11 << 16)) >> 16) as u8
    }

    pub fn promotion(&self) -> PieceType {
        PieceType::from(((self.0 & (0b111 << 18)) >> 18) as u8)
    }

    pub fn byte(&self, Game { turn, .. }: &Game) -> u8 {
        if self.promotion() != PieceType::None {
            return (turn << 3) + (self.promotion() as u8);
        }
        (turn << 3) + self.piece_type_raw()
    }

    pub fn fill_start_position(&mut self, game: &Game) -> bool {
        match &self.piece_type() {
            PieceType::Pawn => {
                let moves = (
                    if game.turn == 0 {
                        WHITE_PAWN_MOVES.iter()
                    } else {
                        BLACK_PAWN_MOVES.iter()
                    }
                )
                    .filter(|m| { m.check(self, game) })
                    .collect::<Vec<_>>();
                match moves.len() {
                    1 => {
                        self.0 |= ((self.end_position() - moves[0].diff).to_byte() as u32) << 6;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            PieceType::Knight => {
                let moves = KNIGHT_MOVES.iter()
                    .filter(|m| { m.check(self, game) })
                    .collect::<Vec<_>>();
                match moves.len() {
                    1 => {
                        self.0 |= ((self.end_position() - moves[0].diff).to_byte() as u32) << 6;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            PieceType::Rook => {
                let moves = ROOK_MOVES.iter()
                    .filter(|m| { m.check(self, game) })
                    .collect::<Vec<_>>();
                match moves.len() {
                    1 => {
                        self.0 |= ((self.end_position() - moves[0].diff).to_byte() as u32) << 6;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            PieceType::Bishop => {
                let moves = BISHOP_MOVES.iter()
                    .filter(|m| { m.check(self, game) })
                    .collect::<Vec<_>>();
                match moves.len() {
                    1 => {
                        self.0 |= ((self.end_position() - moves[0].diff).to_byte() as u32) << 6;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            PieceType::Queen => {
                let moves = BISHOP_MOVES.iter()
                    .chain(ROOK_MOVES.iter())
                    .filter(|m| { m.check(self, game) })
                    .collect::<Vec<_>>();
                match moves.len() {
                    1 => {
                        self.0 |= ((self.end_position() - moves[0].diff).to_byte() as u32) << 6;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            PieceType::King if self.castling() != 0 => {
                if
                    let Some((king_pos, rook_pos)) =
                        CASTLING[(game.turn * 2 + self.castling() * 3) as usize]
                {
                    if
                        game.position_attacked(king_pos.into()) ||
                        game.position_attacked(rook_pos.into()) ||
                        game.is_king_in_check()[game.turn]
                    {
                        return false;
                    }
                    return true;
                }
                return false;
            }
            PieceType::King => {
                let moves = KING_MOVES.iter()
                    .filter(|m| { m.check(self, game) })
                    .collect::<Vec<_>>();
                match moves.len() {
                    1 => {
                        self.0 |= ((self.end_position() - moves[0].diff).to_byte() as u32) << 6;
                    }
                    _ => {
                        return false;
                    }
                }
            }
            _ => todo!(),
        }
        true
    }

    pub fn from_input(input: &str) -> Result<Self, String> {
        if input.len() < 2 || input.len() > 6 {
            return Err("Invalid input".to_string());
        }

        if input == "O-O" {
            return Ok(PMove::castle(1));
        } else if input == "O-O-O" {
            return Ok(PMove::castle(2));
        }

        let mut chars = input.chars();
        let piece_type = match chars.next().unwrap() {
            'p' => PieceType::Pawn,
            'r' => PieceType::Rook,
            'n' => PieceType::Knight,
            'b' => PieceType::Bishop,
            'q' => PieceType::Queen,
            'k' => PieceType::King,
            _ => {
                return Err("Invalid piece".to_string());
            }
        };

        let is_capture = if input.len() > 3 {
            match chars.next().unwrap() {
                'x' => true,
                _ => false,
            }
        } else {
            false
        };

        let x = match chars.next().unwrap() {
            x @ 'a'..='h' => (x as usize) - ('a' as usize),
            _ => {
                return Err("Invalid x coordinate".to_string());
            }
        };
        let y = match chars.next().unwrap() {
            y @ '1'..='8' => (y as usize) - ('1' as usize),
            _ => {
                return Err("Invalid y coordinate".to_string());
            }
        };

        let promotion = if
            (input.len() == 6 && is_capture) ||
            (!is_capture &&
                (input.len() == 5 || (input.len() == 4 && piece_type == PieceType::Pawn)))
        {
            chars.next().unwrap();
            match chars.next().unwrap() {
                'q' => Some(PieceType::Queen),
                'r' => Some(PieceType::Rook),
                'n' => Some(PieceType::Knight),
                'b' => Some(PieceType::Bishop),
                _ => {
                    return Err("Invalid promotion".to_string());
                }
            }
        } else {
            None
        };

        let partial_move = PMove::partial(
            0,
            ((x << 3) | y) as u8,
            piece_type as u8,
            is_capture,
            0,
            promotion.map(|x| x as u8).unwrap_or(0)
        );

        println!("{:b}", partial_move.0);

        Ok(partial_move)
    }
}
