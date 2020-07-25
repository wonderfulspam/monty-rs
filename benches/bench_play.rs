use criterion::{criterion_group, criterion_main, Criterion};
use monty_rs::{play_threaded};

pub fn criterion_benchmark(c: &mut Criterion) {
    c.bench_function("play_multiple", |b| b.iter(|| play_threaded(10_000)));
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
