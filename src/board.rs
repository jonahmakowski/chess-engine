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
    #[allow(unused_variables)]
    fn generic_movement(&self, axis: &[(i8, i8)]) -> Vec<Move> {
        todo!()
    }

    pub fn get_valid_moves(&self) -> Vec<Move> {
        match self.typ {
            PieceType::Bishop => self.generic_movement(&[(1, 1), (-1, 1), (-1, -1), (1, -1)]),
            PieceType::Rook => self.generic_movement(&[(1, 0), (0, 1), (-1, 0), (0, -1)]),
            PieceType::Queen => self.generic_movement(&[
                (1, 1),
                (-1, 1),
                (-1, -1),
                (1, -1),
                (1, 0),
                (0, 1),
                (-1, 0),
                (0, -1),
            ]),
            PieceType::King => todo!(),
            PieceType::Knight => todo!(),
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
}

// Print the board out using unicode stuff
impl std::fmt::Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut output = String::new();

        for rank in (0..8).rev() {
            output += &format!("{} ", rank + 1);
            for file in 0..8 {
                let index = rank * 8 + file;
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
