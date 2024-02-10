use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
#[serde(rename_all = "lowercase")]
pub enum PieceKind {
    Pawn,
    Horse,
    Bishop,
    Rook,
    Queen,
    King,
}

impl PieceKind {
    pub fn letter(&self) -> &'static str {
        match self {
            Self::Pawn => "p",
            Self::Horse => "n",
            Self::Bishop => "b",
            Self::Rook => "r",
            Self::Queen => "q",
            Self::King => "k",
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Debug, Default)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    #[default]
    White,
    Black,
}

impl Color {
    pub fn opposite(&self) -> Self {
        match self {
            Self::White => Self::Black,
            Self::Black => Self::White,
        }
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy, Debug)]
pub struct Piece {
    pub kind: PieceKind,
    pub color: Color,
}

impl Piece {
    pub fn new(kind: PieceKind, color: Color) -> Self {
        Self { kind, color }
    }
}
