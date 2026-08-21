mod board;
mod bot;
mod user_interface;

fn main() {
    let mut board = board::Board::new();

    println!("{}", board);

    loop {
        user_interface::play_move(&mut board, board::Side::White);
        println!("{}", board);

        let Some(best_move) = bot::choose_move(7, &board, board::Side::Black) else {
            break;
        };

        println!("Black plays {}", best_move);
        board.apply_move(best_move);
        println!("{}", board);
    }
}
