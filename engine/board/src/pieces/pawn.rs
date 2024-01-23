use serde::{Deserialize, Serialize};

use crate::{
    mov::{Mov, Moves},
    Color, Coordinate,
};

use super::Piece;

#[derive(PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Pawn;

impl Piece for Pawn {
    fn calculate_available_moves(
        &self,
        board: &crate::Board,
        from: Coordinate,
    ) -> crate::mov::Moves {
        Moves { from, tos: vec![] }
    }
    fn move_is_available(&self, board: &crate::Board, mov: Mov) -> crate::mov::InvalidMoveReason {
        crate::mov::InvalidMoveReason::Invalid
    }
}
