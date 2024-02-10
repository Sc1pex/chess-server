use super::*;

pub async fn handle_move(
    Path(id): Path<Uuid>,
    State(state): State<AppState>,
    Json(jm): Json<Move>,
) -> AppResult<Json<Resp>> {
    let mut board = state.lock().unwrap();
    let board = board.get_mut(&id).ok_or(anyhow!("invalid id"))?;
    let legal_moves = legal_moves(&board);
    let m = legal_moves
        .into_iter()
        .find(|m| jm.from == m.from as usize && jm.to == m.to as usize)
        .ok_or(anyhow!("invalid move"))?;
    println!("move: {:?}", m);
    board.make_move(m);

    Ok(Json(Resp {
        id,
        legal_moves: map_legal_moves(&board),
        pieces: map_pieces(&board),
    }))
}
