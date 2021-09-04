#[warn(missing_docs)]
#[cfg(not(test))]
#[cfg(not(test))]
/// Module for the Board struct
mod board;

#[cfg(test)]
pub mod board;

/// Module containing miscellaneous components used within the board and game modules
#[cfg(not(test))]
mod components;

#[cfg(test)]
pub mod components;
// pub mod error;

#[warn(missing_docs)]
/// Module for the Game struct
pub mod game;
