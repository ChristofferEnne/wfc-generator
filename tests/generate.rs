#[cfg(test)]
mod tests {
  use std::path::{Path, PathBuf};

  use wfc_generator::tile::Tile;
  use wfc_generator::{PatternSetting, WFC};

  #[test]
  fn pattern_buffer() {
    let mut iterations = 10;
    let mut successes = 0;

    let mut wfc = WFC::new(
      PatternSetting::PatternBuffer(pattern_test_buffer()),
      50,
      50,
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

    wfc.setup();
    wfc.generate();
    wfc.draw();
    wfc.export_csv(Path::new("../export.csv").to_path_buf());
    wfc.export_bytes(Path::new("../data.csv").to_path_buf());
  }

  #[test]
  fn export() {
    let mut wfc = WFC::new(
      PatternSetting::PatternBuffer(pattern_test_buffer()),
      50,
      20,
      0
    );

    wfc.setup();
    wfc.generate();
    wfc.draw();
    wfc.export_csv(Path::new("../export.csv").to_path_buf());
  }

  fn pattern_test_buffer() -> Vec<Tile> {
    vec![Tile::new(
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

    //Tile::new(
    //  "├".to_string(),
    //  "├".to_string(),
    //  0,
    //  (
    //    "air".to_string(),
    //    "rock".to_string(),
    //    "rock".to_string(),
    //    "rock".to_string(),
    //  ),
    //),
    //Tile::new(
    //  "┬".to_string(),
    //  "┬".to_string(),
    //  1,
    //  (
    //    "rock".to_string(),
    //    "air".to_string(),
    //    "rock".to_string(),
    //    "rock".to_string(),
    //  ),
    //),
    //Tile::new(
    //  "┤".to_string(),
    //  "┤".to_string(),
    //  2,
    //  (
    //    "rock".to_string(),
    //    "rock".to_string(),
    //    "air".to_string(),
    //    "rock".to_string(),
    //  ),
    //),
    //Tile::new(
    //  "┴".to_string(),
    //  "┴".to_string(),
    //  3,
    //  (
    //    "rock".to_string(),
    //    "rock".to_string(),
    //    "rock".to_string(),
    //    "air".to_string(),
    //  ),
    //),
    //Tile::new("╴".to_string(), 0, (
    //  "rock".to_string(),
    //  "air".to_string(),
    //  "air".to_string(),
    //  "air".to_string()
    //  ),
    //),
    //Tile::new("╵".to_string(), 1, (
    //  "air".to_string(),
    //  "rock".to_string(),
    //  "air".to_string(),
    //  "air".to_string()
    //),),
    //Tile::new("╶".to_string(), 2, (
    //  "air".to_string(),
    //  "air".to_string(),
    //  "rock".to_string(),
    //  "air".to_string()
    //),),
    //Tile::new("╷".to_string(), 3, (
    //  "air".to_string(),
    //  "air".to_string(),
    //  "air".to_string(),
    //  "rock".to_string()
    //),),
    ]
  }
}
