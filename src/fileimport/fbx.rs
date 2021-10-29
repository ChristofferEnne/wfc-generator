use regex::RegexSet;
use std::{
  fs::File,
  io::{BufRead, BufReader},
  path::Path
};

enum Section {
  Vertice,
  Triangle
}

#[derive(Debug)]
pub(crate) struct FBX {
  vertices: Vec<f32>,
  triangles: Vec<u32>
}

/*
 */

impl FBX {
  pub(crate) fn new() -> Self {
    Self {
      vertices: Vec::new(),
      triangles: Vec::new()
    }
  }

  pub fn load_file(path: &Path) -> FBX {
    let file = File::open(path).unwrap();

    let mut fbx = FBX::new();
    let mut pushing = false;
    let mut section = Section::Triangle;

    let lines = BufReader::new(file).lines();
    for line in lines {
      if let Ok(ip) = line {
        if ip.contains("}") && pushing {
          pushing = false;
        }

        if pushing {
          let mut b: Vec<&str> = Vec::new();
          let s = ip.clone().trim().to_string();
          if let Some(n) = ip.find(" ") {
            b = s.split_at(n).1.split(",").collect();
          } else {
            b = s.split(",").collect();
          }
          match section {
            Section::Vertice => {
              for s in b {
                fbx.vertices.push(s.parse::<f32>().unwrap());
              }
            }
            Section::Triangle => {
              for s in b {
                fbx
                  .triangles
                  .push(s.replace("-", "").parse::<u32>().unwrap());
              }
            }
          }
        }

        if ip.contains("Vertices:") && !pushing {
          section = Section::Vertice;
          pushing = true;
        } else if ip.contains("PolygonVertexIndex:") && !pushing {
          section = Section::Triangle;
          pushing = true;
        }
      }
    }
    fbx
  }

  pub fn parse_file(path: &Path) {
    let scope_start = RegexSet::new(&[r"([[:alpha:]]+):\s+.+\{"]).unwrap();
    let scope_end = RegexSet::new(&[r"\s+\}"]).unwrap();

    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();
    for line in lines {
      if let Ok(l) = line {
        if scope_start.matches(&l).matched_any() {
          println!("# {}", l);
        }
        if scope_end.matches(&l).matched_any() {
          println!("% {}", l);
        }
      }
    }
    // Iterate over and collect all of the matches.
    //let matches: Vec<_> = set.matches("foobar").into_iter().collect();
    //assert_eq!(matches, vec![0, 2, 3, 4, 6]);
  }
}
