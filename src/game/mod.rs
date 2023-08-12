use crate::*;

pub mod checks;
pub mod default;
mod index_extensions;

#[derive(Debug, PartialEq)]
pub struct Game {
    board: [[Piece; 8]; 8],
    pub turn: u8,
    pub check: Checks,
    pub king_positions: [Position; 2],
    pub moves: Moves,
    pub hash_history: Vec<[u64; 4]>,
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

pub enum GameResult {
    Win(u8),
    Draw,
    InProgress(Result<(), Box<dyn Error>>),
}

impl GameResult {
    pub fn unwrap_err_as_string(&self) -> String {
        match self {
            InProgress(r) =>
                match r {
                    Ok(_) => panic!("GameResult::unwrap_error called on non-error"),
                    Err(e) => e.to_string(),
                }
            _ => panic!("GameResult::unwrap_error called on non-error"),
        }
    }
}

type MoveResult = Result<(), String>;

impl Game {
    pub fn make_pmove(&mut self, mut pmove: PMove) -> MoveResult {
        if !pmove.fill_start_position(self) {
            return Err("Invalid move".to_string());
        }

        self.piece_pmove(pmove);
        if pmove.piece_type() == PieceType::King {
            self.update_king_pos(pmove.end_position());
        }

        self.moves.push(pmove);
        self.hash_history.push(self.serialize_to_ints());
        self.check = self.is_king_in_check();
        self.turn ^= 1;
        Ok(())
    }

    fn piece_pmove(&mut self, pmove: PMove) {
        if pmove.is_capture() && self[pmove.end_position()].is_empty() {
            self.empty(pmove.end_position().with_y(pmove.start_position().y));
        }

        let final_piece = Piece::from_position(pmove.byte(self), pmove.end_position());
        if
            let Some((king_pos, rook_pos)) =
                CASTLING[(self.turn * 2 + pmove.castling() * 3) as usize]
        {
            let (king_mask, rook_mask) = match self.turn {
                0 => (0b0110, 0b0010),
                1 => (0b1110, 0b1010),
                _ => unreachable!(),
            };
            self[king_pos] = Piece::from_position(king_mask, king_pos.into());
            self[rook_pos] = Piece::from_position(rook_mask, rook_pos.into());
        } else {
            self[pmove.end_position()] = final_piece;
            self.empty(pmove.start_position());
        }
    }

    pub fn castling_allowed(&self, king_pos: (usize, usize), rook_pos: (usize, usize)) -> bool {
        self.position_attacked(king_pos.into()) ||
            self.position_attacked(rook_pos.into()) ||
            self.is_king_in_check()[self.turn]
    }

    fn update_king_pos(&mut self, king_pos: Position) {
        self.king_positions[self.turn as usize] = king_pos;
    }

    fn empty<T>(&mut self, position: T) where T: Into<Position> + Copy {
        self[position.into()] = Piece::empty(position.into());
    }

    pub fn is_path(
        &self,
        path_type: PiecePath,
        start_position: Position,
        end_position: Position
    ) -> bool {
        match path_type {
            PiecePath::Straight => {
                if start_position.x == end_position.x {
                    let range = if start_position.y < end_position.y {
                        start_position.y + 1..end_position.y
                    } else {
                        end_position.y + 1..start_position.y
                    };
                    return range.step_by(1).all(|y| self[(start_position.x, y)].is_empty());
                } else if start_position.y == end_position.y {
                    let range = if start_position.x < end_position.x {
                        start_position.x + 1..end_position.x
                    } else {
                        end_position.x + 1..start_position.x
                    };
                    return range.step_by(1).all(|x| self[(x, start_position.y)].is_empty());
                }
                false
            }
            PiecePath::Diagonal => {
                if start_position.x == end_position.x || start_position.y == end_position.y {
                    return false;
                }
                let (step_x, step_y) = (
                    if start_position.x < end_position.x { 1 } else { -1 },
                    if start_position.y < end_position.y { 1 } else { -1 },
                );
                let (mut x, mut y) = (
                    (start_position.x as isize) + step_x,
                    (start_position.y as isize) + step_y,
                );
                while
                    (x, y) != end_position.into() &&
                    Game::in_bounds((x as usize, y as usize).into())
                {
                    if !self[(x as usize, y as usize)].is_empty() {
                        return false;
                    }
                    x += step_x;
                    y += step_y;
                }
                true
            }
        }
    }

    pub fn is_king_in_check(&self) -> Checks {
        self.king_positions
            .iter()
            .map(|pos| self.position_attacked(*pos))
            .collect()
    }

    pub fn position_attacked(&self, end_position: Position) -> bool {
        PieceType::iter().any(|p_type| {
            let mut p_move = PMove::partial(0, end_position.to_byte(), p_type as u8, true, 0, 0);
            p_move.fill_start_position(self)
        })
    }

    pub fn in_bounds(pos: Position) -> bool {
        pos.x <= 7 && pos.y <= 7
    }

    pub fn get_board(&self) -> &[[Piece; 8]; 8] {
        &self.board
    }

    pub fn check_draw(&self) -> bool {
        // Check if only pieces left are kings
        if
            self.board
                .iter()
                .flat_map(|row| row.iter())
                .all(|piece| (piece.is_empty() || piece.piece_type() == (PieceType::King as u8)))
        {
            return true;
        }

        // Check if same position has been repeated 3 times
        if
            self.hash_history
                .iter()
                .filter(|pos| **pos == self.serialize_to_ints())
                .count() >= 3
        {
            return true;
        }

        // Check if 50 moves have been made without a capture or pawn move
        let last_50 = self.moves.last_50();
        if
            last_50.len() == 50 &&
            last_50.iter().all(|x| x.piece_type() != PieceType::Pawn) &&
            last_50.iter().all(|x| !x.is_capture())
        {
            return true;
        }

        // TODO: Stalemate
        if self.get_valid_moves().len() == 0 && !self.check_win() {
            return true;
        }

        false
    }

    pub fn check_win(&self) -> bool {
        self.get_valid_moves().len() == 0 && self.check[self.turn as usize]
    }

    pub fn get_valid_moves(&self) -> Vec<PMove> {
        let mut moves = Vec::new();
        for y in 0..8usize {
            for x in 0..8 {
                if self[(x, y)].is_empty() || self[(x, y)].color() != self.turn {
                    continue;
                }

                let piece_moves = PMove::for_piece((x, y).into(), self[(x, y)], self);
                moves.extend(piece_moves);
            }
        }
        moves
    }

    pub fn serialize_to_ints(&self) -> [u64; 4] {
        let mut ints = [0u64; 4];
        for (i, row) in self.board.iter().enumerate() {
            for (j, piece) in row.iter().enumerate() {
                ints[i / 2] |= (piece.byte as u64) << (64 - ((j + 1) * 4 + ((i + 1) % 2) * 32));
            }
        }
        ints
    }
}
