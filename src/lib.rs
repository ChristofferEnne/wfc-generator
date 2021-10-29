use rand::{prelude::StdRng, Rng, SeedableRng};

use hashbrown::HashMap;
use std::fs;
use std::fs::File;
use std::path::PathBuf;
use std::thread::yield_now;
use std::{ffi::OsStr, io::Write};

pub mod dir;
pub mod fileimport;
mod intersection;
pub mod tiles;
use dir::Direction;
use tiles::tile::Tile;

/// Front facing
///
/// Z height
/// │
/// │   Y depth
/// │ /
/// └─────── X width

pub enum PatternSetting {
  FromDirectory(PathBuf),
  PatternBuffer(Vec<Tile>)
}

#[derive()]
pub struct WFC {
  linking: Vec<[Vec<usize>; 6]>,
  patterncount: usize,
  wave: Vec<Vec<usize>>,
  entropy: HashMap<usize, usize>,

  width: usize,
  depth: usize,
  height: usize,

  cellcount: usize,
  seed: u64,

  min_cell: usize,
  base_cell: usize,
  x: usize,
  y: usize,
  z: usize,
  i: usize,
  j: usize,
  v: usize,

  neihbour_cell: usize,
  stack: Vec<usize>,
  possible: Vec<usize>,

  rng: StdRng,

  min_entropy: Vec<usize>,

  keep: Vec<usize>
}

impl WFC {
  pub fn new(
    linking: Vec<[Vec<usize>; 6]>,
    width: usize,
    depth: usize,
    height: usize,
    seed: u64
  ) -> Self {
    // Make sure we have some tiles to work with
    if linking.is_empty() {
      panic!("[wfc-generator] tiles is empty.");
    }

    Self {
      patterncount: linking.len(),
      wave: Vec::with_capacity(depth * width * height),
      entropy: HashMap::with_capacity(depth * width * height),
      cellcount: width * depth * height,

      stack: Vec::with_capacity(depth * width * height),
      keep: Vec::with_capacity(linking.len()),
      possible: Vec::with_capacity(linking.len() * linking.len()),
      rng: StdRng::seed_from_u64(seed),

      linking,
      width,
      depth,
      height,
      seed,

      min_cell: 0,
      base_cell: 0,

      x: 0,
      y: 0,
      z: 0,

      i: 0,
      j: 0,
      v: 0,

      neihbour_cell: 0,
      min_entropy: Vec::with_capacity(depth * width * height)
    }
  }

  pub fn generate(&mut self) -> bool {
    // Wave keeps track of all the available patterns, for each
    // cell. At start start, all patterns are valid anywhere in the Wave so
    // each subarray is a list of indices of all the patterns
    // [cells].[patterns]
    self.wave.clear();

    // Premake a vector with all the tile indexes
    // that we can clone into wave later
    //self.possible = (0..self.tiles.len()).collect();
    self.possible.clear();
    for t in 0..self.patterncount {
      self.possible.push(t);
    }

    // Clone the premade vector into each cell on wave
    for i in 0..self.cellcount {
      self.wave.insert(i, self.possible.clone());
    }
    // Clear the premade vector
    self.possible.clear();

    // Entropy should normally be populated with entropy values.
    // Entropy is just a fancy way to represent the number of patterns
    // still available in a cell. We can skip this computation and populate
    // the array with the number of available patterns instead.
    //
    // At start all patterns are valid anywhere in the Wave, so all cells
    // share the same value (npat). We must however pick one cell at random and
    // assign a lower value to it. Why ? Because the algorithm in draw() needs
    // to find a cell with the minimum non-zero entropy value.
    self.entropy.clear();
    //self.entropy = vec![self.tiles.len(); self.cellcount];
    for i in 0..self.cellcount {
      self.entropy.insert(i, self.patterncount);
    }

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
      self.v = self.wave[self.min_cell][self.v]; // index of selected pattern

      // The Wave's subarray corresponding to the cell with min entropy should
      // now only contains the id of the selected pattern.
      self.wave[self.min_cell].clear();
      self.wave[self.min_cell].push(self.v);

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
        self.iterate();
      }
      //print!("\x1B[2J");
      //self.draw();
      //self.pause();
    }
    // success
    true
  }

  fn iterate(&mut self) {
    // pop() the last index in stack
    // and get the indices of its 4 neighboring cells (W, N, E, S).
    // We have to keep them withing bounds and make sure they wrap around.
    self.base_cell = self.stack.pop().unwrap(); // index of current cell

    self.x = self.index_x(self.base_cell);
    self.y = self.index_y(self.base_cell);
    self.z = self.index_z(self.base_cell);
    for (i, dir) in Direction::iterator().enumerate() {
      self.neihbour_cell = match dir {
        Direction::West => {
          self.coord_index(self.x + self.cellcount - 1, self.y, self.z)
        }
        Direction::North => {
          self.coord_index(self.x, self.y + self.cellcount - 1, self.z)
        }
        Direction::East => {
          self.coord_index(self.x + self.cellcount + 1, self.y, self.z)
        }
        Direction::South => {
          self.coord_index(self.x, self.y + self.cellcount + 1, self.z)
        }
        Direction::Up => {
          self.coord_index(self.x, self.y, self.z + self.cellcount + 1)
        }
        Direction::Down => {
          self.coord_index(self.x, self.y, self.z + self.cellcount - 1)
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
          for pattern in &self.linking[base_tile.clone()][i] {
            self.possible.push(pattern.clone());
          }
        }
        //println!("{:?}", self.possible);

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

        self.i = 0;
        self.j = 0;
        self.possible.sort();
        self.possible.dedup();
        while self.i < self.wave[self.neihbour_cell].len()
          && self.j < self.possible.len()
        {
          if self.wave[self.neihbour_cell][self.i] == self.possible[self.j] {
            self.keep.push(self.wave[self.neihbour_cell][self.i]);
            self.j += 1;
            self.i += 1;
          } else if self.wave[self.neihbour_cell][self.i]
            > self.possible[self.j]
          {
            self.j += 1;
          } else {
            self.i += 1;
          }
        }

        if self.keep.len() < self.v {
          //self.wave[self.neihbour_cell] = std::mem::take(self.keep);
          std::mem::swap(&mut self.wave[self.neihbour_cell], &mut self.keep);

          //if self.wave[self.neihbour_cell].intersect(&self.possible) {
          // If they don't intersect (patterns that could have been placed
          // there but are not available) it means we ran
          // into a "contradiction". We have to stop the whole WFC
          // algorithm.
          if self.wave[self.neihbour_cell].is_empty() {
            self.print_contradiction(self.neihbour_cell);
            panic!("iteration failed");
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
          self
            .entropy
            .insert(self.neihbour_cell, self.wave[self.neihbour_cell].len());

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

  //fn pause(&self) {
  //  let mut stdout = stdout();
  //  stdout.write(b"Press Enter to continue...").unwrap();
  //  stdout.flush().unwrap();
  //  stdin().read(&mut [0]).unwrap();
  //}

  pub fn set_seed(&mut self, seed: u64) {
    self.seed = seed;
  }

  pub fn draw(&self, tiles: &Vec<Tile>) {
    println!("");
    for y in 0..self.depth {
      for x in 0..self.width {
        if self.wave[(x + (y * self.width))].len() > 1 {
          print!("{}", self.wave[(x + (y * self.width))].len());
        } else {
          let t = self.wave[(x + (y * self.width))].iter().next().unwrap();
          print!("{}", tiles[*t].filename);
        }
      }
      println!("");
    }
  }

  pub fn draw_data(&self) {
    println!("");
    for y in 0..self.depth {
      for x in 0..self.width {
        if self.wave[(x + (y * self.width))].len() > 1 {
          print!("#,");
        } else {
          let t = self.wave[(x + (y * self.width))].iter().next().unwrap();
          print!("{},", t);
        }
      }
      println!("");
    }
  }

  pub fn export(&self, path: PathBuf) {
    let display = path.display();

    // Open a file in write-only mode, returns `io::Result<File>`
    let mut file = match File::create(&path) {
      Err(why) => panic!("couldn't create {}: {}", display, why),
      Ok(file) => file
    };

    //// Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
    let mut data: Vec<u8> = Vec::with_capacity(self.cellcount);
    for map in &self.wave {
      data.push(map[0] as u8);
    }

    match file.write_all(&data) {
      Err(why) => panic!("couldn't write to {}: {}", display, why),
      Ok(_) => println!("successfully wrote to {}", display)
    }
  }

  fn print_contradiction(&self, index: usize) {
    println!("contradiction found:");
    let x = self.index_x(index);
    let y = self.index_y(index);
    let z = self.index_z(index);
    println!("x:{} y:{} z:{}", x, y, z);
    for dir in Direction::iterator() {
      let (nx, ny, nz) = match dir {
        Direction::West => (self.index_x(x + self.cellcount - 1), y, z),
        Direction::North => (x, self.index_y(y + self.cellcount - 1), z),
        Direction::East => (self.index_x(x + self.cellcount + 1), y, z),
        Direction::South => (x, self.index_y(y + self.cellcount + 1), z),
        Direction::Up => (x, y, self.index_z(z + self.cellcount + 1)),
        Direction::Down => (x, y, self.index_z(z + self.cellcount - 1))
      };
      println!("nx:{} ny:{} nz:{}", nx, ny, nz);

      let pattern = &self.wave[self.coord_index(nx, ny, nz)];
      match dir {
        Direction::West => {
          println!("West: {:?}", pattern);
        }
        Direction::North => {
          println!("North: {:?}", pattern);
        }
        Direction::East => {
          println!("East: {:?}", pattern);
        }
        Direction::South => {
          println!("South: {:?}", pattern);
        }
        Direction::Up => {
          println!("Up: {:?}", pattern);
        }
        Direction::Down => {
          println!("Down: {:?}", pattern);
        }
      }
    }
  }

  fn index_x(&self, index: usize) -> usize {
    (index % self.width) % self.cellcount
  }

  fn index_y(&self, index: usize) -> usize {
    ((index / self.width) % self.depth) % self.cellcount
  }

  fn index_z(&self, index: usize) -> usize {
    (index / (self.width * self.depth)) % self.cellcount
  }

  fn coord_index(&self, x: usize, y: usize, z: usize) -> usize {
    (x + self.width * y + self.width * self.depth * z) % self.cellcount
  }
}
