// #[cfg(test)]
use crate::game::{board::C4Board, components::TileType::*};

#[test]
fn test_detect_4_right() {
    let mut b = C4Board::default();
    let t = Player1;

    b.place_tile_static(0, 0, t);
    b.place_tile_static(1, 0, t);
    b.place_tile_static(2, 0, t);
    b.place_tile_static(3, 0, t);

    assert_ne!(b.check4(), Empty);
}

#[test]
fn test_detect_4_up() {
    let mut b = C4Board::default();
    let t = Player1;

    b.place_tile_static(0, 0, t);
    b.place_tile_static(0, 1, t);
    b.place_tile_static(0, 2, t);
    b.place_tile_static(0, 3, t);

    assert_ne!(b.check4(), Empty);
}

#[test]
fn test_detect_4_up_right() {
    let mut b = C4Board::default();
    let t = Player1;

    b.place_tile_static(0, 3, t);
    b.place_tile_static(1, 2, t);
    b.place_tile_static(2, 1, t);
    b.place_tile_static(3, 0, t);

    assert_ne!(b.check4(), Empty);
}

#[test]
fn test_detect_4_up_left() {
    let mut b = C4Board::default();
    let t = Player1;

    b.place_tile_static(0, 0, t);
    b.place_tile_static(1, 1, t);
    b.place_tile_static(2, 2, t);
    b.place_tile_static(3, 3, t);

    assert_ne!(b.check4(), Empty);
}
#[test]
fn test_detect_3_right() {
    let mut b = C4Board::default();
    let t = Player1;

    b.place_tile_static(0, 0, t);
    b.place_tile_static(1, 0, t);
    b.place_tile_static(2, 0, t);

    assert_eq!(b.check4(), Empty);
}

#[test]
fn test_detect_3_up() {
    let mut b = C4Board::default();
    let t = Player1;

    b.place_tile_static(0, 0, t);
    b.place_tile_static(0, 1, t);
    b.place_tile_static(0, 2, t);

    assert_eq!(b.check4(), Empty);
}

#[test]
fn test_detect_3_up_right() {
    let mut b = C4Board::default();
    let t = Player1;

    b.place_tile_static(0, 3, t);
    b.place_tile_static(1, 2, t);
    b.place_tile_static(2, 1, t);

    assert_eq!(b.check4(), Empty);
}

#[test]
fn test_detect_3_up_left() {
    let mut b = C4Board::default();
    let t = Player1;

    b.place_tile_static(0, 0, t);
    b.place_tile_static(1, 1, t);
    b.place_tile_static(2, 2, t);

    assert_eq!(b.check4(), Empty);
}

#[test]
fn test_detect_4_red() {
    let mut b = C4Board::default();
    let t = Player1;

    b.place_tile_static(0, 0, t);
    b.place_tile_static(1, 0, t);
    b.place_tile_static(2, 0, t);
    b.place_tile_static(3, 0, t);

    assert_eq!(b.check4(), Player1);
}

#[test]
fn test_detect_4_yellow() {
    let mut b = C4Board::default();
    let t = Player2;

    b.place_tile_static(0, 0, t);
    b.place_tile_static(1, 0, t);
    b.place_tile_static(2, 0, t);
    b.place_tile_static(3, 0, t);

    assert_eq!(b.check4(), Player2);
}

#[test]
fn test_detect_4_fail() {
    let b = C4Board::default();

    assert_eq!(b.check4(), Empty);
}
