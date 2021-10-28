use std::{fs::File, io::{BufRead, BufReader}, path::Path};

enum Section {
  Vertice,
  Triangle
}

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct FBX {
  vertices: Vec<f32>,
  triangles: Vec<u32>
}

impl FBX {
    pub(crate) fn new() -> Self { Self { vertices: Vec::new(), triangles: Vec::new() } }

  pub fn load_file(path: &Path) -> FBX {
    let file = File::open(path).unwrap();

    let mut fbx = FBX::new();
    let mut pushing = false;
    let mut section= Section::Triangle;

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
          }else{
            b = s.split(",").collect();
          }
          match section {
            Section::Vertice => {
              for s in b {
                fbx.vertices.push(s.parse::<f32>().unwrap());
              }
            },
            Section::Triangle => {
              for s in b {
                fbx.triangles.push(s.replace("-", "").parse::<u32>().unwrap());
              }
            },
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
}