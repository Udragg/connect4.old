use crate::game::board::Board;

pub struct Ai<'a> {
    board: &'a Board,
}

impl<'a> Ai<'a> {
    pub(crate) fn new(board: &'a Board) -> Self {
        Self { board }
    }
}
