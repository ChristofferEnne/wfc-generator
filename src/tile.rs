pub struct Tile {
  pub id: usize,
  pub name: String,
  pub filename: String,
  pub rotation: usize, // nr of extra rotation variants to create
  pub connectors: ((u8, bool), (u8, bool), (u8, bool), (u8, bool)) // socket, flipped
}

impl Tile {
  pub fn new(
    id: usize,
    name: String,
    filename: String,
    rotation: usize,
    connectors: ((u8, bool), (u8, bool), (u8, bool), (u8, bool))
  ) -> Self {
    Self {
      id,
      name,
      filename,
      rotation,
      connectors
    }
  }
}
