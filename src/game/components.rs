use std::{fmt, io::stdin, str::FromStr};

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

impl Default for TileType {
    fn default() -> Self {
        Self::Empty
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

pub enum Input {
    Col(usize),
    Yes,
    No,
    Enter,
}

impl Input {
    pub fn get_input() -> Option<Self> {
        let mut buf = String::new();

        stdin().read_line(&mut buf).expect("Could not read stdin");

        match Input::from_str(&buf) {
            Ok(input) => Some(input),
            Err(_) => None,
        }
    }
}

impl FromStr for Input {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().trim() {
            "" => Ok(Input::Enter),
            "yes" | "y" => Ok(Input::Yes),
            "no" | "n" => Ok(Input::No),
            b => match b.parse::<usize>() {
                Ok(i) => Ok(Input::Col(i)),
                Err(_) => Err("Could not parse str to Input"),
            },
        }
    }
}
