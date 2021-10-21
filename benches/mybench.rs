use criterion::{criterion_group, criterion_main, Criterion};
use wfc_generator::tile::Tile;
use wfc_generator::{PatternSetting, WFC};

pub fn criterion_benchmark(c: &mut Criterion) {
  let mut group = c.benchmark_group("generation");
  group.bench_function("complete", |b| {
    b.iter(|| {
      let mut wfc = WFC::new(
        PatternSetting::PatternBuffer(pattern_test_buffer()),
        50,
        50,
        0
      );
      wfc.setup();
      wfc.generate();
    })
  });


  group.bench_function("preallocation", |b| {
    let mut wfc = WFC::new(
      PatternSetting::PatternBuffer(pattern_test_buffer()),
      50,
      50,
      0
    );
    b.iter(|| {
      wfc.setup();
      wfc.generate();
    })
  });
}

fn pattern_test_buffer() -> Vec<Tile> {
  vec![
    // removed this pattern to get a more steady result
    //Tile::new(
    //  "O".to_string(),
    //  "O".to_string(),
    //  0,
    //  (
    //    "-".to_string(),
    //    "-".to_string(),
    //    "-".to_string(),
    //    "-".to_string()
    //  )
    //),
    Tile::new(
      " ".to_string(),
      " ".to_string(),
      0,
      (
        "air".to_string(),
        "air".to_string(),
        "air".to_string(),
        "air".to_string()
      )
    ),
    Tile::new(
      "└".to_string(),
      "└".to_string(),
      0,
      (
        "air".to_string(),
        "rock".to_string(),
        "rock".to_string(),
        "air".to_string()
      )
    ),
    Tile::new(
      "┌".to_string(),
      "┌".to_string(),
      1,
      (
        "air".to_string(),
        "air".to_string(),
        "rock".to_string(),
        "rock".to_string()
      )
    ),
    Tile::new(
      "┐".to_string(),
      "┐".to_string(),
      2,
      (
        "rock".to_string(),
        "air".to_string(),
        "air".to_string(),
        "rock".to_string()
      )
    ),
    Tile::new(
      "┘".to_string(),
      "┘".to_string(),
      3,
      (
        "rock".to_string(),
        "rock".to_string(),
        "air".to_string(),
        "air".to_string()
      )
    ),
    Tile::new(
      "┼".to_string(),
      "┼".to_string(),
      0,
      (
        "rock".to_string(),
        "rock".to_string(),
        "rock".to_string(),
        "rock".to_string()
      )
    ),
    Tile::new(
      "─".to_string(),
      "─".to_string(),
      0,
      (
        "rock".to_string(),
        "air".to_string(),
        "rock".to_string(),
        "air".to_string()
      )
    ),
    Tile::new(
      "│".to_string(),
      "│".to_string(),
      0,
      (
        "air".to_string(),
        "rock".to_string(),
        "air".to_string(),
        "rock".to_string()
      )
    ),
  ]
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
