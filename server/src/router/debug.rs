use super::*;
use lib::movegen::generate_moves;

pub fn router() -> Router<AppState> {
    Router::new().route("/perft", post(perft))
}

#[derive(Deserialize)]
struct PerftData {
    fen: String,
    depth: u32,
}

async fn perft(Json(data): Json<PerftData>) -> AppResult<impl IntoResponse> {
    let board = Board::from_fen(&data.fen);
    let mut count = 0;
    let mut res = vec![];

    let start = std::time::Instant::now();
    for m in generate_moves(&board) {
        let mut b = board.clone();
        if b.make_move(m) {
            let x = run_perft(data.depth - 1, &mut b);
            res.push(format!("{}: {}\n", m, x));

            count += x;
        }
    }

    Ok(format!(
        "{}\n{} in {:?}",
        res.into_iter().fold(String::new(), |acc, m| acc + &m),
        count,
        start.elapsed()
    ))
}

fn run_perft(depth: u32, board: &mut Board) -> u128 {
    if depth == 0 {
        return 1;
    }

    let moves = generate_moves(board);
    let mut count = 0;

    for m in moves {
        let mut b = board.clone();
        if b.make_move(m) {
            let x = run_perft(depth - 1, &mut b);
            count += x;
        }
    }

    count
}
