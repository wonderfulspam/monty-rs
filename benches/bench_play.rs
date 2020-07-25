use criterion::{
    criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion, PlotConfiguration,
};
use monty_rs::play_threaded;

fn bench_steps(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("play_steps");
    group.plot_config(plot_config);
    for iters in [1, 10, 100, 1_000, 10_000, 100_000].iter() {
        group.bench_with_input(BenchmarkId::from_parameter(iters), iters, |b, &iters| {
            b.iter(|| play_threaded(iters))
        });
    }
    group.finish();
}

criterion_group!(benches, bench_steps);
criterion_main!(benches);
