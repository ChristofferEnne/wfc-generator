use std::{ffi::OsStr, fs::{self, File}, io::Write, path::{self, PathBuf}};

use hashbrown::HashMap;

use crate::tiles::tile::Tile;

pub trait TileLoader {
  fn tiles(&self) -> &Vec<Tile>;
  fn links(&self) -> &Vec<[Vec<usize>; 4]>;
  fn set_link(self, links: Vec<[Vec<usize>; 4]>);
  fn load(&mut self) -> Vec<[Vec<usize>; 4]>;

  fn linking(&mut self) -> Vec<[Vec<usize>; 4]> {
    let mut flip_required: HashMap<usize, bool> = HashMap::new();
    for tile in self.tiles() {
      if tile.connectors.0.1 {
        flip_required.insert(tile.connectors.0.0, true);
      }else if !flip_required.contains_key(&tile.connectors.0.0) {
        flip_required.insert(tile.connectors.0.0, false);
      }
      if tile.connectors.1.1 {
        flip_required.insert(tile.connectors.1.0, true);
      }else if !flip_required.contains_key(&tile.connectors.1.0) {
        flip_required.insert(tile.connectors.1.0, false);
      }
      if tile.connectors.2.1 {
        flip_required.insert(tile.connectors.2.0, true);
      }else if !flip_required.contains_key(&tile.connectors.2.0) {
        flip_required.insert(tile.connectors.2.0, false);
      }
      if tile.connectors.3.1 {
        flip_required.insert(tile.connectors.3.0, true);
      }else if !flip_required.contains_key(&tile.connectors.3.0) {
        flip_required.insert(tile.connectors.3.0, false);
      }
    }

    println!("{:?} - {:?}", flip_required.len(), flip_required);

    // Array A (for Adjacencies) is an index datastructure that describes the
    // ways that the patterns can be placed near one another. More
    // explanations below
    let mut adjancencies: Vec<[Vec<usize>; 4]> = Vec::with_capacity(self.tiles().len());
    for _ in 0..self.tiles().len() {
      adjancencies.push([vec![], vec![], vec![], vec![]]);
    }

    // Computation of patterns compatibilities (check if some patterns are
    // adjacent, if so -> store them based on their location)

    // EXAMPLE:
    //  If pattern index 42 can placed to the right of pattern index 120,
    //  we will store this adjacency rule as follow:
    //
    //  A[120][1].add(42)
    //
    //  Here '1' stands for 'right' or 'East'/'E'
    //
    //  0 = left or West/W
    //  1 = right or East/E
    //  2 = up or North/N
    //  3 = down or South/S

    // Comparing patterns to each other
    for (i, tile) in self.tiles().iter().enumerate() {
      for (n, ntile) in self.tiles().iter().enumerate() {
        // (in case when N = 3) If the first two columns of pattern 1 == the
        // last two columns of pattern 2 --> pattern 2 can be placed to
        // the left (0) of pattern 1
        if *flip_required.get(&tile.connectors.0.0).unwrap() {
          if tile.connectors.0.0 == ntile.connectors.2.0 && tile.connectors.0.1 != ntile.connectors.2.1 {
            adjancencies[i][0].push(n);
          }
          if tile.connectors.1.0 == ntile.connectors.3.0 && tile.connectors.1.1 != ntile.connectors.3.1  {
            adjancencies[i][1].push(n);
          }
          if tile.connectors.2.0 == ntile.connectors.0.0 && tile.connectors.2.1 != ntile.connectors.0.1  {
            adjancencies[i][2].push(n);
          }
          if tile.connectors.3.0 == ntile.connectors.1.0 && tile.connectors.3.1 != ntile.connectors.1.1  {
            adjancencies[i][3].push(n);
          }
        } else {
          if tile.connectors.0.0 == ntile.connectors.2.0 {
            adjancencies[i][0].push(n);
          }
          if tile.connectors.1.0 == ntile.connectors.3.0 {
            adjancencies[i][1].push(n);
          }
          if tile.connectors.2.0 == ntile.connectors.0.0 {
            adjancencies[i][2].push(n);
          }
          if tile.connectors.3.0 == ntile.connectors.1.0 {
            adjancencies[i][3].push(n);
          }
        }
      }
    }
    adjancencies
  }

  fn export(&mut self, path: PathBuf) {
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
      Err(why) => panic!("couldn't create {}: {}", display, why),
      Ok(file) => file
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    let mut data = Vec::new();
    data.push(format!("row name,id,filename,rotation,connectors,flip"));
    for tile in self.tiles() {
      data.push(format!(
        r#"{},{},{},{},"{},{},{},{}","{},{},{},{}""#,
        tile.name,
        tile.id,
        tile.filename,
        tile.rotation,
        tile.connectors.0.0,
        tile.connectors.1.0,
        tile.connectors.2.0,
        tile.connectors.3.0,
        tile.connectors.0.1,
        tile.connectors.1.1,
        tile.connectors.2.1,
        tile.connectors.3.1
      ));
    }

    match file.write_all(data.join("\n").as_bytes()) {
      Err(why) => panic!("couldn't write to {}: {}", display, why),
      Ok(_) => println!("successfully wrote to {}", display)
    }
  }
}

pub struct TestLoader {
  tiles: Vec<Tile>,
  links: Vec<[Vec<usize>; 4]>,
}

impl  TestLoader {
  pub fn new() -> Self { Self {tiles: Vec::new(), links: Vec::new()} }
}

impl TileLoader for TestLoader {
  fn load(&mut self) -> Vec<[Vec<usize>; 4]> {
    // 0 none
    // 1 pipe
    self.tiles = vec![
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
    ];
    self.linking()
  }

  fn tiles(&self) -> &Vec<Tile> {
    &self.tiles
  }

  fn links(&self) -> &Vec<[Vec<usize>; 4]>{
    &self.links
  }

  fn set_link(mut self, links: Vec<[Vec<usize>; 4]>){
    self.links = links;
  }
}

pub struct DirectoryLoader {
  path: PathBuf,
  tiles: Vec<Tile>,
  links: Vec<[Vec<usize>; 4]>,
}

impl DirectoryLoader {
  pub fn new(path: PathBuf) -> Self { 
    Self { 
      path, 
      tiles: Vec::new(), 
      links: Vec::new() 
    } 
  }
}

impl TileLoader for DirectoryLoader {
  fn tiles(&self) -> &Vec<Tile> {
    &self.tiles
  }

  fn links(&self) -> &Vec<[Vec<usize>; 4]>{
    &self.links
  }
  
  fn set_link(mut self, links: Vec<[Vec<usize>; 4]>){
    self.links = links;
  }

  fn load(&mut self) -> Vec<[Vec<usize>; 4]> {
    // Get a iterator over the directory
    let dir_iter = match fs::read_dir(self.path.clone()) {
      Ok(iter) => iter,
      Err(e) => panic!("{}", e)
    };
    
    // Create the tiles vector
    self.tiles = Vec::new();

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

          self.tiles.push(Tile::new(
            id,
            format!("{}_{}", name[0].to_string(), self.tiles.len().to_string()),
            entry
            .path()
            .file_name()
            .unwrap()
            .to_str()
            .unwrap()
            .to_string(),
            rot,
            (
              (connectors[rot_i[0]].parse::<usize>().unwrap(),flip[rot_i[0]]),
              (connectors[rot_i[1]].parse::<usize>().unwrap(),flip[rot_i[1]]),
              (connectors[rot_i[2]].parse::<usize>().unwrap(),flip[rot_i[2]]),
              (connectors[rot_i[3]].parse::<usize>().unwrap(),flip[rot_i[3]])
            )
          ));
        }
      };
      id += 1;
    }
    self.linking()
  }
}

