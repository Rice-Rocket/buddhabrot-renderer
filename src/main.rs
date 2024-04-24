use color::{Color, Rgb};
use images::Image;
use sample::sample;

mod complex;
mod images;
mod color;
mod sample;


const IM_WIDTH: usize = 2048;
const IM_HEIGHT: usize = 2048;
const IM_SIZE: usize = IM_WIDTH * IM_HEIGHT;
const N_ITERATIONS: u32 = 100000;
const SAMPLE_MULT: u32 = 20;

const NORMALIZE: bool = false;
const EXR: bool = true;
const ROTATE: bool = true;
const REFLECT: bool = true;

fn main() {
    let mut im = Image::<Rgb, IM_SIZE, IM_WIDTH>::new();
    sample(&mut im, N_ITERATIONS, SAMPLE_MULT);

    if NORMALIZE {
        let mut max = Rgb::new(0.0, 0.0, 0.0);
        for pixel in im.pixels() {
            max = max.max(*pixel);
        }

        for pixel in im.pixels_mut() {
            pixel.r /= max.r;
            pixel.g /= max.g;
            pixel.b /= max.b;
        }
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
                im.get((x, y)).into()
            }
            ).unwrap();
    } else {
        let mut imgbuf = image::ImageBuffer::new(IM_WIDTH as u32, IM_HEIGHT as u32);

        for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
            let c = im.get((x as usize, y as usize)).map(|x| x * 255.0);
            *pixel = image::Rgb([c.r as u8, c.g as u8, c.b as u8]);
        }

        imgbuf.save("output/buddhabrot.png").unwrap();
    }
}
