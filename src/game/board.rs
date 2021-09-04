use crate::game::components::{PlaceResult, TileType};
use std::{fmt, vec};

/// Struct containing board variables & logic
pub struct Board {
    #[cfg(not(test))]
    board: Vec<Vec<TileType>>,
    #[cfg(test)]
    pub board: Vec<Vec<TileType>>,
    dim: (usize, usize), // (x, y)
}

impl Board {
    /// Create an empty 7x6 board
    pub fn default() -> Self {
        let dim = (7, 6);
        let board = vec![vec![TileType::Empty; dim.0]; dim.1];
        Self { board, dim }
    }

    /// Create an empty board with custom dimensions
    #[allow(dead_code)]
    pub fn new(dim_x: usize, dim_y: usize) -> Self {
        let dim = (dim_x, dim_y);
        let board = vec![vec![TileType::Empty; dim.0]; dim.1];
        Self { board, dim }
    }

    /// Create a 7x6 board filled with custom tile type
    #[allow(dead_code)]
    pub fn new_filled(tile_type: TileType) -> Self {
        let dim = (7, 6);
        let board = vec![vec![tile_type; dim.0]; dim.1];
        Self { board, dim }
    }

    /// Place a tile in specified column
    /// # Return
    /// Returns [PlaceResult::Success] if successful
    ///
    /// Returns [PlaceResult::InvalidColumn] if the column is larger than the x dimension or smaller than 0
    ///
    /// Returns [PlaceResult::ColumnFull] if there are no empty spaces in the column
    pub fn place_tile(&mut self, column: usize, tile: TileType) -> PlaceResult {
        if column < 1 || column >= self.dim.0 {
            return PlaceResult::InvalidColumn;
        } else if tile == TileType::Empty {
            return PlaceResult::InvalidTileType;
        }
        for y in 1..=self.dim.1 {
            if self.board[self.dim.1 - y][column - 1] == TileType::Empty {
                self.board[self.dim.1 - y][column - 1] = tile;
                return PlaceResult::Success;
            }
        }

        PlaceResult::ColumnFull
    }

    /// Check if there are 4 tiles of the same type (excluding [TileType::Empty]) in a horizontal, vertical or diagonal row.
    /// # Return
    /// Returns the [TileType] of the 4 tiles in a row, or [TileType::Empty] if no 4 in a rows where found
    pub fn check4(&self) -> TileType {
        for y in 0..self.dim.1 {
            for x in 0..self.dim.0 {
                let main_tile = self.board[y][x];
                if main_tile != TileType::Empty {
                    // check right
                    if x < self.dim.0 - 3
                        && self.board[y][x + 1] == main_tile
                        && self.board[y][x + 2] == main_tile
                        && self.board[y][x + 3] == main_tile
                    {
                        return main_tile;
                    }

                    // check up
                    if y > 2 {
                        if self.board[y - 1][x] == main_tile
                            && self.board[y - 2][x] == main_tile
                            && self.board[y - 3][x] == main_tile
                        {
                            return main_tile;
                        }

                        // check up & right
                        if x < self.dim.0 - 3
                            && self.board[y - 1][x + 1] == main_tile
                            && self.board[y - 2][x + 2] == main_tile
                            && self.board[y - 2][x + 2] == main_tile
                        {
                            return main_tile;
                        }

                        // check up & left
                        if x > 2
                            && self.board[y - 1][x - 1] == main_tile
                            && self.board[y - 2][x - 2] == main_tile
                            && self.board[y - 2][x - 2] == main_tile
                        {
                            return main_tile;
                        }
                    }
                }
            }
        }

        TileType::Empty
    }

    pub fn clear(&mut self) {
        self.board = vec![vec![TileType::Empty; self.dim.0]; self.dim.1]
    }

    #[cfg(test)]
    #[doc(hidden)]
    pub fn place_tile_static(&mut self, x: usize, y: usize, tile: TileType) {
        self.board[y][x] = tile;
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "#-1--2--3--4--5--6--7-#")?;
        for y in 0..self.dim.1 {
            write!(f, "|")?;
            for x in 0..self.dim.0 {
                match self.board[y][x] {
                    TileType::Empty => write!(f, " . ")?,
                    TileType::Player1 => write!(f, " x ")?,
                    TileType::Player2 => write!(f, " o ")?,
                }
            }
            writeln!(f, "|")?;
        }
        writeln!(f, "#---------------------#")
    }
}
