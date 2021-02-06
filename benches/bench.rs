use criterion::{
    criterion_group, criterion_main, AxisScale, BenchmarkId, Criterion, PlotConfiguration,
};
use delaunator::{Point, Triangulation};
use rand::{Rng, SeedableRng};
use rand_xorshift::XorShiftRng;
use std::iter::repeat_with;

const COUNTS: &[usize] = &[100, 1000, 10_000, 100_000];

fn bench(c: &mut Criterion) {
    let mut rng = XorShiftRng::from_seed([0; 16]);

    let all_points: Vec<_> = repeat_with(|| rng.gen::<(f64, f64)>())
        .map(|(x, y)| Point { x, y })
        .take(*COUNTS.last().unwrap())
        .collect();

    let mut group = c.benchmark_group("triangulate");

    for &count in COUNTS {
        group.bench_function(BenchmarkId::from_parameter(count), |b| {
            let points = &all_points[..count];
            b.iter(move || Triangulation::new(points))
        });
    }

    group
        .sample_size(20)
        .plot_config(PlotConfiguration::default().summary_scale(AxisScale::Logarithmic));

    group.finish();
}

criterion_group!(benches, bench);
criterion_main!(benches);
