use criterion::{criterion_group, criterion_main, Criterion};
use wfc_generator::tile::Tile;
use wfc_generator::tileloader::{TestLoader, TileLoader};
use wfc_generator::{PatternSetting, WFC};

pub fn criterion_benchmark(c: &mut Criterion) {
  let mut group = c.benchmark_group("generation");

  

  group.bench_function("complete", |b| {
    b.iter(|| {
      let mut wfc = WFC::new(
        TestLoader::new().load(),
        50,
        50,
        0
      );
      wfc.generate();
    })
  });


  group.bench_function("preallocation", |b| {
    let mut wfc = WFC::new(
      TestLoader::new().load(),
      50,
      50,
      0
    );
    b.iter(|| {
      wfc.generate();
    })
  });

  let mut wfc = WFC::new(
    TestLoader::new().load(),
    50,
    50,
    0
  );
  group.bench_function("generate", |b| {
    b.iter(|| {
      wfc.generate();
    })
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
