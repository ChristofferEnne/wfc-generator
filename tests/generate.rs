#[cfg(test)]
mod tests {
  use std::path::{Path, PathBuf};

  use wfc_generator::tile::Tile;
  use wfc_generator::tileloader::{DirectoryLoader, TestLoader, TileLoader};
  use wfc_generator::{PatternSetting, WFC};

  #[test]
  fn pattern_buffer() {
    let iterations = 10;
    let mut successes = 0;

    let tiles = TestLoader::new().load();
    let mut wfc = WFC::new(
      tiles,
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
    wfc.draw();
    assert_eq!(successes, iterations);
  }

  #[test]
  fn from_directory() {
    let tiles = DirectoryLoader::new(PathBuf::from(
        r"D:\qrnch Dropbox\Christoffer Enne\wfcproject\Content\Tiles\"
      )).load();
    let mut wfc = WFC::new(
      tiles,
      10,
      10,
      0
    );

    wfc.generate();
    wfc.draw_data();
    wfc.export_csv(Path::new("../export.csv").to_path_buf());
    wfc.export_bytes(Path::new("../data.wfc").to_path_buf());
  }
}