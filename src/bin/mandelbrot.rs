extern crate image;
extern crate num;
use num::complex::Complex;
use std::cmp::max;
use std::sync::mpsc;
use std::thread;

const MAX_ITER: u32 = 30000;
const IMG_X: usize = 10000; // image width in pixels
const IMG_Y: usize = 10000; // image height in pixels
                            // const XVIEWWIDTH: f64 = 4.2; // we take -2.1 to 2.1
                            // const YVIEWWIDTH: f64 = 4.2; // we take -2.1 to 2.1
const YVIEWWIDTH: f64 = 2.8;
const XVIEWWIDTH: f64 = 2.8;
const XVIEWLEFT: f64 = -2.2;
const XVIEWRIGHT: f64 = XVIEWLEFT + XVIEWWIDTH;
const XLEFTMOST: f64 = -XVIEWWIDTH / 2.0;
const YUPPERMOST: f64 = -YVIEWWIDTH / 2.0;
const GRID_WIDTH: f64 = XVIEWWIDTH / (IMG_X as f64);
const GRID_HEIGHT: f64 = YVIEWWIDTH / (IMG_Y as f64);
const DIVERGE_CRITERIA: f64 = 16.0; // we take r^2 > 16.0 as outside in judging the convergence

const MAX_JOBS: usize = 10;
const VERBOSE: bool = false;

fn main() {
    let mut dat: Vec<u32> = vec![0; (IMG_X * IMG_Y) as usize];
    let (tx, rx) = mpsc::channel();
    for i in 1..MAX_JOBS {
        let tx1 = mpsc::Sender::clone(&tx);
        mk_calc_worker(tx1, i);
    }
    mk_calc_worker(tx, 0);

    for (loc, result) in rx {
        if let Some(n) = result {
            if VERBOSE {
                println!("{}\t{}", n, loc);
            }
            if let Some(p) = dat.get_mut(loc) {
                *p = n;
            }
        }
    }

    save(&dat);
}

fn mk_calc_worker(tx: mpsc::Sender<(usize, Option<u32>)>, modulo: usize) -> () {
    thread::spawn(move || {
        for i in (modulo..IMG_X * IMG_Y).step_by(MAX_JOBS) {
            tx.send((i, calc_val(get_loc(i)))).unwrap();
        }
    });
}

fn get_loc(i: usize) -> Complex<f64> {
    let block_x = f64::from((i % IMG_X) as u32);
    let block_y = f64::from((i / IMG_X) as u32);
    Complex::new(
        XVIEWLEFT + GRID_WIDTH * block_x,
        YUPPERMOST + GRID_HEIGHT * block_y,
    )
}

fn to_colour(n: u32, m: u32) -> image::Rgba<u8> {
    if n == 0 {
        image::Rgba([0, 0, 0, 220])
    } else {
        let v = (255.0 * f64::from(n).ln() / f64::from(m).ln()).floor() as u8;
        image::Rgba([0, v / 2, v, 255])
    }
}

fn to_colour1(n: u32, m: u32) -> image::Rgba<u8> {
    if n == 0 {
        image::Rgba([0, 0, 0, 220])
    } else {
        let v = (255.0 * (f64::from(n).ln() / f64::from(m).ln()))
            .powi(2)
            .floor() as u8;
        image::Rgba([v / 4, v / 2, v, 255])
    }
}

fn calc_val(c: Complex<f64>) -> Option<u32> {
    let mut z = Complex::new(0.0, 0.0);
    for iteration in 0..MAX_ITER {
        z = z * z + c;
        if z.norm_sqr() > DIVERGE_CRITERIA {
            return Some(iteration + 1);
        }
    }
    None
}
fn save(dat: &Vec<u32>) -> () {
    let img_buf = draw_picture(&dat);
    img_buf.save(format!("out.png")).unwrap();
}

fn draw_picture(dat: &Vec<u32>) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    let maximum = dat.iter().fold(0, |m, &v| max(m, v));
    if VERBOSE {
        println!("MAX={}", maximum);
    }
    let mut img_buf = image::ImageBuffer::new(IMG_X as u32, IMG_Y as u32);
    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let val = dat[x as usize + y as usize * IMG_X];
        *pixel = to_colour1(val, maximum);
    }
    img_buf
}
