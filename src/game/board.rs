use crate::game::components::{PlaceResult, TileType};
use std::fmt;

pub(crate) type C4Board = Board<6, 7>;

#[derive(Debug)]
/// Struct containing board variables & logic
pub struct Board<const H: usize, const W: usize> {
    #[cfg(not(test))]
    board: [[TileType; W]; H],
    #[cfg(test)]
    pub board: [[TileType; W]; H],
    // dim: (usize, usize), // (x, y)
}

impl<const H: usize, const W: usize> Board<H, W> {
    /// Create a 7x6 board filled with custom tile type
    #[allow(dead_code)]
    pub fn new_filled(tile_type: TileType) -> Self {
        let board = [[tile_type; W]; H];
        Self { board }
    }

    /// Place a tile in specified column
    /// # Return
    /// Returns [PlaceResult::Success] if successful
    ///
    /// Returns [PlaceResult::InvalidColumn] if the column is larger than the x dimension or smaller than 0
    ///
    /// Returns [PlaceResult::ColumnFull] if there are no empty spaces in the column
    pub fn place_tile(&mut self, column: usize, tile: TileType) -> PlaceResult {
        if column < 1 || column > W {
            return PlaceResult::InvalidColumn;
        } else if tile == TileType::Empty {
            return PlaceResult::InvalidTileType;
        }
        for y in 1..=H {
            if self.board[H - y][column - 1] == TileType::Empty {
                self.board[H - y][column - 1] = tile;
                return PlaceResult::Success;
            }
        }

        PlaceResult::ColumnFull
    }

    /// Check if there are 4 tiles of the same type (excluding [TileType::Empty]) in a horizontal, vertical or diagonal row.
    /// # Return
    /// Returns the [TileType] of the 4 tiles in a row, or [TileType::Empty] if no 4 in a rows where found
    pub fn check4(&self) -> TileType {
        for y in 0..H {
            for x in 0..W {
                let main_tile = self.board[y][x];
                if main_tile != TileType::Empty {
                    // check right
                    if x < W - 3
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
                        if x < W - 3
                            && self.board[y - 1][x + 1] == main_tile
                            && self.board[y - 2][x + 2] == main_tile
                            && self.board[y - 3][x + 3] == main_tile
                        {
                            return main_tile;
                        }

                        // check up & left
                        if x > 2
                            && self.board[y - 1][x - 1] == main_tile
                            && self.board[y - 2][x - 2] == main_tile
                            && self.board[y - 3][x - 3] == main_tile
                        {
                            return main_tile;
                        }
                    }
                }
            }
        }

        TileType::Empty
    }

    ///
    pub fn clear(&mut self) {
        self.board = [[TileType::Empty; W]; H]
    }

    #[cfg(test)]
    #[doc(hidden)]
    pub fn place_tile_static(&mut self, x: usize, y: usize, tile: TileType) {
        self.board[y][x] = tile;
    }
}

impl<const H: usize, const W: usize> Default for Board<H, W> {
    /// Create an empty 7x6 board
    fn default() -> Self {
        let board = [[TileType::Empty; W]; H];
        Self { board }
    }
}

impl<const H: usize, const W: usize> fmt::Display for Board<H, W> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "#-1--2--3--4--5--6--7-#")?;
        for y in 0..H {
            write!(f, "|")?;
            for x in 0..W {
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
