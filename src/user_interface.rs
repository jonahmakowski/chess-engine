use crate::board::*;
use anyhow::{Result, bail};
use simple_lib::input::get_string_input;

pub fn play_move(board: &mut Board, side: Side) {
    let valid_moves = board.get_all_moves(Some(side));

    loop {
        println!("Enter a move");
        match get_string_input() {
            Ok(inp) => match parse_move(&inp) {
                Ok(m) => {
                    if valid_moves.contains(&m) {
                        board.apply_move(m);
                        break;
                    }
                }
                Err(e) => eprintln!("{}", e),
            },
            Err(err) => eprintln!("{}", err),
        }
    }
}

fn parse_move(m: &str) -> Result<Move> {
    let mut locations: [(u32, u32); 2] = [(0, 0), (0, 0)];

    for (index, letter) in m.chars().enumerate() {
        if letter == '\n' {
            println!("Got to newline");
            continue;
        }

        if index % 2 == 0 {
            match letter_to_number(letter) {
                Some(num) => {
                    if num < 8 {
                        locations[index / 2].0 = num
                    } else {
                        bail!("This letter doesn't belong on a chessboard!")
                    }
                }
                None => bail!("Letter was invalid"),
            }
        } else {
            match letter.to_digit(10) {
                Some(num) => {
                    if (1..=8).contains(&num) {
                        locations[index / 2].1 = num - 1
                    } else {
                        bail!("This number does not fit on a chessboard")
                    }
                }
                None => {
                    bail!("{} is not a number", letter)
                }
            }
        }

        println!("{}\n{:#?}\n\n", index, locations);
    }

    let start_cords = Board::xy_to_index(locations[0].0, locations[0].1);
    let end_cords = Board::xy_to_index(locations[1].0, locations[1].1);

    let parsed_move = Move {
        start_index: start_cords,
        end_index: end_cords,
    };

    println!("{}", parsed_move);

    Ok(parsed_move)
}

fn letter_to_number(c: char) -> Option<u32> {
    if c.is_ascii_alphabetic() && c.is_ascii_uppercase() {
        Some((c as u32) - ('A' as u32))
    } else if c.is_ascii_alphabetic() && c.is_ascii_lowercase() {
        Some((c as u32) - ('a' as u32))
    } else {
        None
    }
}
