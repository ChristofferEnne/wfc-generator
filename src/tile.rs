pub struct Tile {
  pub id: usize,
  pub name: String,
  pub filename: String,
  pub rotation: usize,
  pub connectors: (u8, u8, u8, u8)
}

impl Tile {
  pub fn new(
    id: usize,
    name: String,
    filename: String,
    rotation: usize,
    connectors: (u8, u8, u8, u8)
  ) -> Self {
    Self {
      id,
      name,
      filename,
      rotation,
      connectors
    }
  }

  // 0 none
  // 1 pipe
  pub fn testbuffer() -> Vec<Tile> {
    vec![
      Tile::new(
        0,
        " ".to_string(),
        " ".to_string(),
        0,
        (
          0,
          0,
          0,
          0
        )
      ),
      Tile::new(
        1,
        "└".to_string(),
        "└".to_string(),
        0,
        (
          0,
          1,
          1,
          0
        )
      ),
      Tile::new(
        1,
        "┌".to_string(),
        "┌".to_string(),
        1,
        (
          0,
          0,
          1,
          1
        )
      ),
      Tile::new(
        1,
        "┐".to_string(),
        "┐".to_string(),
        2,
        (
          1,
          0,
          0,
          1
        )
      ),
      Tile::new(
        1,
        "┘".to_string(),
        "┘".to_string(),
        3,
        (
          1,
          1,
          0,
          0
        )
      ),
      Tile::new(
        2,
        "┼".to_string(),
        "┼".to_string(),
        0,
        (
          1,
          1,
          1,
          1
        )
      ),
      Tile::new(
        3,
        "─".to_string(),
        "─".to_string(),
        0,
        (
          1,
          0,
          1,
          0
        )
      ),
      Tile::new(
        3,
        "│".to_string(),
        "│".to_string(),
        0,
        (
          0,
          1,
          0,
          1
        )
      ),
    ]
  }
  
}
