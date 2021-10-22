use rand::{
  prelude::{SliceRandom, StdRng, ThreadRng},
  Rng, SeedableRng
};
//use std::collections::HashSet;
use hashbrown::HashSet;
//use std::collections::HashMap,
use hashbrown::HashMap;
use std::{ffi::OsStr, io::Write};
use std::{fmt::format, fs::File};

use std::{fs, path::PathBuf};

pub mod dir;
mod intersection;
pub mod tile;
use crate::dir::Direction;
use crate::intersection::Intersect;
use crate::tile::Tile;


pub enum PatternSetting {
  FromDirectory(PathBuf),
  PatternBuffer(Vec<Tile>)
}

#[derive()]
pub struct WFC {
  tiles: Vec<Tile>,
  wave: Vec<Vec<usize>>,
  entropy: HashMap<usize, usize>,
  adjancencies: Vec<[Vec<usize>; 4]>,
  width: usize,
  height: usize,
  cellcount: usize,
  seed: u64,

  min_cell: usize,
  base_cell: usize,
  x: usize,
  y: usize,
  i: usize,
  j: usize,
  v: usize,
  neihbour_cell: usize,
  selected_pattern: usize,
  stack: Vec<usize>,
  possible: Vec<usize>,
  rng: StdRng,

  min_entropy: Vec<usize>,

  keep: Vec<usize>
}

impl WFC {
  pub fn new(
    pattern_setting: PatternSetting,
    width: usize,
    height: usize,
    seed: u64
  ) -> Self {
    let tiles = match pattern_setting {
      PatternSetting::FromDirectory(path) => {
        // Get a iterator over the directory
        let dir_iter = match fs::read_dir(path) {
          Ok(iter) => iter,
          Err(e) => panic!("{}", e)
        };

        // Create the tiles vector
        //let mut tiles: Vec<Tile> =
        // Vec::with_capacity(dir_iter.count().clone());
        let mut tiles: Vec<Tile> = Vec::new();

        // Iterating through the directory of tile models
        for entry in dir_iter {
          let entry = entry.unwrap();

          // skip file if its not a fbx file
          if entry.path().extension() != Some(&OsStr::new("fbx")) {
            println!("file.");
            continue;
          }

          // appending tile to 'tiles' array list
          // file naming convention: name_left_back_right_front_turns.fbx
          if let Some(filename) = entry.path().file_stem() {
            let name: Vec<&str> =
              filename.to_str().unwrap().split('_').collect();

            // rotate by 90° the number of times specified in tile name
            // cycles the connectors around (index 1-4)
            let rotations = name[5].parse::<usize>().unwrap();
            for rot in 0..rotations + 1 {
              tiles.push(Tile::new(
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
                  name[(rot + 0) % 4 + 1].to_string(),
                  name[(rot + 1) % 4 + 1].to_string(),
                  name[(rot + 2) % 4 + 1].to_string(),
                  name[(rot + 3) % 4 + 1].to_string()
                )
              ));
            }
          };
        }
        tiles
      }
      PatternSetting::PatternBuffer(buffer) => buffer
    };

    Self {
      adjancencies: Vec::with_capacity(tiles.len()),
      wave: Vec::with_capacity(height * width),
      entropy: HashMap::with_capacity(height * width),
      cellcount: width * height,

      stack: Vec::with_capacity(height * width),
      keep: Vec::with_capacity(tiles.len()),
      possible: Vec::with_capacity(tiles.len()),
      rng: StdRng::seed_from_u64(seed),

      width,
      height,
      tiles,
      seed,

      min_cell: 0,
      base_cell: 0,
      x: 0,
      y: 0,
      i: 0,
      j: 0,
      v: 0,
      neihbour_cell: 0,
      selected_pattern: 0,
      //rng: rand::thread_rng(),
      min_entropy: Vec::with_capacity(height * width)
    }
  }

  pub fn setup(&mut self) {
    // Wave keeps track of all the available patterns, for each
    // cell. At start start, all patterns are valid anywhere in the Wave so
    // each subarray is a list of indices of all the patterns
    // [cells].[patterns]
    self.wave = Vec::with_capacity(self.cellcount);
    for i in 0..self.cellcount {
      let mut valids = Vec::new();
      for t in 0..self.tiles.len() {
        valids.push(t);
      }
      //println!("valids: {:?}", valids);
      self.wave.insert(i, valids);
    }

    // Entropy should normally be populated with entropy values.
    // Entropy is just a fancy way to represent the number of patterns
    // still available in a cell. We can skip this computation and populate
    // the array with the number of available patterns instead.
    //
    // At start all patterns are valid anywhere in the Wave, so all cells
    // share the same value (npat). We must however pick one cell at random and
    // assign a lower value to it. Why ? Because the algorithm in draw() needs
    // to find a cell with the minimum non-zero entropy value.
    self.entropy = HashMap::with_capacity(self.cellcount);
    for i in 0..self.cellcount {
      self.entropy.insert(i, self.tiles.len());
    }

    // Array A (for Adjacencies) is an index datastructure that describes the
    // ways that the patterns can be placed near one another. More
    // explanations below
    self.adjancencies = Vec::with_capacity(self.tiles.len());
    for _ in 0..self.tiles.len() {
      self.adjancencies.push([vec![], vec![], vec![], vec![]]);
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
    for key in 0..self.tiles.len() {
      let selected = &self.tiles[key];
      for other_index in 0..self.tiles.len() {
        let other = &self.tiles[other_index];
        // (in case when N = 3) If the first two columns of pattern 1 == the
        // last two columns of pattern 2 --> pattern 2 can be placed to
        // the left (0) of pattern 1
        if selected.connectors.0 == other.connectors.2 {
          self.adjancencies[key][0].push(other_index);
        }
        if selected.connectors.1 == other.connectors.3 {
          self.adjancencies[key][1].push(other_index);
        }
        if selected.connectors.2 == other.connectors.0 {
          self.adjancencies[key][2].push(other_index);
        }
        if selected.connectors.3 == other.connectors.1 {
          self.adjancencies[key][3].push(other_index);
        }
      }
    }
  }

  pub fn generate(&mut self) -> bool {
    // Simple stopping mechanism
    // if entropy list is empty we have collapse all cells
    while !self.entropy.is_empty() {
      // OBSERVATION

      //Find cell with minimum entropy (not collapsed yet).
      self.v = usize::MAX;
      self.min_entropy.clear();
      self.min_cell = {
        for (index, val) in &self.entropy {
          if val < &self.v {
            self.v = *val;
            self.min_entropy.clear();
          }
          if val <= &self.v {
            self.min_entropy.push(*index);
          }
        }
        self.v = self.rng.gen_range(0..self.min_entropy.len());
        self.min_entropy[self.v]
      };

      // COLLAPSE

      // Among the patterns available in the selected cell (the one with min
      // entropy), select one pattern randomly, weighted by the frequency
      // that pattern appears in the input image.
      self.v = self.wave[self.min_cell].len();
      self.v = self.rng.gen_range(0..self.v);
      self.selected_pattern =
        *self.wave[self.min_cell].iter().collect::<Vec<&usize>>()[self.v]; // index of selected pattern


      // // index of selected pattern

      // The Wave's subarray corresponding to the cell with min entropy should
      // now only contains the id of the selected pattern
      self.wave[self.min_cell] =
        vec![self.selected_pattern].into_iter().collect();

      // remove min_cell from entropy to collapse it.
      self.entropy.remove(&self.min_cell);

      // PROPAGATION ----------------------------------------------------------
      // Once a cell is collapsed, its index is put in a stack.
      // That stack is meant later to temporarily store indices of neighoring
      // cells
      self.stack.clear();
      self.stack.push(self.min_cell);

      // The propagation will last as long as that stack is filled with indices
      while !self.stack.is_empty() {
        // pop() the last index in stack
        // and get the indices of its 4 neighboring cells (W, N, E, S).
        // We have to keep them withing bounds and make sure they wrap around.
        self.base_cell = self.stack.pop().unwrap(); // index of current cell

        self.x = self.base_cell % self.width;
        self.y = self.base_cell / self.width;
        for (i, dir) in Direction::iterator().enumerate() {
          self.neihbour_cell = match dir {
            Direction::West => {
              (self.x + self.cellcount - 1) % self.width + self.y * self.width
            }
            Direction::North => {
              self.x
                + ((self.y + self.cellcount - 1) % self.height) * self.width
            }
            Direction::East => {
              (self.x + self.cellcount + 1) % self.width + self.y * self.width
            }
            Direction::South => {
              self.x
                + ((self.y + self.cellcount + 1) % self.height) * self.width
            }
          };

          // index of negihboring cell

          // We make sure the neighboring cell is not collapsed yet
          // (we don't want to update a cell that has only 1 pattern available)
          if self.entropy.contains_key(&self.neihbour_cell) {
            // Then we check all the patterns that COULD be placed at that
            // location. So all the patterns that fit with the base
            // cell.

            //EX: if the neighboring cell is on the left of the current cell
            // (east side), we look at all the patterns that can be
            // placed on the left of each pattern contained in the
            // current cell.
            self.possible.clear();
            // for all possible tiles in base_cell
            for base_tile in &self.wave[self.base_cell] {
              // for all tiles that can connect to base_tiles
              for pattern in &self.adjancencies[base_tile.clone()][i] {
                self.possible.push(pattern.clone());
              }
            }

            // We also look at the patterns that ARE available in the
            // neighboring cell

            // Now we make sure that the neighboring cell really need to be
            // updated. If all its available patterns are already
            // in the list of all the possible patterns:
            // —> there’s no need to update it
            // (the algorithm skip this neighbor and goes on to the next)

            // there are any patterns from the cell missing in possible
            // we must update this cell.

            // If it is not a subset of the possible list:
            // —> we look at the intersection of the two sets (all the
            // patterns that can be placed at that location and
            // that, "luckily", are available at that same location)
            self.v = self.wave[self.neihbour_cell].len();
            //for entry in &self.wave[self.neihbour_cell] {
            //  if self.possible.contains(&entry) {
            //    self.keep.push(*entry);
            //  }
            //}

            self.i = 0;
            self.j = 0;
            self.possible.sort();
            self.possible.dedup();
            while self.i < self.wave[self.neihbour_cell].len() && self.j < self.possible.len() {
              if self.wave[self.neihbour_cell][self.i] == self.possible[self.j] {
                self.keep.push(self.wave[self.neihbour_cell][self.i]);
                self.j+=1;
                self.i+=1;
              } else if self.wave[self.neihbour_cell][self.i] > self.possible[self.j] {
                self.j+=1;
              } else {
                self.i+=1;
              }
            }

            if self.keep.len() < self.v {
              //self.wave[self.neihbour_cell] = std::mem::take(self.keep);
              std::mem::swap(
                &mut self.wave[self.neihbour_cell],
                &mut self.keep
              );

              //if self.wave[self.neihbour_cell].intersect(&self.possible) {
              // If they don't intersect (patterns that could have been placed
              // there but are not available) it means we ran
              // into a "contradiction". We have to stop the whole WFC
              // algorithm.
              if self.wave[self.neihbour_cell].is_empty() {
                self.print_contradiction(self.base_cell);
                return false;
              }

              // If, on the contrary, they do intersect -> we update the
              // neighboring cell with that refined
              // list of pattern's indices

              // Because that neighboring cell has been updated, its number of
              // valid patterns has decreased and its entropy
              // must be updated accordingly. Note that we're
              // subtracting a small random value to mix things up: sometimes
              // cells we'll end-up with the same minimum entropy
              // value and this prevent to always select the first one of them.
              // It's a cosmetic trick to break the monotony of the animation
              self.entropy.insert(
                self.neihbour_cell,
                self.wave[self.neihbour_cell].len()
              );

              // Finally, and most importantly, we add the index of that
              // neighboring cell to the stack so it becomes the
              // next current cell in turns (the one whose neighbors will be
              // updated during the next while loop)
              self.stack.push(self.neihbour_cell);
            }
          } else {
            //println!("does not contain key");
          }
          self.keep.clear();
        }
      }
      //print!("\x1B[2J");
      //self.draw();
      //self.pause();
    }
    // success
    true
  }

  //fn pause(&self) {
  //  let mut stdout = stdout();
  //  stdout.write(b"Press Enter to continue...").unwrap();
  //  stdout.flush().unwrap();
  //  stdin().read(&mut [0]).unwrap();
  //}

  pub fn set_seed(&mut self, seed: u64) {
    self.seed = seed;
  }

  pub fn draw(&self) {
    println!("");
    for y in 0..self.height {
      for x in 0..self.width {
        if self.wave[(x + (y * self.width))].len() > 1 {
          print!("{}", self.wave[(x + (y * self.width))].len());
        } else {
          let t = self.wave[(x + (y * self.width))].iter().next().unwrap();
          print!("{}", self.tiles[*t].filename);
        }
      }
      println!("");
    }
  }

  pub fn export_csv(&self, path: PathBuf) {
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
      Err(why) => panic!("couldn't create {}: {}", display, why),
      Ok(file) => file
    };

    // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    let mut data = Vec::new();
    data.push(format!("row name,filename,rotation,connectors"));
    for tile in &self.tiles {
      data.push(format!(
        r#"{},{},{},"{},{},{},{}""#,
        tile.name,
        tile.filename,
        tile.rotation,
        tile.connectors.0,
        tile.connectors.1,
        tile.connectors.2,
        tile.connectors.3
      ));
    }

    match file.write_all(data.join("\n").as_bytes()) {
      Err(why) => panic!("couldn't write to {}: {}", display, why),
      Ok(_) => println!("successfully wrote to {}", display)
    }
  }

  pub fn export_bytes(&self, path: PathBuf) {
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
      Err(why) => panic!("couldn't create {}: {}", display, why),
      Ok(file) => file
    };

    let mut data1: Vec<String> = Vec::new();
    let mut data2: Vec<String> = Vec::with_capacity(self.cellcount);
    data1.push(format!("row name,cells"));
    for cell in &self.wave {
      data2.push(cell[0].to_string());
    }
    data1.push(format!(r#"data,"{}""#, data2.join(",")));

    //// Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    //let mut data: Vec<u8> = Vec::with_capacity(self.cellcount);
    //for map in &self.wave {
    //  data.push(map.0 as u8);
    //}

    match file.write_all(data1.join("\n").as_bytes()) {
      Err(why) => panic!("couldn't write to {}: {}", display, why),
      Ok(_) => println!("successfully wrote to {}", display)
    }
  }

  fn print_contradiction(&self, index: usize) {
    println!("contradiction found:");
    let x = index % self.width;
    let y = index / self.width;
    println!("x{} y{}", x, y);
    for dir in Direction::iterator() {
      let (nx, ny) = match dir {
        Direction::West => ((x + self.cellcount - 1) % self.width, y),
        Direction::North => (x, (y + self.cellcount - 1) % self.width),
        Direction::East => ((x + self.cellcount + 1) % self.width, y),
        Direction::South => (x, (y + self.cellcount + 1) % self.width)
      };
      println!("nx{} ny{}", nx, ny);
      println!("{} - {:?}", (nx + (ny * self.width)), self.wave.len());
      println!("{} - {:?}", dir, self.wave[(nx + ny * self.width)]);
    }
  }
}
