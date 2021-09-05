use crate::game::{
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
fn test_row_high() {
    let mut b = Board::default();
    let t = TileType::Player1;

    assert_eq!(b.place_tile(8, t), PlaceResult::InvalidColumn);
}

#[test]
fn test_row_low() {
    let mut b = Board::default();
    let t = TileType::Player1;

    assert_eq!(b.place_tile(0, t), PlaceResult::InvalidColumn);
}

#[test]
fn test_row_edge_low() {
    let mut b = Board::default();
    let t = TileType::Player1;

    assert_eq!(b.place_tile(1, t), PlaceResult::Success);
}

#[test]
fn test_row_edge_high() {
    let mut b = Board::default();
    let t = TileType::Player1;

    assert_eq!(b.place_tile(7, t), PlaceResult::Success);
}
