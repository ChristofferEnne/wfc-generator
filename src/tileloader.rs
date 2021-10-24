use std::{ffi::OsStr, fs, path::{self, PathBuf}};

use crate::tile::Tile;

pub trait TileLoader {
    fn load(&mut self) -> Vec<Tile>;
}

pub struct TestLoader {}

impl  TestLoader {
  pub fn new() -> Self { Self {} }
}

impl TileLoader for TestLoader {
  fn load(&mut self) -> Vec<Tile> {
    // 0 none
    // 1 pipe
    vec![
      Tile::new(
        0,
        " ".to_string(),
        " ".to_string(),
        0,
        (
          (0,false),
          (0,false),
          (0,false),
          (0,false),
        )
      ),
      Tile::new(
        1,
        "└".to_string(),
        "└".to_string(),
        0,
        (
          (0,false),
          (1,false),
          (1,false),
          (0,false),
        )
      ),
      Tile::new(
        1,
        "┌".to_string(),
        "┌".to_string(),
        1,
        (
          (0,false),
          (0,false),
          (1,false),
          (1,false),
        )
      ),
      Tile::new(
        1,
        "┐".to_string(),
        "┐".to_string(),
        2,
        (
          (1,false),
          (0,false),
          (0,false),
          (1,false),
        )
      ),
      Tile::new(
        1,
        "┘".to_string(),
        "┘".to_string(),
        3,
        (
          (1,false),
          (1,false),
          (0,false),
          (0,false),
        )
      ),
      Tile::new(
        2,
        "┼".to_string(),
        "┼".to_string(),
        0,
        (
          (1,false),
          (1,false),
          (1,false),
          (1,false),
        )
      ),
      Tile::new(
        3,
        "─".to_string(),
        "─".to_string(),
        0,
        (
          (1,false),
          (0,false),
          (1,false),
          (0,false),
        )
      ),
      Tile::new(
        3,
        "│".to_string(),
        "│".to_string(),
        0,
        (
          (0,false),
          (1,false),
          (0,false),
          (1,false),
        )
      ),
    ]
  }
}

pub struct DirectoryLoader {
  path: PathBuf
}

impl DirectoryLoader {
  pub fn new(path: PathBuf) -> Self { Self { path } }
}

impl TileLoader for DirectoryLoader {

  fn load(&mut self) -> Vec<Tile> {
    // Get a iterator over the directory
    let dir_iter = match fs::read_dir(self.path.clone()) {
      Ok(iter) => iter,
      Err(e) => panic!("{}", e)
    };
    
    // Create the tiles vector
    //let mut tiles: Vec<Tile> =
    // Vec::with_capacity(dir_iter.count().clone());
    let mut tiles: Vec<Tile> = Vec::new();
    // Iterating through the directory of tile models
    let mut id = 0;
    for entry in dir_iter {
      let entry = entry.unwrap();

      // skip file if its not a fbx file
      if entry.path().extension() != Some(&OsStr::new("fbx")) {
        continue;
      }

      // appending tile to 'tiles' array list
      // file naming convention: name_left_back_right_front_turns.fbx
      if let Some(filename) = entry.path().file_stem() {
        let name: Vec<&str> =
          filename.to_str().unwrap().split('_').collect();

        let mut connectors = [
          name[1].to_string(),
          name[2].to_string(),
          name[3].to_string(),
          name[4].to_string()
        ];

        // if any of the connectors end with a 'f' they are flipped
        let mut flip = [false,false,false,false]; 
        for c in 0..connectors.len() {
          if connectors[c].len() > 1 {
            flip[c] = connectors[c].pop() == Some('f')
          };
        }

        // parse connectors
        // rotate by 90° the number of times specified in tile name
        // cycles the connectors around (index 1-4)
        let rotations = name[5].parse::<usize>().unwrap();
        for rot in 0..rotations + 1 {
          let rot_i = [
            (rot + 0) % 4,
            (rot + 1) % 4,
            (rot + 2) % 4,
            (rot + 3) % 4
          ];

          tiles.push(Tile::new(
            id,
            format!("{}_{}", name[0].to_string(), tiles.len().to_string()),
            entry
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
            rot,
            (
              (connectors[rot_i[0]].parse::<u8>().unwrap(),flip[rot_i[0]]),
              (connectors[rot_i[1]].parse::<u8>().unwrap(),flip[rot_i[1]]),
              (connectors[rot_i[2]].parse::<u8>().unwrap(),flip[rot_i[2]]),
              (connectors[rot_i[3]].parse::<u8>().unwrap(),flip[rot_i[3]])
            )
          ));
        }
      };
      id += 1;
    }
    tiles
  }
}

