use std::io::{stdout, Write};

use crate::{
    ai::Ai,
    game::{
        board::C4Board,
        components::{ActivePlayer, Input, PlaceResult, Score, TileType},
    },
};

/// Game struct manages the course of the game.
/// Contains the public methods to create and start a new game
pub struct Game<'a> {
    board: C4Board,
    score: Score,
    active_player: ActivePlayer,

    ai: Option<Ai<'a>>,
}

impl<'a> Game<'a> {
    /// Creates a new Game instance.
    pub fn new() -> Self {
        Self {
            board: C4Board::default(),
            score: Score::new(),
            active_player: ActivePlayer::Player1,
            ai: None,
        }
    }

    /// Adds an Ai instance to the game
    pub fn add_ai(&'a mut self) {
        self.ai = Some(Ai::new(&self.board));
    }

    /// Ask player if they want to start a new round.
    /// If the received input is invalid it will call itself recursively until a valid input is received
    fn ask_new_round(&self) -> bool {
        print!("Do you want to start a new round? [Y/n] : ");
        stdout()
            .lock()
            .flush()
            .ok()
            .expect("Could not flush stdout");
        let input = Input::get_input();

        if let Some(i) = input {
            return match i {
                Input::Yes | Input::Enter => true,
                Input::No => false,
                _ => self.ask_new_round(),
            };
        }
        self.ask_new_round()
    }

    /// Asks the column number the player wants to play
    /// If the received input is invalid it will call itself recursively until a valid input is received
    fn get_col(&mut self) -> usize {
        print!("Player {}'s turn. Column number: ", self.active_player);
        stdout()
            .lock()
            .flush()
            .ok()
            .expect("Could not flush stdout");
        let input = Input::get_input();

        if let Some(Input::Col(c)) = input {
            return c.min(7).max(1);
        }

        println!("Input must be a number between 1 and 7");
        self.get_col()
    }

    /// Starts a single round.
    pub fn start(&mut self) {
        loop {
            print!("{}", self.board);
            // println!("{}'s turn", self.active_player);

            let col = self.get_col();
            if self.board.place_tile(col, self.active_player.to_tiletype()) != PlaceResult::Success
            {
                println!("Unable to place piece at column {}", col);
                continue;
            }

            match self.board.check4() {
                TileType::Player1 => {
                    self.score.p1 += 1;
                    break;
                }
                TileType::Player2 => {
                    self.score.p2 += 1;
                    break;
                }
                _ => {
                    self.active_player.swap();
                }
            }
        }
        print!("{}", self.board);
        println!("{} wins the match", self.active_player);
        self.active_player.swap();

        println!(
            "Player 1 points: {}\nPlayer 2 points: {}",
            self.score.p1, self.score.p2
        );
    }

    /// Starts a new round after the end of every round, until the player decides to exit.
    pub fn start_loop(&mut self) {
        loop {
            if !self.ask_new_round() {
                break;
            };
            self.start();
            self.board.clear();
        }
    }
}
