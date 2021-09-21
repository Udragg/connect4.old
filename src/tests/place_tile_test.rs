use crate::game::{
    board::C4Board,
    components::{PlaceResult, TileType},
};

#[test]
fn test_place() {
    let mut b = C4Board::default();
    let t = TileType::Player1;

    for x in 1..8 {
        b.place_tile(x, t);
        assert_eq!(b.board[5][x - 1], t);
    }
}

#[test]
fn test_column_overflow() {
    let mut b = C4Board::default();
    let t = TileType::Player1;
    let r = 4;

    for _ in 0..6 {
        b.place_tile(r, t);
    }

    assert_eq!(b.place_tile(r, t), PlaceResult::ColumnFull);
}

#[test]
fn test_row_high() {
    let mut b = C4Board::default();
    let t = TileType::Player1;

    assert_eq!(b.place_tile(8, t), PlaceResult::InvalidColumn);
}

#[test]
fn test_row_low() {
    let mut b = C4Board::default();
    let t = TileType::Player1;

    assert_eq!(b.place_tile(0, t), PlaceResult::InvalidColumn);
}

#[test]
fn test_row_edge_low() {
    let mut b = C4Board::default();
    let t = TileType::Player1;

    assert_eq!(b.place_tile(1, t), PlaceResult::Success);
}

#[test]
fn test_row_edge_high() {
    let mut b = C4Board::default();
    let t = TileType::Player1;

    assert_eq!(b.place_tile(7, t), PlaceResult::Success);
}
