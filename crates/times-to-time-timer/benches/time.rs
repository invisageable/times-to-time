use times_to_time_timer::timer::Timer;
use times_to_time_timer::unit::Unit;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn criterion_benchmark(c: &mut Criterion) {
  let mut timer = Timer::new();

  timer.start();
  timer.sleep(100);
  timer.end();

  c.bench_function("duration", |b| {
    b.iter(|| black_box(timer.duration()));
  });

  c.bench_function("duration_in_unit", |b| {
    b.iter(|| timer.duration_in_unit(black_box(Unit::S)));
  });
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
