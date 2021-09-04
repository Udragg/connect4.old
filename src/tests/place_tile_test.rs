use crate::{
    board::Board,
    components::{PlaceResult, TileType},
};

#[test]
fn test_place() {
    let mut b = Board::default();
    let t = TileType::Player1;
    let r = 4;

    b.place_tile(r, t);
    assert_eq!(t, b.board[5][r - 1]);
}

#[test]
fn test_column_overflow() {
    let mut b = Board::default();
    let t = TileType::Player1;
    let r = 4;

    for _ in 0..6 {
        b.place_tile(r, t);
    }

    assert_eq!(b.place_tile(r, t), PlaceResult::ColumnFull);
}

#[test]
fn test_row_overflow() {
    let mut b = Board::default();
    let t = TileType::Player1;

    assert_eq!(b.place_tile(8, t), PlaceResult::InvalidColumn);
}

#[test]
fn test_row_underflow() {
    let mut b = Board::default();
    let t = TileType::Player1;

    assert_eq!(b.place_tile(0, t), PlaceResult::InvalidColumn);
}
