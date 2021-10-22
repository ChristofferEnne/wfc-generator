pub struct Tile {
  pub name: String,
  pub filename: String
}

impl Tile {
  pub fn new(
    name: String,
    filename: String
  ) -> Self {
    Self {
      name,
      filename
    }
  }
}

pub struct TileContainer {
  pub tiles: Vec<Tile>,
  pub connectors: (usize, usize, usize, usize),
  pub rotation: usize,
}

impl TileContainer {
  pub fn new(
    connectors: (usize, usize, usize, usize),
    rotation: usize
  ) -> Self {
    Self {
      tiles: Vec::new(),
      connectors,
      rotation,
    }
  }
}