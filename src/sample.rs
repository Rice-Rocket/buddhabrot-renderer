use rand::{Rng, thread_rng};

use crate::{color::Rgb, complex::Complex, images::Image};

pub fn sample<const D: usize, const W: usize>(im: &mut Image<Rgb, D, W>, n: u32, m: u32) {
    let mut rng = thread_rng();

    for i in 0..D * m as usize {
        let r1 = rng.gen_range(0f32..1f32) * 4.0 - 2.0;
        let r2 = rng.gen_range(0f32..1f32) * 4.0 - 2.0;
        let c = transform(Complex::new(r1, r2));
        let trajectory = mandelbrot(c, n);

        for z in trajectory {
            let p = transform_inverse(z) * 0.25 + 0.5;
            let px = Complex::new(p.re * W as f32, p.im * (D / W) as f32).map(|x| x as i32);
            if !im.is_inside(px.into()) {
                continue;
            }

            im.add(px.map(|x| x as usize).into(), Rgb::new(1.0, 1.0, 1.0));
        }

        print!("\rpixel: {}/{}", i, D * m as usize);
    }
}


fn transform(c: Complex<f32>) -> Complex<f32> {
    c
}

fn transform_inverse(c: Complex<f32>) -> Complex<f32> {
    c
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
