use wfc_generator::tiles::tile::Tile;
use wfc_generator::tiles::tileloader::{TestLoader, TileLoader};
use wfc_generator::{PatternSetting, WFC};

fn main() {
  let iterations = 10;
  let mut successes = 0;
  let mut tileloader = TestLoader::new();
  let mut wfc = WFC::new(
    tileloader.load(),
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
  wfc.draw(tileloader.tiles());
  assert_eq!(successes, iterations);
}

// vim: set ft=rust et sw=2 ts=2 sts=2 cinoptions=2 tw=79 :
