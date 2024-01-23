mod pawn;

use core::fmt::Debug;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::{
    mov::{InvalidMoveReason, Mov, Moves},
    Board, Coordinate,
};

pub trait Piece: PartialEq + PartialOrd + Debug + Serialize + DeserializeOwned {
    fn calculate_available_moves(&self, board: &Board, from: Coordinate) -> Moves;
    fn move_is_available(&self, board: &Board, mov: Mov) -> InvalidMoveReason;
}

use self::pawn::Pawn;

#[derive(PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum PieceType {
    Pawn(Pawn),
}

impl Piece for PieceType {
    fn calculate_available_moves(&self, board: &Board, from: Coordinate) -> Moves {
        match self {
            PieceType::Pawn(x) => x.calculate_available_moves(board, from),
        }
    }
    fn move_is_available(&self, board: &Board, mov: Mov) -> InvalidMoveReason {
        match self {
            PieceType::Pawn(x) => x.move_is_available(board, mov),
        }
    }
}
