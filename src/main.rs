use color::{Color, Rgb};
use images::Image;
use sample::sample;

mod complex;
mod images;
mod color;
mod sample;


const IM_WIDTH: usize = 1024;
const IM_HEIGHT: usize = 1024;
const IM_SIZE: usize = IM_WIDTH * IM_HEIGHT;
const N_ITERATIONS: u32 = 100000;
const SAMPLE_MULT: u32 = 20;

const NORMALIZE: bool = false;
const EXR: bool = true;

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
