use criterion::{
    criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion, PlotConfiguration,
};
use monty_rs::MontyHall;
use rand::rngs::mock::StepRng;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand_xoshiro::SplitMix64;

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

fn bench_rngs(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_rngs");
    let iters = 1_000_000;
    let mut m = MontyHall::default();
    group.bench_function("XorShiftRng", move |b| b.iter(|| m.play_multiple(iters)));

    let rng = StepRng::new(0, 1);
    let mut m = MontyHall::new_with_rng(rng);
    group.bench_function("Mock::StepRng", move |b| b.iter(|| m.play_multiple(iters)));

    let rng = SmallRng::from_entropy();
    let mut m = MontyHall::new_with_rng(rng);
    group.bench_function("SmallRng", move |b| b.iter(|| m.play_multiple(iters)));

    let rng = SplitMix64::seed_from_u64(0);
    let mut m = MontyHall::new_with_rng(rng);
    group.bench_function("SplitMix64", move |b| b.iter(|| m.play_multiple(iters)));
}

criterion_group!(steps, bench_steps);
criterion_group!(rngs, bench_rngs);
criterion_main!(steps, rngs);
