#![feature(non_ascii_idents)]
extern crate image;
extern crate imageproc;
extern crate num;
extern crate rand;

use num::complex::Complex;
use rand::random;
use rand::Rng;
use std::cmp::max;
use std::f64::consts::PI;
use std::time::SystemTime;

const MAX_ITER: usize = 10000; // how many iterations to do before deciding the series doesn't diverge
const RANDOM_REPEATS: usize = 400000; // how many random points to use for drawing
const IMG_X: usize = 600; // image width in pixels
const IMG_Y: usize = 600; // image height in pixels
const XVIEWWIDTH: f64 = 4.0; // we take -2.0 to 2.0
const YVIEWWIDTH: f64 = 4.0; // we take -2.0 to 2.0
const OUTSIDE: f64 = 4.0; // we take r^2 > 4.0 as outside, using in the random point generation and judging the convergence

fn main() {
    let mut dat: Vec<u32> = vec![0; (IMG_X * IMG_Y) as usize];
    let mut img_buf = image::ImageBuffer::new(IMG_X as u32, IMG_Y as u32);
    let mut rng = rand::thread_rng();
    for i in 0..RANDOM_REPEATS {
        // println!("{} th gen", i);
        let c: Complex<f64> = random_point(&mut rng);
        if let Some(zs) = generate_zs(c) {
            for z in &zs {
                add_point(&mut dat, z);
                add_point(&mut dat, &z.conj());
            }
        }
    }
    let maximum = dat.iter().fold(0, |m, &v| max(m, v));
    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let val = (255 * (dat[x as usize + y as usize * IMG_X]) / maximum) as u8;
        *pixel = image::Rgb([0, val, val]);
    }
    let now = SystemTime::now();
    img_buf
        .save(format!(
            "brot-{}-maxi{}-rr{}-max{}.png",
            now.duration_since(SystemTime::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            MAX_ITER,
            RANDOM_REPEATS,
            maximum
        ))
        .unwrap();
}

fn add_point(dat: &mut Vec<u32>, z: &Complex<f64>) {
    let x = ((XVIEWWIDTH / 2.0 + z.re) * IMG_X as f64 / XVIEWWIDTH).floor() as usize;
    let y = ((YVIEWWIDTH / 2.0 + z.im) * IMG_Y as f64 / YVIEWWIDTH).floor() as usize;
    // println!("{} {} {}", x, y, x + IMG_X * y);
    if let Some(p) = dat.get_mut(x + IMG_X * y) {
        *p += 1;
    } else {
        panic!("what!?");
    }
}
fn generate_zs(c: Complex<f64>) -> Option<Vec<Complex<f64>>> {
    let mut z = Complex::new(0.0, 0.0);
    let mut results = vec![];
    for _ in 0..MAX_ITER {
        z = z * z + c;
        if z.norm_sqr() > OUTSIDE {
            // diverge
            return Some(results);
        } else {
            results.push(z);
        }
    }
    // didn't diverge
    None
}

fn random_point(g: &mut rand::ThreadRng) -> Complex<f64> {
    /// takes a random point from (r,θ) where 0 <= r <= OUTSIDE and 0 <= θ <= pi.
    let θ: f64 = PI * g.gen::<f64>();
    let r: f64 = OUTSIDE * g.gen::<f64>();
    Complex::new(r * θ.cos(), r * θ.sin())
}
