use crate::game::board::Board;

pub struct Ai {
    board: Board,
}

impl Ai {
    pub(crate) fn new(board: Board) -> Self {
        Self { board }
    }
}
