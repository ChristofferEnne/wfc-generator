use rand::{
  prelude::{SliceRandom, StdRng, ThreadRng},
  Rng, SeedableRng
};
//use std::collections::HashSet;
use hashbrown::HashSet;
//use std::collections::HashMap,
use hashbrown::HashMap;
use std::{convert::TryInto, fs::File};
use std::{ffi::OsStr, io::Write};
use std::{fmt, slice::Iter};

use std::{fs, path::PathBuf};

pub struct Tile {
  name: String,
  filename: String,
  rotation: usize,
  connectors: (String, String, String, String)
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

enum Direction {
  West,
  North,
  East,
  South
}

impl Direction {
  pub fn iterator() -> Iter<'static, Direction> {
    static DIRECTIONS: [Direction; 4] = [
      Direction::West,
      Direction::North,
      Direction::East,
      Direction::South
    ];
    DIRECTIONS.iter()
  }
}

impl fmt::Debug for Direction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}

impl fmt::Display for Direction {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{:?}", self)
  }
}
pub enum PatternSetting {
  FromDirectory(PathBuf),
  PatternBuffer(Vec<Tile>)
}

pub struct WFC {
  tiles: Vec<Tile>,
  wave: Vec<HashSet<usize>>,
  entropy: HashMap<usize, usize>,
  adjancencies: Vec<[Vec<usize>; 4]>,
  width: usize,
  height: usize,
  cellcount: usize,
  seed: u64
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
          Err(e) => panic!(e)
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
      width,
      height,
      tiles,
      seed
    }
  }

  pub fn setup(&mut self) {
    // Wave keeps track of all the available patterns, for each
    // cell. At start start, all patterns are valid anywhere in the Wave so
    // each subarray is a list of indices of all the patterns
    // [cells].[patterns]
    self.wave = Vec::with_capacity(self.cellcount);
    for i in 0..self.cellcount {
      let mut valids = HashSet::new();
      for t in 0..self.tiles.len() {
        valids.insert(t);
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
    //let H: Vec<usize> = Vec::new();
    self.entropy = HashMap::with_capacity(self.cellcount);
    for i in 0..self.cellcount {
      self.entropy.insert(i, self.tiles.len());
    }

    // replace a random value with a lower one
    //let rngi = self.rng.gen_range(0..self.width*self.height);
    //self.entropy.insert(rngi, 1);
    //self.wave.insert(rngi,
    // vec![self.rng.gen_range(0..ntiles)].into_iter().collect());

    // Array A (for Adjacencies) is an index datastructure that describes the
    // ways that the patterns can be placed near one another. More
    // explanations below
    self.adjancencies = Vec::with_capacity(self.tiles.len());
    for _ in 0..self.tiles.len() {
      self.adjancencies.push([vec![], vec![], vec![], vec![]]);
    }

    //println!("adjancencies: {:?}", adjancencies);

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
    //println!("{:?}", self.adjancencies);
    //println!("adjancencies: {:?}", adjancencies);
  }

  pub fn generate(&mut self) -> bool {
    let mut min_cell = 0;
    let mut base_cell = 0;
    let mut x = 0;
    let mut y = 0;
    let mut v = 0;
    let mut neihbour_cell = 0;
    let mut selected_pattern: usize = 0;
    let mut stack: Vec<usize> = Vec::with_capacity(self.cellcount);
    let mut possible = HashSet::<usize>::with_capacity(self.tiles.len());
    //let mut rng = StdRng::seed_from_u64(self.seed);
    let mut rng = rand::thread_rng();
    // Simple stopping mechanism ---------------------------------------------
    // if entropy list is empty we have collapse all cells
    while !self.entropy.is_empty() {
      //println!("cellcnt: {}", self.cellcount);
      //println!("wavelen: {}", self.wave.len());
      // OBSERVATION
      // ----------------------------------------------------------- Find
      // cell with minimum entropy (not collapsed yet).
      min_cell = self.get_min_random(&mut rng, &self.entropy);
      //println!("min entropy cell: {:?}", min_cell);
      // COLLAPSE --------------------------------------------------------------
      // Among the patterns available in the selected cell (the one with min
      // entropy), select one pattern randomly, weighted by the frequency
      // that pattern appears in the input image.
      v = self.wave[min_cell].len();
      v = rng.gen_range(0..v);
      selected_pattern =
        *self.wave[min_cell].iter().collect::<Vec<&usize>>()[v]; // index of selected pattern

      //println!("we lock in pattern: {:?}", selected_pattern);

      //let id: usize = self.wave[&min_cell].iter().next().unwrap().clone();
      // // index of selected pattern

      // The Wave's subarray corresponding to the cell with min entropy should
      // now only contains the id of the selected pattern
      //self.wave[&min_cell].clear();
      //self.wave[&min_cell].insert(selected_pattern);
      self.wave[min_cell] = vec![selected_pattern].into_iter().collect();

      // remove min_cell from entropy to collapse it.
      self.entropy.remove(&min_cell);

      // PROPAGATION ----------------------------------------------------------
      // Once a cell is collapsed, its index is put in a stack.
      // That stack is meant later to temporarily store indices of neighoring
      // cells
      stack.clear();
      stack.push(min_cell);

      // The propagation will last as long as that stack is filled with indices
      while !stack.is_empty() {
        //println!("stack: {:?}", stack);
        // pop() the last index in stack
        // and get the indices of its 4 neighboring cells (W, N, E, S).
        // We have to keep them withing bounds and make sure they wrap around.
        base_cell = stack.pop().unwrap(); // index of current cell

        //println!("iwidth {:?} iheight {:?} id_c {:?} iid_c {:?}", iw, ih,
        // base_cell, icc);
        x = base_cell % self.width;
        y = base_cell / self.width;
        for (i, dir) in Direction::iterator().enumerate() {
          neihbour_cell = match dir {
            Direction::West => {
              (x + self.cellcount - 1) % self.width + y * self.width
            }
            Direction::North => {
              x + ((y + self.cellcount - 1) % self.height) * self.width
            }
            Direction::East => {
              (x + self.cellcount + 1) % self.width + y * self.width
            }
            Direction::South => {
              x + ((y + self.cellcount + 1) % self.height) * self.width
            }
          };
          // index of negihboring cell
          //println!("dirnr {:?} x {:?}({:?}) y {:?}({:?}) id_n {:?}", dir, x,
          // coord.0, y, coord.1, neihbour_cell);

          // We make sure the neighboring cell is not collapsed yet
          // (we don't want to update a cell that has only 1 pattern available)
          if self.entropy.contains_key(&neihbour_cell) {
            // Then we check all the patterns that COULD be placed at that
            // location. So all the patterns that fit with the base
            // cell.

            //EX: if the neighboring cell is on the left of the current cell
            // (east side), we look at all the patterns that can be
            // placed on the left of each pattern contained in the
            // current cell.
            possible.clear();
            // for all possible tiles in base_cell
            for base_tile in &self.wave[base_cell] {
              //print!("dis tiles: {} ", self.tiles[*base_tile].filename);
              //print!("pos tiles: ");
              // for all tiles that can connect to base_tiles
              for pattern in &self.adjancencies[base_tile.clone()][i] {
                possible.insert(pattern.clone());
                //print!(r#""{}" "#, self.tiles[*pattern].filename);
              }
            }
            //println!("");

            // We also look at the patterns that ARE available in the
            // neighboring cell

            // Now we make sure that the neighboring cell really need to be
            // updated. If all its available patterns are already
            // in the list of all the possible patterns:
            // —> there’s no need to update it
            // (the algorithm skip this neighbor and goes on to the next)
            //println!("subset: {:?} - {:?}", possible,
            // self.wave[&neihbour_cell]);
            if !self.wave[neihbour_cell].is_subset(&possible) {
              //println!("is subset");

              // If it is not a subset of the possible list:
              // —> we look at the intersection of the two sets (all the
              // patterns that can be placed at that location and
              // that, "luckily", are available at that same location)
              let mut intersection = HashSet::with_capacity(self.wave.len());
              let mut set = self.wave.get_mut(neihbour_cell);
              for i in self.wave[neihbour_cell].intersection(&possible) {
                intersection.insert(i.clone());
              }

              //println!("intersection: {:?}", intersection);

              // If they don't intersect (patterns that could have been placed
              // there but are not available) it means we ran
              // into a "contradiction". We have to stop the whole WFC
              // algorithm.
              if intersection.is_empty() {
                self.print_contradiction(base_cell);
                return false;
              }

              // If, on the contrary, they do intersect -> we update the
              // neighboring cell with that refined
              // list of pattern's indices
              self.wave[neihbour_cell] = intersection;

              // Because that neighboring cell has been updated, its number of
              // valid patterns has decreased and its entropy
              // must be updated accordingly. Note that we're
              // subtracting a small random value to mix things up: sometimes
              // cells we'll end-up with the same minimum entropy
              // value and this prevent to always select the first one of them.
              // It's a cosmetic trick to break the monotony of the animation
              self
                .entropy
                .insert(neihbour_cell, self.wave[neihbour_cell].len());

              // Finally, and most importantly, we add the index of that
              // neighboring cell to the stack so it becomes the
              // next current cell in turns (the one whose neighbors will be
              // updated during the next while loop)
              stack.push(neihbour_cell);
              //println!("pushed to stack");
            }
          } else {
            //println!("does not contain key");
          }
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
    //let mut drawing: Vec<String> = vec!["".to_string()];
    for y in 0..self.height {
      for x in 0..self.width {
        if self.wave[(x + (y * self.width))].len() > 1 {
          print!("{}", self.wave[(x + (y * self.width))].len());
        } else {
          let t = self.wave[(x + (y * self.width))].iter().next().unwrap();
          //drawing.push(self.tiles[*t].filename.clone());
          print!("{}", self.tiles[*t].filename);
        }
      }
      //drawing.push(r#"\r\n"#.to_string());
      println!("");
    }
    //println!("{}", drawing.join(""));
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

  //pub fn export_bytes(&self, path: PathBuf) {
  //  let display = path.display();
  //
  //  // Open a file in write-only mode, returns `io::Result<File>`
  //  let mut file = match File::create(&path) {
  //    Err(why) => panic!("couldn't create {}: {}", display, why),
  //    Ok(file) => file
  //  };
  //
  //  // Write the `LOREM_IPSUM` string to `file`, returns `io::Result<()>`
  //  let mut data: Vec<u8> = Vec::with_capacity(self.cellcount);
  //  for map in &self.wave {
  //    data.push(map.0 as u8);
  //  }
  //
  //  match file.write_all(&data) {
  //    Err(why) => panic!("couldn't write to {}: {}", display, why),
  //    Ok(_) => println!("successfully wrote to {}", display)
  //  }
  //}

  fn get_min_random(
    &self,
    rng: &mut ThreadRng,
    map: &HashMap<usize, usize>
  ) -> usize {
    let mut min = usize::MAX;
    let mut min_index = vec![];

    for (index, val) in map {
      if *val < min {
        min = *val;
        min_index.clear();
      }
      if *val <= min {
        min_index.push(*index);
      }
    }
    min_index.shuffle(rng);

    //println!("min entropy is {:?} at cell {:?}", min, min_index);
    min_index[0]
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

trait Sub {
  fn is_subset_vec(&self, subset: Vec<usize>) -> bool;
}

impl Sub for HashSet<usize> {
  fn is_subset_vec(&self, subset: Vec<usize>) -> bool {
    for i in &subset {
      if !self.contains(i) {
        return false;
      }
    }
    true
  }
}
