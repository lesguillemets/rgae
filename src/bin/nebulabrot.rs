extern crate image;
extern crate imageproc;
extern crate num;
extern crate rand;

use imageproc::drawing::{draw_filled_rect_mut, draw_line_segment_mut};
use imageproc::rect::Rect;
use num::complex::Complex;
use rand::random;
use std::cmp::max;
const MAX_ITER: usize = 500;

fn main() {
    let imgx: usize = 800;
    let imgy: usize = 800;
    let xviewwidth = 4.0;
    let yviewwidth = 4.0;
    let random_repeats = 4000;
    let mut dat: Vec<u32> = vec![0; ((imgx * imgy) as usize)];
    let mut img_buf = image::ImageBuffer::new(imgx as u32, imgy as u32);
    let mut maximum = 0;
    for i in 0..random_repeats {
        println!("{} th gen", i);
        let c: Complex<f64> = Complex::new(
            xviewwidth * random::<f64>() - xviewwidth / 2.0,
            yviewwidth * random::<f64>() - yviewwidth / 2.0,
        );
        if let Some(zs) = generate_zs(c) {
            for z in &zs {
                let x = ((xviewwidth / 2.0 + z.re) * imgx as f64 / xviewwidth).floor() as usize;
                let y = ((yviewwidth / 2.0 + z.im) * imgy as f64 / yviewwidth).floor() as usize;
                // println!("{} {} {}", x, y, x + imgx * y);
                if let Some(p) = dat.get_mut(x + imgx * y) {
                    *p += 1;
                } else {
                    panic!("what!?");
                }
            }
        }
        maximum = dat.iter().fold(0, |m, &v| max(m, v));
        for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
            let val = (255 * (dat[x as usize + y as usize * imgx]) / maximum) as u8;
            *pixel = image::Rgb([0, val, val]);
        }
    }
    img_buf
        .save(format!(
            "brot-maxi{}-rr{}-max{}.png",
            MAX_ITER, random_repeats, maximum
        ))
        .unwrap();
}

fn generate_zs(c: Complex<f64>) -> Option<Vec<Complex<f64>>> {
    let mut z = Complex::new(0.0, 0.0);
    let mut results = vec![];
    for _ in 0..MAX_ITER {
        z = z * z + c;
        if z.norm_sqr() > 4.0 {
            return Some(results);
        } else {
            results.push(z);
        }
    }
    // verge
    return None;
}
