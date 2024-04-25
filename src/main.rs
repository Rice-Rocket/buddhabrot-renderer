use std::sync::{Arc, Mutex, MutexGuard};

use buddhabrot::{color::{Color, Float}, images::Image, sample::sample};


const IM_WIDTH: usize = 4096;
const IM_HEIGHT: usize = 4096;
const IM_SIZE: usize = IM_WIDTH * IM_HEIGHT;
const N_ITERATIONS: u32 = 500000;
const SAMPLE_MULT: u32 = 50;
const PROGRESS_UPDATE: usize = IM_WIDTH * 2;

const NORMALIZE: bool = false;
const EXR: bool = true;
const ROTATE: bool = true;
const REFLECT: bool = true;


fn normalize<T: Color + Clone + Copy + Send + Sync + 'static>(im: &mut MutexGuard<Image<T>>) {
    let mut max = T::empty();
    for pixel in im.pixels() {
        max = max.max(*pixel);
    }

    for pixel in im.pixels_mut() {
        pixel.cdiv_assign(max);
    }
}

fn main() {
    let im = Arc::new(Mutex::new(Image::<Float>::new(IM_SIZE, IM_WIDTH)));
    sample(im.clone(), N_ITERATIONS, SAMPLE_MULT, PROGRESS_UPDATE);

    let mut im = im.lock().unwrap();

    if NORMALIZE {
        normalize(&mut im);
    }

    if REFLECT {
        for i in 0..IM_SIZE / 2 {
            let x = i % IM_WIDTH;
            let y = i / IM_WIDTH;
            let c1 = im.get((x, y));
            let c2 = im.get((x, IM_HEIGHT - y - 1));
            im.add((x, IM_HEIGHT - y - 1), c1);
            im.add((x, y), c2);
        }
    }

    if ROTATE {
        for i in 0..IM_SIZE {
            let x = i % IM_WIDTH;
            let y = i / IM_WIDTH;
            if x > y {
                im.swap((x, y), (y, x));
            }
        }
    }

    if EXR {
        exr::image::write::write_rgb_file(
            "output/buddhabrot.exr",
            IM_WIDTH, IM_HEIGHT,
            |x, y| {
                im.get((x, y)).to_tuple()
            }
            ).unwrap();
    } else {
        let mut imgbuf = image::ImageBuffer::new(IM_WIDTH as u32, IM_HEIGHT as u32);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let c = im.get((x as usize, y as usize)).map(|x| x * 255.0);
            let v = c.to_tuple();
            *pixel = image::Rgb([v.0 as u8, v.1 as u8, v.2 as u8]);
        }

        imgbuf.save("output/buddhabrot.png").unwrap();
    }
}
