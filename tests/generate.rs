#[cfg(test)]
mod tests {
  use std::path::{Path, PathBuf};

  use wfc_generator::tiles::tile::Tile;
  use wfc_generator::tiles::tileloader::{DirectoryLoader, TestLoader, TileLoader};
  use wfc_generator::{PatternSetting, WFC};

  #[test]
  fn pattern_buffer() {
    let iterations = 10;
    let mut successes = 0;

    let mut tileloader = TestLoader::new();
    let mut wfc = WFC::new(
      tileloader.load(),
      50,
      50,
      0
    );

    for i in 0..iterations {
      wfc.set_seed(i);
      if wfc.generate() == true {
        successes += 1;
      }
    }
    wfc.draw(&tileloader.tiles());
    assert_eq!(successes, iterations);
  }

  #[test]
  fn from_directory() {
    let mut tileloader = DirectoryLoader::new(PathBuf::from(
        r"D:\qrnch Dropbox\Christoffer Enne\wfcproject\Content\Tiles\"
      ));
    let mut wfc = WFC::new(
      tileloader.load(),
      10,
      10,
      0
    );

    wfc.generate();
    wfc.draw_data();
    tileloader.export(Path::new("../export.csv").to_path_buf());
    wfc.export(Path::new("../data.wfc").to_path_buf());
  }
}