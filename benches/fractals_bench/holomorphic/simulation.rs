use criterion::{black_box, criterion_group, BenchmarkId, Criterion};
use fractals::holomorphic::{simulation::Simulation, test::mandelbrot};
use std::time::Duration;

/// Calculate required messurement time from pessimistic estimated throughput.
/// For fast running benchmarks, we grant at least 1s.
fn expected_messurement_time(elements: u32, n: u64, sample_size: usize, expected_throughput_s: f64) -> Duration {
    let s = ((elements as f64) * (n as f64) * (sample_size as f64) / expected_throughput_s)
                 .max(1.0);
    Duration::from_secs_f64(s)
}

pub fn bench_simulation(c: &mut Criterion) {
    let mut mandelbrot_800_600 = mandelbrot(800, 600);
    let mut mandelbrot_1920_1080 = mandelbrot(1920, 1080);

    let sample_size = 10;
    let expected_throughput_s: f64 = 170e6;  // pixel * iterations / second

    let mut group = c.benchmark_group("simulation");
    group.warm_up_time(Duration::from_millis(100));
    group.sample_size(sample_size);
    group.sampling_mode(criterion::SamplingMode::Flat);
    for iterations in [1, 10, 100, 1000].iter() {
        group.measurement_time(expected_messurement_time(800*600, *iterations, sample_size, expected_throughput_s));
        group.throughput(criterion::Throughput::Elements(800*600*iterations));
        group.bench_with_input(BenchmarkId::new("mandelbrot_800x600", iterations), iterations, |b, &iterations| {
            b.iter(|| mandelbrot_800_600.steps(black_box(iterations as i32)));
        });
    }
    for iterations in [1, 10, 100, 200].iter() {
        group.measurement_time(expected_messurement_time(1920*1080, *iterations, sample_size, expected_throughput_s));
        group.throughput(criterion::Throughput::Elements(1920*1080*iterations));
        group.bench_with_input(BenchmarkId::new("mandelbrot_1920x1080", iterations), iterations, |b, &iterations| {
            b.iter(|| mandelbrot_1920_1080.steps(black_box(iterations as i32)));
        });
    }
    group.finish();
}

criterion_group!(benches_simulation, bench_simulation);
