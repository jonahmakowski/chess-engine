use crate::board::*;

pub fn choose_move(depth: usize, board: &Board, side: Side) -> Option<Move> {
    let mut best_move = None;
    let mut best_score = i32::MIN;
    let mut alpha = i32::MIN;

    for m in board.get_all_moves(Some(side)) {
        let child = board.copy_with_move(m);
        let score = search(
            &child,
            depth.saturating_sub(1),
            side.opposite(),
            side,
            alpha,
            i32::MAX,
        );

        if score > best_score {
            best_score = score;
            best_move = Some(m);
        }

        alpha = alpha.max(best_score);
    }

    best_move
}

fn search(
    board: &Board,
    depth: usize,
    side_to_move: Side,
    bot_side: Side,
    mut alpha: i32,
    mut beta: i32,
) -> i32 {
    if depth == 0 {
        return score_board(board, bot_side);
    }

    let moves = board.get_all_moves(Some(side_to_move));
    if moves.is_empty() {
        return score_board(board, bot_side);
    }

    if side_to_move == bot_side {
        let mut best_score = i32::MIN;

        for m in moves {
            let child = board.copy_with_move(m);
            let score = search(
                &child,
                depth - 1,
                side_to_move.opposite(),
                bot_side,
                alpha,
                beta,
            );

            best_score = best_score.max(score);
            alpha = alpha.max(best_score);

            if alpha >= beta {
                break;
            }
        }

        best_score
    } else {
        let mut best_score = i32::MAX;

        for m in moves {
            let child = board.copy_with_move(m);
            let score = search(
                &child,
                depth - 1,
                side_to_move.opposite(),
                bot_side,
                alpha,
                beta,
            );

            best_score = best_score.min(score);
            beta = beta.min(best_score);

            if alpha >= beta {
                break;
            }
        }

        best_score
    }
}

fn score_board(board: &Board, side: Side) -> i32 {
    board.get_score::<i32>(side) - board.get_score::<i32>(side.opposite())
}
