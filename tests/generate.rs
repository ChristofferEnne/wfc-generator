#[cfg(test)]
mod tests {
  use std::path::{Path, PathBuf};

  use wfc_generator::tile::Tile;
  use wfc_generator::{PatternSetting, WFC};

  #[test]
  fn pattern_buffer() {
    let iterations = 10;
    let mut successes = 0;

    let mut wfc = WFC::new(
      PatternSetting::PatternBuffer(Tile::testbuffer()),
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
    let mut wfc = WFC::new(
      PatternSetting::FromDirectory(PathBuf::from(
        r"D:\qrnch Dropbox\Christoffer Enne\IslandDemo\Content\Models\Tiles\"
      )),
      10,
      10,
      0
    );

    wfc.generate();
    wfc.draw();
    wfc.export_csv(Path::new("../export.csv").to_path_buf());
    wfc.export_bytes(Path::new("../data.csv").to_path_buf());
  }
}