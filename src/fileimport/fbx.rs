use regex::{Captures, Regex, RegexSet};
use std::{fs::File, io::{BufRead, BufReader}, ops::RangeBounds, path::Path};

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
    let mut stack: Vec<String> = Vec::new();
    let scope_start = Regex::new(r"([[:alpha:]]+):\s+.+\{").unwrap();
    let scope_end = Regex::new(r"\s+\}").unwrap();

    let vertrices_match = Regex::new(r"Vertices:\s+\*[0-9]s+\{").unwrap();
    let polygonvertexindex_match = Regex::new(r"PolygonVertexIndex:\s+\*[0-9]s+\{").unwrap();
    let array_cap = Regex::new(r"\s+a:\s+([[:alnum:]]+)").unwrap();

    let file = File::open(path).unwrap();
    let lines = BufReader::new(file).lines();

    let mut mesh = FBX::new();
    for line in lines {
      if let Ok(l) = line {
        let ln = l.to_string();
        if stack.len() == 3 {
          match *stack.last().unwrap() {
            "Vertices" => {
              let caps = array_cap.captures(&ln).unwrap();
              for s in caps[0].split(',').collect::<Vec<&str>>() {
                mesh.vertices.push(s.parse::<f32>().unwrap());
              }
            },
            "PolygonVertexIndex" => {
              let caps = array_cap.captures(&ln).unwrap();
              for s in caps[0].split(',').collect::<Vec<&str>>() {
                if s.contains('-') {
                  mesh.vertices.push(s.replace("-", "").parse::<f32>().unwrap() -1_f32);
                }
              }
            },
            _ => {}
          }
        } else {
          let caps_start = scope_start.captures(&ln).unwrap();
          if let Some(cap) = caps_start.get(1).clone() {
              stack.push(cap.as_str());
          } else {
            let caps_end = scope_end.captures(&ln).unwrap();
            if let Some(_cap) = caps_end.get(1) {
              stack.pop();
            }
          }
        }
      }
    }

    
    // Iterate over and collect all of the matches.
    //let matches: Vec<_> = set.matches("foobar").into_iter().collect();
    //assert_eq!(matches, vec![0, 2, 3, 4, 6]);
  }
}
