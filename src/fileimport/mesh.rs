use regex::{Captures, Regex, RegexSet};
use std::{
  fs::File,
  io::{BufRead, BufReader},
  ops::RangeBounds,
  path::Path
};

enum Section {
  Vertice,
  Triangle
}


#[derive(Debug)]
pub(crate) struct Vertex {
  x: f32,
  y: f32,
  z: f32,
}

impl Vertex {
    pub(crate) fn new(x: f32, y: f32, z: f32) -> Self { Self { x, y, z } }
}

#[derive(Debug)]
pub(crate) struct Surface {
  indexes: Vec<u32>,
}

impl Surface {
    pub(crate) fn new() -> Self { Self { indexes: Vec::new() } }
}

#[derive(Debug)]
pub(crate) struct Mesh {
  vertices: Vec<Vertex>,
  surfaces: Vec<Surface>
}

impl Mesh {
  pub(crate) fn new() -> Self {
    Self {
      vertices: Vec::new(),
      surfaces: Vec::new()
    }
  }

  pub fn parse_fbx(path: &Path) -> Mesh {
    //println!("Processing file: {}", path.display());

    // the stack keeps track of the hierarchy of the scopes are are traversing
    let mut stack: Vec<String> = Vec::new();

    let scope_start =
      Regex::new(r"([[:alpha:]][[:alnum:]]*):\s+.+\{").unwrap();
    let scope_end = Regex::new(r"\s*\}").unwrap();
    let f_array_cap_v = Regex::new(r"
    (?P<x>[0-9.E-]+),
    (?P<y>[0-9.E-]+),
    (?P<z>[0-9.E-]+)").unwrap();
    let idx_array_cap = Regex::new(r"\s+a:\s+([0-9,-]+)").unwrap();
    
    //let f_array_cap = Regex::new(r"\s+a:\s+([0-9,.E-]+)").unwrap();

    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    let mut mesh = Mesh::new();
    let mut surface = Surface::new();
    for line in lines {
      if let Ok(l) = line {
        let ln = l.to_string();

        // look for scope ends
        if scope_end.is_match(&ln) {
          stack.pop();
          //println!("Stack: {:?}", stack);
          continue;
        }

        if stack.len() == 3 {
          let m: &str = stack.last().unwrap().as_ref();
          match stack.last().unwrap().as_ref() {
            "Vertices" => {
              //let caps = f_array_cap.captures(&ln).unwrap();
              for caps in f_array_cap_v.captures_iter(&ln) {
                println!("X: {:?}, Y: {:?}, Z: {:?}",
                  &caps["x"], &caps["y"], &caps["z"]);
                  mesh.vertices.push(Vertex::new(
                    caps["x"].parse::<f32>().unwrap(), 
                    caps["y"].parse::<f32>().unwrap(), 
                    caps["z"].parse::<f32>().unwrap()) );
              }

              //println!("cap: {:?}", caps);
              //for s in caps[1].split(',').collect::<Vec<&str>>() {
              //  mesh.vertices.push(s.parse::<f32>().unwrap());
              //}
            }
            "PolygonVertexIndex" => {
              let caps = idx_array_cap.captures(&ln).unwrap();
              //println!("cap: {:?}", caps);
              for s in caps[1].split(',').collect::<Vec<&str>>() {
                if s.starts_with("-") {
                  surface.indexes.push(s[1..].parse::<u32>().unwrap() - 1);
                  mesh.surfaces.push(surface);
                  surface = Surface::new();
                } else {
                  surface.indexes.push(s.parse::<u32>().unwrap());
                }
              }
            }
            _ => {}
          }
        }

        // look for scope starts
        match scope_start.captures(&ln) {
          Some(cap) => {
            if let Some(cap) = cap.get(1) {
              stack.push(cap.as_str().to_string());
              //println!("Stack: {:?}", stack);
            }
          }
          None => {}
        }
      }
    }

    println!("{:?}", mesh);
    mesh
  }

  pub fn get_links() {

  }
}
