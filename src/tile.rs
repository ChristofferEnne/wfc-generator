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
}
