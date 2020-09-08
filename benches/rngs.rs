use criterion::{
    criterion_group, criterion_main, measurement::WallTime, BenchmarkGroup, Criterion,
};
use monty_rs::MontyHall;
use rand::rngs::SmallRng;
use rand::SeedableRng;
use rand::{rngs::mock::StepRng, RngCore};
use rand_xorshift::XorShiftRng;
use rand_xoshiro::SplitMix64;

const ITERS: u64 = 1_000_000;

fn bench_rngs(c: &mut Criterion) {
    let mut group = c.benchmark_group("bench_rngs");

    let rng = XorShiftRng::seed_from_u64(0);
    do_bench(&mut group, rng, "XorShiftRng");

    let rng = StepRng::new(0, 1);
    do_bench(&mut group, rng, "Mock::StepRng");

    let rng = SmallRng::from_entropy();
    do_bench(&mut group, rng, "SmallRng");

    let rng = SplitMix64::seed_from_u64(0);
    do_bench(&mut group, rng, "SplitMix64");
}

fn do_bench<R>(group: &mut BenchmarkGroup<WallTime>, rng: R, name: &str)
where
    R: RngCore,
{
    let mut m = MontyHall::new_with_rng(rng);
    group.bench_function(name, move |b| b.iter(|| m.play_multiple(ITERS)));
}

criterion_group!(rngs, bench_rngs);
criterion_main!(rngs);
