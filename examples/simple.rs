use std::path::{Path, PathBuf};

use wfc_generator::tile::Tile;
use wfc_generator::{PatternSetting, WFC};


fn main() {
  let mut iterations = 10;
  let mut successes = 0;

  let mut wfc = WFC::new(
    PatternSetting::PatternBuffer(pattern_test_buffer()),
    10,
    10,
    0
  );

  for i in 0..iterations {
    wfc.setup();
    wfc.set_seed(i);
    if wfc.generate() == true {
      successes += 1;
    }
  }
  wfc.draw();
  assert_eq!(successes, iterations);
}


fn pattern_test_buffer() -> Vec<Tile> {
  vec![
    Tile::new(
      "O".to_string(),
      "O".to_string(),
      0,
      (
        "-".to_string(),
        "-".to_string(),
        "-".to_string(),
        "-".to_string()
      )
    ),
    Tile::new(
      " ".to_string(),
      " ".to_string(),
      0,
      (
        "air".to_string(),
        "air".to_string(),
        "air".to_string(),
        "air".to_string()
      )
    ),
    Tile::new(
      "└".to_string(),
      "└".to_string(),
      0,
      (
        "air".to_string(),
        "rock".to_string(),
        "rock".to_string(),
        "air".to_string()
      )
    ),
    Tile::new(
      "┌".to_string(),
      "┌".to_string(),
      1,
      (
        "air".to_string(),
        "air".to_string(),
        "rock".to_string(),
        "rock".to_string()
      )
    ),
    Tile::new(
      "┐".to_string(),
      "┐".to_string(),
      2,
      (
        "rock".to_string(),
        "air".to_string(),
        "air".to_string(),
        "rock".to_string()
      )
    ),
    Tile::new(
      "┘".to_string(),
      "┘".to_string(),
      3,
      (
        "rock".to_string(),
        "rock".to_string(),
        "air".to_string(),
        "air".to_string()
      )
    ),
    Tile::new(
      "┼".to_string(),
      "┼".to_string(),
      0,
      (
        "rock".to_string(),
        "rock".to_string(),
        "rock".to_string(),
        "rock".to_string()
      )
    ),
    Tile::new(
      "─".to_string(),
      "─".to_string(),
      0,
      (
        "rock".to_string(),
        "air".to_string(),
        "rock".to_string(),
        "air".to_string()
      )
    ),
    Tile::new(
      "│".to_string(),
      "│".to_string(),
      0,
      (
        "air".to_string(),
        "rock".to_string(),
        "air".to_string(),
        "rock".to_string()
      )
    ),
  ]
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
