use crate::game::board::C4Board;

/// AI struct
pub struct Ai<'a> {
    _board: &'a C4Board,
}

impl<'a> Ai<'a> {
    pub(crate) fn new(_board: &'a C4Board) -> Self {
        todo!()
    }
}
