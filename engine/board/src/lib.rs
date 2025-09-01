use serde::{Deserialize, Serialize};

use pieces::{Piece, PieceType};

mod mov;
mod pieces;

#[derive(PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub enum Color {
    White,
    Black,
}

#[derive(PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Coordinate {
    pub x: usize,
    pub y: usize,
}

#[derive(PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Cell {
    coord: Coordinate,
    piece: PieceType,
    piece_color: Color,
}

#[derive(PartialEq, PartialOrd, Debug, Serialize, Deserialize)]
pub struct Board {
    pub player_turn: Color,
    pub cells: Vec<Cell>,
}

impl Board {
    fn get_cell_at(&self, coord: &Coordinate) -> Option<&Cell> {
        self.cells.iter().find(|cell| &cell.coord == coord)
    }
}
