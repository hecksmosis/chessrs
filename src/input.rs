use crate::{ PieceType, Position, PMove };

#[derive(Debug, Copy, Clone)]
pub struct Input {
    pub piece_type: PieceType,
    pub is_capture: bool,
    pub end_position: Position,
    pub castling: u8,
    pub promotion: Option<PieceType>,
}

impl Input {
    pub fn build(s: &str) -> Result<Input, String> {
        if s.len() > 6 || s.len() < 2 {
            return Err("Invalid input".to_string());
        }

        if s == "O-O" || s == "O-O-O" {
            return Ok(Input {
                piece_type: PieceType::King,
                is_capture: false,
                end_position: (0usize, 0).into(),
                castling: if s == "O-O" {
                    1
                } else {
                    2
                },
                promotion: None,
            });
        }

        let mut chars = s.chars();
        println!("{}", s.len());
        let piece_type = if s.len() == 2 {
            PieceType::Pawn
        } else {
            match chars.next().unwrap() {
                'p' => PieceType::Pawn,
                'r' => PieceType::Rook,
                'n' => PieceType::Knight,
                'b' => PieceType::Bishop,
                'q' => PieceType::Queen,
                'k' => PieceType::King,
                _ => {
                    return Err("Invalid piece".to_string());
                }
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
            (s.len() == 6 && is_capture) ||
            (!is_capture && (s.len() == 5 || (s.len() == 4 && piece_type == PieceType::Pawn)))
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

        println!(
            "{:b}",
            PMove::partial(
                0,
                ((x << 3) | y) as u8,
                piece_type as u8,
                is_capture,
                0,
                promotion.map(|x| x as u8).unwrap_or(0)
            ).0
        );

        Ok(Input {
            piece_type,
            is_capture,
            end_position: (x, y).into(),
            castling: 0,
            promotion,
        })
    }
}
