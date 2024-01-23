use serde::{Deserialize, Serialize};

use crate::Coordinate;

#[derive(PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Mov {
    pub from: Coordinate,
    pub to: Coordinate,
}

#[derive(PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Moves {
    pub from: Coordinate,
    pub tos: Vec<Coordinate>,
}

pub enum InvalidMoveReason {
    /// Used when a move is valid
    None,
    /// Used when a move is simply invalid, such as not moving a bishop diagonally
    Invalid,
    /// Used when a move is attempted, but that would leave your king in check (or other game-ending situations)
    InCheck,
}
