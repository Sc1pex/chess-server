use crate::error::AppResult;
use anyhow::anyhow;
use axum::{
    extract::{Path, State},
    response::IntoResponse,
    routing::post,
    Json, Router,
};
use lib::{board::Board, movegen::legal_moves, piece::Piece};
use serde::{Deserialize, Serialize};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tower_http::{cors::CorsLayer, trace::TraceLayer};
use uuid::Uuid;

mod debug;
mod handle_move;
mod new_game;

type AppState = Arc<Mutex<HashMap<Uuid, Board>>>;

pub fn app() -> Router {
    let state: HashMap<Uuid, Board> = HashMap::new();
    let state = Arc::new(Mutex::new(state));

    Router::new()
        .route("/start", post(new_game::new_game))
        .route("/:game_id", post(handle_move::handle_move))
        .nest("/debug", debug::router())
        .layer(CorsLayer::permissive())
        .layer(TraceLayer::new_for_http())
        .with_state(state)
}

#[derive(Serialize)]
struct Resp {
    id: Uuid,
    legal_moves: Vec<Move>,
    pieces: Vec<(usize, Piece)>,
}

#[derive(Serialize, Deserialize, Debug)]
struct Move {
    from: usize,
    to: usize,
}

fn map_legal_moves(board: &Board) -> Vec<Move> {
    legal_moves(board)
        .into_iter()
        .map(|m| Move {
            from: m.from as usize,
            to: m.to as usize,
        })
        .collect()
}

fn map_pieces(board: &Board) -> Vec<(usize, Piece)> {
    let mut pieces = vec![];

    for i in 0..64 {
        if let Some(piece) = board.piece(i) {
            pieces.push((i as usize, piece));
        }
    }

    pieces
}
