use rand::{Rng, thread_rng};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{sync::{Arc, Mutex}, thread};

use crate::{color::Rgb, complex::Complex, images::Image};


pub fn sample(im: Arc<Mutex<Image<Rgb>>>, n: u32, m: u32, progress_update: usize) {
    let cpus = num_cpus::get();
    let size = im.lock().unwrap().size;
    let width = im.lock().unwrap().width;
    let iters = size * m as usize;

    let multiprogress = MultiProgress::new();
    let style = ProgressStyle::with_template("{spinner:.green} [{elapsed}] [{bar:50.white/blue}] {pos}/{len} ({eta})").unwrap().progress_chars("=> ").tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏");
    let bar = multiprogress.add(ProgressBar::new(iters as u64).with_style(style));

    let mut threads = Vec::new();

    for _id in 0..cpus {
        let bar = bar.clone();
        let im = im.clone();
        threads.push(thread::spawn(move || {
            let mut rng = thread_rng();
            let mut subim = Image::<Rgb>::new(size, width);

            for i in 0..iters.div_ceil(cpus) {
                let r1 = rng.gen_range(0f32..1f32) * 4.0 - 2.0;
                let r2 = rng.gen_range(0f32..1f32) * 4.0 - 2.0;
                let c = transform(Complex::new(r1, r2));
                let trajectory = mandelbrot(c, n);

                for z in trajectory {
                    let p = transform_inverse(z) * 0.25 + 0.5;
                    let px = Complex::new(p.re * width as f32, p.im * (size / width) as f32).map(|x| x as i32);
                    if !is_inside(width, size, px.into()) {
                        continue;
                    }

                    subim.add(px.map(|x| x as usize).into(), Rgb::new(1.0, 1.0, 1.0));
                }

                if i % progress_update == 0 {
                    bar.inc(progress_update as u64)
                }
            }

            let mut global_im = im.lock().unwrap();
            for (x, y, px) in subim.into_enumerate_pixels() {
                global_im.add((x, y), px);
            }
        }))
    }

    for thread in threads {
        let _ = thread.join();
    }

    multiprogress.clear().unwrap();
}


#[inline]
fn transform(c: Complex<f32>) -> Complex<f32> {
    c
}

#[inline]
fn transform_inverse(c: Complex<f32>) -> Complex<f32> {
    c
}

#[inline]
pub fn is_inside(width: usize, size: usize, px: (i32, i32)) -> bool {
    (px.0 >= 0) && (px.1 >= 0) && (px.0 < width as i32) && (px.1 < (size / width) as i32)
}

fn mandelbrot(c: Complex<f32>, n: u32) -> Vec<Complex<f32>> {
    let mut z = c;
    let mut sequence = Vec::new();
    for _ in 0..n {
        sequence.push(z);
        z = z * z + c;
        if z.abs() > 2.0 {
            return sequence;
        }
    }
    Vec::new()
}
