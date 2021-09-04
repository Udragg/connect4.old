use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    /// empty
    Empty = 0,
    /// player 1
    Player1 = 1,
    /// player 2
    Player2 = 2,
}

impl fmt::Display for TileType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Self::Player1 => write!(f, "Player 1"),
            &Self::Player2 => write!(f, "Player 2"),
            &Self::Empty => write!(f, "Empty"),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PlaceResult {
    Success,
    ColumnFull,
    InvalidColumn,
    InvalidTileType,
}

pub struct Score {
    pub p1: isize,
    pub p2: isize,
}

impl Score {
    pub fn new() -> Self {
        Self { p1: 0, p2: 0 }
    }
}

#[derive(Debug)]
pub enum ActivePlayer {
    Player1,
    Player2,
}

impl ActivePlayer {
    pub fn swap(&mut self) {
        match self {
            &mut ActivePlayer::Player1 => *self = ActivePlayer::Player2,
            &mut ActivePlayer::Player2 => *self = ActivePlayer::Player1,
        }
    }

    pub fn to_tiletype(&self) -> TileType {
        match self {
            ActivePlayer::Player1 => TileType::Player1,
            ActivePlayer::Player2 => TileType::Player2,
        }
    }
}

impl fmt::Display for ActivePlayer {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            &Self::Player1 => write!(f, "Player 1"),
            &Self::Player2 => write!(f, "Player 2"),
        }
    }
}
