use num_traits::PrimInt;

#[derive(Clone, Copy, Debug, PartialEq)]
enum PieceType {
    Pawn,
    Knight,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceType {
    fn get_value(&self) -> u16 {
        match self {
            PieceType::Pawn => 100,
            PieceType::Knight => 320,
            PieceType::Bishop => 330,
            PieceType::Rook => 500,
            PieceType::Queen => 900,
            PieceType::King => 60_000,
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
struct Piece {
    // Rook, knight, queen, etc
    typ: PieceType,
    // White/Black
    side: Side,
}

impl std::fmt::Display for Piece {
    // Get unicode versions of the piece for printing
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.typ {
            PieceType::Pawn => write!(
                f,
                "{}",
                if let Side::White = self.side {
                    "♙"
                } else {
                    "♟"
                }
            ),
            PieceType::Knight => write!(
                f,
                "{}",
                if let Side::White = self.side {
                    "♘"
                } else {
                    "♞"
                }
            ),
            PieceType::Bishop => write!(
                f,
                "{}",
                if let Side::White = self.side {
                    "♗"
                } else {
                    "♝"
                }
            ),
            PieceType::Rook => write!(
                f,
                "{}",
                if let Side::White = self.side {
                    "♖"
                } else {
                    "♜"
                }
            ),
            PieceType::Queen => write!(
                f,
                "{}",
                if let Side::White = self.side {
                    "♕"
                } else {
                    "♛"
                }
            ),
            PieceType::King => write!(
                f,
                "{}",
                if let Side::White = self.side {
                    "♔"
                } else {
                    "♚"
                }
            ),
        }
    }
}

// Get all possible moves for a piece
impl Piece {
    fn generic_movement(&self, axises: &[(i8, i8)], board: &Board, location: usize) -> Vec<Move> {
        let mut moves = Vec::new();
        let current_location: (i8, i8) = Board::index_to_xy(location);

        for axis in axises {
            let mut index: i8 = 1;
            loop {
                let considered_delta = (axis.0 * index, axis.1 * index);
                let considered_location: (i8, i8) = (
                    considered_delta.0 + current_location.0,
                    considered_delta.1 + current_location.1,
                );

                if considered_location.0 > 7
                    || considered_location.0 < 0
                    || considered_location.1 > 7
                    || considered_location.1 < 0
                {
                    break;
                }

                let considered_index =
                    Board::xy_to_index(considered_location.0, considered_location.1);

                let piece_at_considered = board.squares[considered_index];

                match piece_at_considered {
                    Some(p) => {
                        if p.side != self.side {
                            moves.push(Move {
                                start_index: location,
                                end_index: considered_index,
                            })
                        }

                        break;
                    }
                    None => moves.push(Move {
                        start_index: location,
                        end_index: considered_index,
                    }),
                }

                index += 1;
            }
        }

        moves
    }

    fn single_movement(
        &self,
        directions: &[(i8, i8)],
        board: &Board,
        location: usize,
    ) -> Vec<Move> {
        let current_location: (i8, i8) = Board::index_to_xy(location);

        let mut moves = Vec::new();

        for dir in directions {
            let considered_location: (i8, i8) =
                (dir.0 + current_location.0, dir.1 + current_location.1);

            if considered_location.0 > 7
                || considered_location.0 < 0
                || considered_location.1 > 7
                || considered_location.1 < 0
            {
                continue;
            }

            let considered_index = Board::xy_to_index(considered_location.0, considered_location.1);

            let piece_at_considered = board.squares[considered_index];

            match piece_at_considered {
                Some(p) => {
                    if p.side != self.side {
                        moves.push(Move {
                            start_index: location,
                            end_index: considered_index,
                        })
                    }
                }
                None => moves.push(Move {
                    start_index: location,
                    end_index: considered_index,
                }),
            }
        }

        moves
    }

    fn pawn_movement(&self, board: &Board, location: usize) -> Vec<Move> {
        let current_location: (i32, i32) = Board::index_to_xy(location);

        let mut moves = Vec::new();

        // Moving forward
        let new_location = if self.side == Side::White {
            location + 8
        } else {
            location - 8
        };

        let up_one = board.squares[new_location];

        if up_one.is_none() {
            moves.push(Move {
                start_index: location,
                end_index: new_location,
            })
        }

        // Starting position two moves up
        if ((Board::index_to_xy::<u8>(location).1 == 1 && self.side == Side::White)
            || (Board::index_to_xy::<u8>(location).1 == 6 && self.side == Side::Black))
            && up_one.is_none()
        {
            let new_location = if self.side == Side::White {
                location + 16
            } else {
                location - 16
            };
            let two_up = board.squares[new_location];

            if two_up.is_none() {
                moves.push(Move {
                    start_index: location,
                    end_index: new_location,
                })
            }
        }

        // Attacking
        let attack1_delta = if self.side == Side::White {
            (1, 1)
        } else {
            (1, -1)
        };
        let attack2_delta = if self.side == Side::White {
            (-1, 1)
        } else {
            (-1, -1)
        };

        let attack1_new_location = (
            attack1_delta.0 + current_location.0,
            attack1_delta.1 + current_location.1,
        );

        let attack2_new_location = (
            attack2_delta.0 + current_location.0,
            attack2_delta.1 + current_location.1,
        );

        if attack1_new_location.0 <= 7
            && attack1_new_location.0 >= 0
            && attack1_new_location.1 <= 7
            && attack1_new_location.1 >= 0
        {
            let attack_index = Board::xy_to_index(attack1_new_location.0, attack1_new_location.1);

            let attack = board.squares[attack_index];

            if let Some(p) = attack
                && p.side != self.side
            {
                moves.push(Move {
                    start_index: location,
                    end_index: attack_index,
                })
            }
        }

        if attack2_new_location.0 <= 7
            && attack2_new_location.0 >= 0
            && attack2_new_location.1 <= 7
            && attack2_new_location.1 >= 0
        {
            let attack_index = Board::xy_to_index(attack2_new_location.0, attack2_new_location.1);

            let attack = board.squares[attack_index];

            if let Some(p) = attack
                && p.side != self.side
            {
                moves.push(Move {
                    start_index: location,
                    end_index: attack_index,
                })
            }
        }

        moves
    }

    pub fn get_valid_moves(&self, board: &Board, location: usize) -> Vec<Move> {
        match self.typ {
            PieceType::Bishop => {
                self.generic_movement(&[(1, 1), (-1, 1), (-1, -1), (1, -1)], board, location)
            }
            PieceType::Rook => {
                self.generic_movement(&[(1, 0), (0, 1), (-1, 0), (0, -1)], board, location)
            }
            PieceType::Queen => self.generic_movement(
                &[
                    (1, 1),
                    (-1, 1),
                    (-1, -1),
                    (1, -1),
                    (1, 0),
                    (0, 1),
                    (-1, 0),
                    (0, -1),
                ],
                board,
                location,
            ),
            PieceType::King => self.single_movement(
                &[
                    (1, 1),
                    (-1, 1),
                    (-1, -1),
                    (1, -1),
                    (1, 0),
                    (0, 1),
                    (-1, 0),
                    (0, -1),
                ],
                board,
                location,
            ),
            PieceType::Knight => self.single_movement(
                &[
                    (2, 1),
                    (2, -1),
                    (-2, 1),
                    (-2, -1),
                    (1, 2),
                    (-1, 2),
                    (1, -2),
                    (-1, -2),
                ],
                board,
                location,
            ),
            PieceType::Pawn => self.pawn_movement(board, location),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Side {
    White,
    Black,
}

#[derive(Clone, PartialEq)]
pub struct Board {
    squares: [Option<Piece>; 64],
    turn: Side,
}

impl Board {
    pub fn new() -> Self {
        let mut squares = [None; 64];

        // Piece layout for back ranks (white and black)
        let back_rank = [
            PieceType::Rook,
            PieceType::Knight,
            PieceType::Bishop,
            PieceType::Queen,
            PieceType::King,
            PieceType::Bishop,
            PieceType::Knight,
            PieceType::Rook,
        ];

        // Place pawns
        for file in 0..8 {
            squares[file + 8] = Some(Piece {
                typ: PieceType::Pawn,
                side: Side::White,
            });
            squares[file + 48] = Some(Piece {
                typ: PieceType::Pawn,
                side: Side::Black,
            });
        }

        // Place back rank pieces
        for (file, &piece_type) in back_rank.iter().enumerate() {
            squares[file] = Some(Piece {
                typ: piece_type,
                side: Side::White,
            });
            squares[file + 56] = Some(Piece {
                typ: piece_type,
                side: Side::Black,
            });
        }

        Board {
            squares,
            turn: Side::White,
        }
    }
}

impl Board {
    // Get the score according to the value of the piece for a side
    fn get_score(&self, side: Side) -> u32 {
        let mut score: u32 = 0;

        for square in &self.squares {
            if let Some(piece) = square
                && piece.side == side
            {
                score += piece.typ.get_value() as u32;
            }
        }

        score
    }

    // Move a piece
    fn apply_move(&mut self, m: Move) {
        self.squares[m.end_index] = self.squares[m.start_index];
        self.squares[m.start_index] = None;
    }

    fn xy_to_index<T: PrimInt>(x: T, y: T) -> usize {
        x.to_usize().unwrap() + y.to_usize().unwrap() * 8
    }

    fn index_to_xy<T: PrimInt>(index: usize) -> (T, T) {
        (
            T::from(index % 8).expect("Impossibly big number"),
            T::from(index / 8).expect("Impossibly big number"),
        )
    }
}

// Print the board out using unicode stuff
impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        for rank in (0..8).rev() {
            output += &format!("{} ", rank + 1);
            for file in 0..8 {
                let index = Self::xy_to_index(file, rank);
                match self.squares[index] {
                    Some(piece) => output += &format!(" {} ", piece),
                    None => output += " □ ",
                }
            }

            output += "\n";
        }

        output += "   A  B  C  D  E  F  G  H ";

        write!(f, "{}", output)
    }
}

struct Move {
    start_index: usize,
    end_index: usize,
}
