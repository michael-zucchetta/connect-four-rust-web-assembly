use crate::models::{ConnectFourBoard, ConnectFourMove};

const WIN_SCORE: i32 = 100_000;

fn piece_owner(piece: ConnectFourMove) -> Option<ConnectFourMove> {
    match piece {
        ConnectFourMove::XPosition | ConnectFourMove::XIgnoredPosition => {
            Some(ConnectFourMove::XPosition)
        }
        ConnectFourMove::OPosition | ConnectFourMove::OIgnoredPosition => {
            Some(ConnectFourMove::OPosition)
        }
        _ => None,
    }
}

fn window_score(window: &[ConnectFourMove], root_move: ConnectFourMove) -> i32 {
    let opponent = opposite_move(root_move);
    let root_count = window
        .iter()
        .filter(|piece| piece_owner(**piece) == Some(root_move))
        .count();
    let opponent_count = window
        .iter()
        .filter(|piece| piece_owner(**piece) == Some(opponent))
        .count();
    if root_count != 0 && opponent_count != 0 {
        return 0;
    }

    let weight = |count| match count {
        1 => 1,
        2 => 12,
        3 => 150,
        4 => WIN_SCORE,
        _ => 0,
    };
    weight(root_count) - weight(opponent_count)
}

fn evaluate(board: &ConnectFourBoard, root_move: ConnectFourMove) -> i32 {
    let mut total = 0;
    let center = (board.width - 1) / 2;
    for piece in &board.board[center] {
        total += match piece_owner(*piece) {
            Some(piece) if piece == root_move => 4,
            Some(_) => -4,
            None => 0,
        };
    }

    for column in 0..board.width {
        for row in 0..board.height {
            for (column_step, row_step) in [(1isize, 0isize), (0, 1), (1, 1), (1, -1)] {
                let end_column = column as isize + 3 * column_step;
                let end_row = row as isize + 3 * row_step;
                if end_column < 0
                    || end_row < 0
                    || end_column >= board.width as isize
                    || end_row >= board.height as isize
                {
                    continue;
                }
                let window = (0..4)
                    .map(|offset| {
                        board.board[(column as isize + offset * column_step) as usize]
                            [(row as isize + offset * row_step) as usize]
                    })
                    .collect::<Vec<_>>();
                total += window_score(&window, root_move);
            }
        }
    }
    total
}

fn opposite_move(move_type: ConnectFourMove) -> ConnectFourMove {
    match move_type {
        ConnectFourMove::XPosition => ConnectFourMove::OPosition,
        ConnectFourMove::OPosition => ConnectFourMove::XPosition,
        _ => panic!("Minmax can only play normal pieces"),
    }
}

fn ordered_moves(board: &ConnectFourBoard) -> Vec<usize> {
    let center = (board.width as isize - 1) / 2;
    let mut moves = board.free_moves();
    moves.sort_by_key(|column| ((*column as isize - center).abs(), *column));
    moves
}

fn score(
    board: &ConnectFourBoard,
    root_move: ConnectFourMove,
    current_move: ConnectFourMove,
    depth: usize,
    mut alpha: i32,
    mut beta: i32,
) -> i32 {
    if let Some((winner, _)) = board.return_winner(0, 0) {
        return if winner == root_move {
            WIN_SCORE + depth as i32
        } else {
            -WIN_SCORE - depth as i32
        };
    }
    if depth == 0 || board.moves_left() == 0 {
        return evaluate(board, root_move);
    }

    let maximizing = current_move == root_move;
    let mut best = if maximizing { i32::MIN } else { i32::MAX };
    for column in ordered_moves(board) {
        let mut next = board.clone();
        next.make_move(current_move, column);
        let candidate = score(
            &next,
            root_move,
            opposite_move(current_move),
            depth - 1,
            alpha,
            beta,
        );
        if maximizing {
            best = best.max(candidate);
            alpha = alpha.max(best);
        } else {
            best = best.min(candidate);
            beta = beta.min(best);
        }
        if alpha >= beta {
            break;
        }
    }
    best
}

pub fn next_move(board: &ConnectFourBoard, root_move: ConnectFourMove, depth: usize) -> usize {
    let mut best_column = ordered_moves(board).into_iter().next().unwrap_or(0);
    let mut best_score = i32::MIN;
    for column in ordered_moves(board) {
        let mut next = board.clone();
        next.make_move(root_move, column);
        let candidate = score(
            &next,
            root_move,
            opposite_move(root_move),
            depth.saturating_sub(1),
            i32::MIN,
            i32::MAX,
        );
        if candidate > best_score {
            best_score = candidate;
            best_column = column;
        }
    }
    best_column
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn blocks_an_opponents_forced_win() {
        let mut board = ConnectFourBoard::new(&vec![6; 7], &vec![]);
        for column in 0..3 {
            assert!(board.make_move(ConnectFourMove::XPosition, column));
        }
        assert_eq!(next_move(&board, ConnectFourMove::OPosition, 2), 3);
    }

    #[test]
    fn rewards_an_unblocked_developing_line() {
        let empty = ConnectFourBoard::new(&vec![6; 7], &vec![]);
        let mut developing = empty.clone();
        developing.make_move(ConnectFourMove::OPosition, 1);
        developing.make_move(ConnectFourMove::OPosition, 2);

        assert!(
            evaluate(&developing, ConnectFourMove::OPosition)
                > evaluate(&empty, ConnectFourMove::OPosition)
        );
    }
}
