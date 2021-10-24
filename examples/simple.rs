use wfc_generator::tile::Tile;
use wfc_generator::tileloader::{TestLoader, TileLoader};
use wfc_generator::{PatternSetting, WFC};

fn main() {
  let iterations = 10;
  let mut successes = 0;

  let mut wfc = WFC::new(
    TestLoader::new().load(),
    10,
    10,
    0
  );

  for i in 0..iterations {
    wfc.set_seed(i);
    if wfc.generate() == true {
      successes += 1;
    }
  }
  wfc.draw();
  assert_eq!(successes, iterations);
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
