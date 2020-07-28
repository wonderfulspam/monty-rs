use criterion::{
    criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion, PlotConfiguration,
};
use monty_rs::{play_multiple, play_threaded};

fn bench_steps(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("play_steps");
    group.plot_config(plot_config);
    for iters in [100, 1_000, 10_000, 100_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(iters), iters, |b, &iters| {
            b.iter(|| play_threaded(iters))
        });
    }
    group.finish();
}

fn bench_single_core(c: &mut Criterion) {
    c.bench_function("play_unthreaded", |b| b.iter(|| play_multiple(10_000)));
}

criterion_group!(benches, bench_steps, bench_single_core);
criterion_main!(benches);
