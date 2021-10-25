use criterion::{criterion_group, criterion_main, Criterion};
use wfc_generator::tiles::tile::Tile;
use wfc_generator::tiles::tileloader::{TestLoader, TileLoader};
use wfc_generator::{PatternSetting, WFC};

pub fn criterion_benchmark(c: &mut Criterion) {
  //let mut group1 = c.benchmark_group("tileloader");
  //
  //
  //group1.bench_function("complete", |b| {
  //  b.iter(|| {
  //    let mut tileloader = TestLoader::new();
  //    tileloader.load();
  //  })
  //});

  let mut group2 = c.benchmark_group("generation");

  let mut tileloader = TestLoader::new();
  let links = tileloader.load();

  group2.bench_function("complete", |b| {
    b.iter(|| {
      let mut wfc = WFC::new(
        links.clone(),
        50,
        50,
        0
      );
      wfc.generate();
    })
  });

  group2.bench_function("preallocation", |b| {
    let mut wfc = WFC::new(
      links.clone(),
      50,
      50,
      0
    );
    b.iter(|| {
      wfc.generate();
    })
  });

  let mut wfc = WFC::new(
    links.clone(),
    50,
    50,
    0
  );
  group2.bench_function("generate", |b| {
    b.iter(|| {
      wfc.generate();
    })
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
