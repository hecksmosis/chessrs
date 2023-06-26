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
    pub fn make_move(&mut self, input: Input) -> MoveResult {
        let previous_position = match PieceMove::from_piece_type(input, self, false, self.turn) {
            Ok(x) => {
                println!("{:?}", x);
                x
            }
            Err(e) => {
                return Err(e);
            }
        };

        if !previous_position.is_valid(self, input.piece_type) {
            return Err("Invalid move".to_string());
        }

        let piece = self.move_piece(&previous_position, &input);
        if self.is_king_in_check()[self.turn] {
            self.revert_move(&previous_position, &input, piece);
            return Err("Invalid move, king in check".to_string());
        } else if input.castling != 0
            && PieceType::iter().any(|piece_type| {
                self.is_attacked(
                    Input {
                        piece_type,
                        is_capture: true,
                        end_position: self.get_rook_pos(&input),
                        castling: 0,
                    },
                    self.turn,
                )
            })
        {
            self.revert_move(&previous_position, &input, piece);
            return Err("Invalid move, king in check".to_string());
        }

        self.moves.push(previous_position);
        self.check = self.is_king_in_check();
        self.turn ^= 1;
        Ok(())
    }

    fn empty<T>(&mut self, position: T)
    where
        T: Into<Position> + Copy,
    {
        self[position.into()] = Piece::empty(position.into());
    }

    fn move_piece(&mut self, previous_move: &PieceMove, input: &Input) -> Piece {
        println!(
            "Coords: {}, previos_position: {}",
            input.end_position, previous_move.piece.position
        );
        if previous_move.en_passant {
            let turn = self.turn;
            self.empty(
                input.end_position.with_y(
                    (input.end_position.y as isize + if turn == 0 { -1 } else { 1 }) as usize,
                ),
            );
        }
        let color_mask = if self.turn == 1 { 0b1000 } else { 0 };
        let piece = Piece::from_position((input.piece_type as u8) ^ color_mask, input.end_position);
        if let Some((king, rook)) = previous_move.castling {
            self.empty(previous_move.piece.position);
            let rook_pos = self.get_rook_pos(input);
            self.empty(rook_pos);
            self[king] = previous_move.piece;
            self[rook] = Piece::new(0b10 ^ color_mask, (rook.x, rook.y));
        } else {
            println!("prev piece position: {:?}", previous_move.piece.position);
            self.empty(previous_move.piece.position);
            self[input.end_position] = piece;
        }
        if piece.piece_type() == (PieceType::King as u8) {
            self.king_positions[self.turn as usize] = input.end_position;
        }

        piece
    }

    fn revert_move(&mut self, previous_move: &PieceMove, input: &Input, piece: Piece) {
        if let Some((king, rook)) = previous_move.castling {
            self.empty(king);
            self.empty(rook);
            self[previous_move.piece.position] = previous_move.piece;
            let prev_rook_pos: Position = match (self.turn, input.castling) {
                (0, 1) => (7usize, 0).into(),
                (0, 2) => (0usize, 0).into(),
                (1, 1) => (7usize, 7).into(),
                (1, 2) => (0usize, 7).into(),
                (_, _) => unreachable!(),
            };
            let color_mask = if self.turn == 1 { 0b1000 } else { 0 };
            self[prev_rook_pos] = Piece::new(0b10 ^ color_mask, (prev_rook_pos.x, prev_rook_pos.y));
        } else {
            self[previous_move.piece.position] = previous_move.piece;
            self.empty(input.end_position);
        }

        if piece.piece_type() == PieceType::King as u8 {
            self.king_positions[self.turn as usize] = previous_move.piece.position;
        }
    }

    fn get_rook_pos(&self, input: &Input) -> Position {
        match (self.turn, input.castling) {
            (0, 1) => (7usize, 0).into(),
            (0, 2) => (0usize, 0).into(),
            (1, 1) => (7usize, 7).into(),
            (1, 2) => (0usize, 7).into(),
            (_, _) => unreachable!(),
        }
    }

    pub fn is_path(
        &self,
        path_type: PiecePath,
        start_position: Position,
        end_position: Position,
    ) -> bool {
        match path_type {
            PiecePath::Straight => {
                if start_position.x == end_position.x {
                    let range = if start_position.y < end_position.y {
                        start_position.y + 1..end_position.y
                    } else {
                        end_position.y + 1..start_position.y
                    };
                    return range
                        .step_by(1)
                        .all(|y| self[(start_position.x, y)].is_empty());
                } else if start_position.y == end_position.y {
                    let range = if start_position.x < end_position.x {
                        start_position.x + 1..end_position.x
                    } else {
                        end_position.x + 1..start_position.x
                    };
                    return range
                        .step_by(1)
                        .all(|x| self[(x, start_position.y)].is_empty());
                }
                false
            }
            PiecePath::Diagonal => {
                if start_position.x == end_position.x || start_position.y == end_position.y {
                    return false;
                }
                let (step_x, step_y) = (
                    if start_position.x < end_position.x {
                        1
                    } else {
                        -1
                    },
                    if start_position.y < end_position.y {
                        1
                    } else {
                        -1
                    },
                );
                let (mut x, mut y) = (
                    start_position.x as isize + step_x,
                    start_position.y as isize + step_y,
                );
                while (x, y) != end_position.into()
                    && Game::in_bounds((x as usize, y as usize).into())
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

    fn get_check_move_coords(&self, piece_type: PieceType, color: u8) -> Input {
        let end_position = if color == 0 {
            self.king_positions[0]
        } else {
            self.king_positions[1]
        };
        Input {
            piece_type,
            is_capture: true,
            end_position,
            castling: 0,
        }
    }

    fn is_attacked(&mut self, input: Input, color: u8) -> bool {
        PieceMove::from_piece_type(input, self, true, color)
            .map(|attacker_move| attacker_move.is_valid(self, input.piece_type))
            .unwrap_or(false)
    }

    pub fn in_bounds(pos: Position) -> bool {
        pos.x <= 7 && pos.y <= 7
    }

    pub fn get_board(&self) -> &[[Piece; 8]; 8] {
        &self.board
    }

    pub fn check_win(&mut self) -> bool {
        self.get_blocking_moves();
        true
    }

    pub fn get_blocking_moves(&mut self) -> bool {
        // Get every valid move from every piece
        let moves = self.get_all_moves();
        println!(
            "Getting all moves: {:?}",
            moves
                .iter()
                .map(|x| match x.piece.piece_type().into() {
                    PieceType::Pawn => "P",
                    PieceType::Knight => "N",
                    PieceType::Bishop => "B",
                    PieceType::Rook => "R",
                    PieceType::Queen => "Q",
                    PieceType::King => "K",
                    PieceType::None => "None",
                }
                .to_owned()
                    + &x.end_position.to_string())
                .collect::<Vec<String>>()
        );

        moves
            .iter()
            .filter(|&x| {
                self.make_move(Input {
                    piece_type: x.piece.piece_type().into(),
                    is_capture: x.is_capture,
                    end_position: x.end_position,
                    castling: 0,
                });
                self.is_king_in_check()[self.turn]
            })
            .for_each(|x| {
                println!("Blocking move: {:?}", x);
            });

        false
    }

    pub fn get_all_moves(&mut self) -> Vec<PieceMove> {
        let mut moves = Vec::new();
        for y in 0..8usize {
            for x in 0..8 {
                if self[(x, y)].is_empty() || self[(x, y)].color() != self.turn {
                    continue;
                }

                let piece_moves = PieceMove::from_piece(self[(x, y)], self);
                moves.extend(piece_moves)
            }
        }
        moves
    }
}
