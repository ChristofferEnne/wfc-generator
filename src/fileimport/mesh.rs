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
pub(crate) struct Mesh {
  vertices: Vec<f32>,
  triangles: Vec<u32>
}

/*
 */

impl Mesh {
  pub(crate) fn new() -> Self {
    Self {
      vertices: Vec::new(),
      triangles: Vec::new()
    }
  }

  pub fn parse_fbx(path: &Path) -> Mesh {
    //println!("Processing file: {}", path.display());

    // the stack keeps track of the hierarchy of the scopes are are traversing
    let mut stack: Vec<String> = Vec::new();

    let scope_start =
      Regex::new(r"([[:alpha:]][[:alnum:]]*):\s+.+\{").unwrap();
    let scope_end = Regex::new(r"\s*\}").unwrap();
    let idx_array_cap = Regex::new(r"\s+a:\s+([0-9,-]+)").unwrap();
    let f_array_cap = Regex::new(r"\s+a:\s+([0-9,.E-]+)").unwrap();

    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    let mut mesh = Mesh::new();
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
              let caps = f_array_cap.captures(&ln).unwrap();
              //println!("cap: {:?}", caps);
              for s in caps[1].split(',').collect::<Vec<&str>>() {
                mesh.vertices.push(s.parse::<f32>().unwrap());
              }
            }
            "PolygonVertexIndex" => {
              let caps = idx_array_cap.captures(&ln).unwrap();
              //println!("cap: {:?}", caps);
              for s in caps[1].split(',').collect::<Vec<&str>>() {
                if s.starts_with("-") {
                  mesh.triangles.push(s[1..].parse::<u32>().unwrap() - 1);
                } else {
                  mesh.triangles.push(s.parse::<u32>().unwrap());
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
}
