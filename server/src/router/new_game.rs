use super::*;

pub async fn new_game(State(state): State<AppState>) -> impl IntoResponse {
    let board = Board::start_pos();
    let id = Uuid::new_v4();
    let legal_moves = map_legal_moves(&board);
    let pieces = map_pieces(&board);
    state.lock().unwrap().insert(id, board);

    Json(Resp {
        id,
        legal_moves,
        pieces,
    })
}
