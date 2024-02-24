use times_to_time_timer::timer::Timer;
use times_to_time_timer::unit::Unit;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

use std::thread;
use std::time::Duration;

fn criterion_benchmark(c: &mut Criterion) {
  let mut timer = Timer::new();

  timer.start();
  thread::sleep(Duration::from_millis(100));
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
