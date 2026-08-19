mod board;
mod bot;

fn main() {
    let mut board = board::Board::new();
    let mut side = board::Side::White;

    println!("{}", board);

    while let Some(best_move) = bot::choose_move(7, &board, side) {
        println!("{} plays {}", side, best_move);
        board.apply_move(best_move);
        println!("{}", board);

        //thread::sleep(Duration::from_secs(3));
        side = side.opposite();
    }
}
