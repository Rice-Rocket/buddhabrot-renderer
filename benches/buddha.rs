extern crate buddhabrot;

use buddhabrot::{color::Rgb, images::Image, sample::sample};
use criterion::{criterion_group, criterion_main, Criterion};


const IM_WIDTH: usize = 256;
const IM_HEIGHT: usize = 256;
const IM_SIZE: usize = IM_WIDTH * IM_HEIGHT;

fn bench() {
    let mut im = Image::<Rgb, IM_SIZE, IM_WIDTH>::new();
    sample(&mut im, 10000, 20);
} 

fn criterion_bench(c: &mut Criterion) {
    c.bench_function("buddha sample 1", |b| b.iter(bench));
}

criterion_group!(
    name = benches;
    config = Criterion::default().sample_size(10);
    targets = criterion_bench
);
criterion_main!(benches);
