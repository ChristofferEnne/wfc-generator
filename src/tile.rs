pub struct Tile {
  pub name: String,
  pub filename: String,
  pub rotation: usize,
  pub connectors: (String, String, String, String)
}

impl Tile {
  pub fn new(
    name: String,
    filename: String,
    rotation: usize,
    connectors: (String, String, String, String)
  ) -> Self {
    Self {
      name,
      filename,
      rotation,
      connectors
    }
  }

  pub fn testbuffer() -> Vec<Tile> {
    vec![
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
}
