use std::num::TryFromIntError;

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

        for axis in axises {
            let index: i8 = 1;
            let mut kill = false;
            loop {
                let considered_location = (axis.0 * index, axis.1 * index);
                let considered_index_helper: Result<usize, TryFromIntError> = (location as i64
                    + (8 * considered_location.0 + considered_location.1) as i64)
                    .try_into();
                let considered_index = considered_index_helper
                    .inspect_err(|_| kill = true)
                    .unwrap();

                if kill || considered_index > 64 {
                    break;
                }

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
        let mut moves = Vec::new();

        for dir in directions {
            let mut kill = false;

            let considered_index_helper: Result<usize, TryFromIntError> =
                (location as i64 + (8 * dir.0 + dir.1) as i64).try_into();
            let considered_index = considered_index_helper
                .inspect_err(|_| kill = true)
                .unwrap();

            if kill || considered_index > 64 {
                continue;
            }

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
            PieceType::Pawn => todo!(),
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

    fn xy_to_index(x: u8, y: u8) -> usize {
        (y * 8 + x).into()
    }
}

// Print the board out using unicode stuff
impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        for rank in (0..8).rev() {
            output += &format!("{} ", rank + 1);
            for file in 0..8 {
                let index = Self::xy_to_index(rank, file);
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
