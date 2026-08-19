#[allow(dead_code)]
mod board;

fn main() {
    let b = board::Board::new();
    println!("{}", b);
    let white_moves = b.get_all_moves(None);
    for m in white_moves {
        println!("{}", m);
    }
}
