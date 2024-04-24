extern crate buddhabrot;

use std::sync::{Arc, Mutex};

use buddhabrot::{color::Rgb, images::Image, sample::sample};
use criterion::{criterion_group, criterion_main, Criterion};


const IM_WIDTH: usize = 256;
const IM_HEIGHT: usize = 256;
const IM_SIZE: usize = IM_WIDTH * IM_HEIGHT;
const PROGRESS_UPDATE: usize = IM_WIDTH;

fn bench() {
    let im = Image::<Rgb>::new(IM_SIZE, IM_WIDTH);
    sample(Arc::new(Mutex::new(im)), 10000, 20, PROGRESS_UPDATE);
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
