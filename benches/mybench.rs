use criterion::{criterion_group, criterion_main, Criterion};
use wfc_generator::{PatternSetting, Tile, WFC};

pub fn criterion_benchmark(c: &mut Criterion) {
  let mut group = c.benchmark_group("generation");

  let mut a = 1;
  let mut c = 1;

  group.bench_function("complete", |b| {
    b.iter(|| {
      let mut wfc = WFC::new(
        PatternSetting::PatternBuffer(pattern_test_buffer()),
        10,
        10,
        0
      );
      wfc.setup();
      wfc.generate();
    })
  });

  //let mut wfc = WFC::new(
  //  PatternSetting::PatternBuffer(pattern_test_buffer()),
  //  10,
  //  10,
  //  0
  //);
  //group.bench_function("init/gen", |b| {
  //  b.iter(|| {
  //    wfc.setup();
  //    wfc.generate();
  //  })
  //});

  /*
  p = 0;
  q = 0;
  group.bench_function("add (threaded)", |b| {
    b.iter(|| {
      p += 2;
      q -= 3;
      let result = client.send(Ops::AddThreaded(p, q)).unwrap();
      assert_eq!(result, q + p);
    })
  });
   */
}

fn pattern_test_buffer() -> Vec<Tile> {
  vec![
    Tile::new(
      "O".to_string(),
      "O".to_string(),
      0,
      (
        "-".to_string(),
        "-".to_string(),
        "-".to_string(),
        "-".to_string()
      )
    ),
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
