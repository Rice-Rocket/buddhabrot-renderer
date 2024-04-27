use rand::{Rng, thread_rng};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{sync::{Arc, Mutex}, thread};

use crate::{color::{Color, ColorChannel}, complex::Complex, images::Image};


pub fn sample<T: Color + Clone + Copy + Send + Sync + 'static>(im: Arc<Mutex<Image<T>>>, n: u32, m: u32, progress_update: usize) {
    let cpus = num_cpus::get();
    let size = im.lock().unwrap().size;
    let width = im.lock().unwrap().width;
    let iters = size * m as usize;

    let multiprogress = MultiProgress::new();
    let style = ProgressStyle::with_template("{spinner:.green} [{elapsed}] [{bar:50.white/blue}] {pos}/{len} ({eta})").unwrap().progress_chars("=> ").tick_chars("⠋⠙⠹⠸⠼⠴⠦⠧⠇⠏");
    let bar = multiprogress.add(ProgressBar::new(iters as u64).with_style(style));
    bar.inc(0);

    let mut threads = Vec::new();

    for _id in 0..cpus {
        // Increment the Arc's reference count to move into each thread
        let bar = bar.clone();
        let im = im.clone();

        threads.push(thread::spawn(move || {
            let mut rng = thread_rng();
            // Create a new thread-local image to prevent blocking
            let mut subim = Image::<T>::new(size, width);

            for i in 0..iters.div_ceil(cpus) {
                // Generate a random complex number
                let r1 = rng.gen_range(0f32..1f32) * 4.0 - 2.0;
                let r2 = rng.gen_range(0f32..1f32) * 4.0 - 2.0;
                let c = transform(Complex::new(r1, r2));

                // Calculate the path of this complex number over n iterations
                let trajectory = mandelbrot(c, n);

                // Iterate through each point in the complex number's journey
                for z in trajectory {
                    // Convert the complex number to pixel coordinates
                    let p = transform_inverse(z) * 0.25 + 0.5;
                    let px = Complex::new(p.re * width as f32, p.im * (size / width) as f32).map(|x| x as i32);
                    
                    // Ensure the complex number is inside the image
                    if !is_inside(width, size, px.into()) {
                        continue;
                    }

                    // Plot the pixel
                    subim.add(px.map(|x| x as usize).into(), T::one(ColorChannel::Red));
                }

                if i != 0 && i % progress_update == 0 {
                    bar.inc(progress_update as u64)
                }
            }

            // Get a mutable reference to the main image, adding the thread-local iimage to it
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
        // Update z using the Mandelbrot set formula: z = z^2 + c
        z = z * z + c;
        // If z escapes the Mandelbrot set, return the sequence
        if z.abs() > 2.0 { 
            return sequence;
         }
    }
    // If the loop completes without escaping, return an empty vector
    Vec::new()
}
