use criterion::{
    criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion, PlotConfiguration,
};
use monty_rs::MontyHall;

fn bench_steps(c: &mut Criterion) {
    let plot_config = PlotConfiguration::default().summary_scale(AxisScale::Logarithmic);

    let mut group = c.benchmark_group("play_steps");
    group.plot_config(plot_config);
    for iters in [1_000, 10_000, 100_000].iter() {
        let mut monty = MontyHall::default();
        group.bench_with_input(BenchmarkId::from_parameter(iters), iters, |b, &iters| {
            b.iter(|| monty.play_multiple(iters))
        });
    }
    group.finish();
}

criterion_group!(steps, bench_steps);
criterion_main!(steps);
