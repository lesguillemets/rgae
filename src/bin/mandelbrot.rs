extern crate image;
extern crate num;
use num::complex::Complex;
use std::cmp::max;
use std::sync::mpsc;
use std::thread;

// |z_MAX_ITER|^2 <  DIVERGE_CRITERIA then we decide the sequence doen't diverge
const MAX_ITER: u32 = 10000;
const DIVERGE_CRITERIA: f64 = 16.0;

const IMG_X: usize = 300; // image width in pixels
const IMG_Y: usize = 300; // image height in pixels

const XVIEWWIDTH: f64 = 2.8; // viewport size. See XVIEWLEFT belpow
const YVIEWWIDTH: f64 = 2.8;

const XVIEWLEFT: f64 = -2.2; // XVIEWLEFT <= Re(z) < XVIEWLEFT + XVIEWWIDTH
const YVIEWTOP: f64 = -YVIEWWIDTH / 2.0;
const GRID_WIDTH: f64 = XVIEWWIDTH / (IMG_X as f64); // each pixel corresponds to this width
const GRID_HEIGHT: f64 = YVIEWWIDTH / (IMG_Y as f64);

const MAX_JOBS: usize = 3; // TODO: get the number from command argument
const VERBOSE: bool = false;

fn main() {
    let mut dat: Vec<u32> = vec![0; (IMG_X * IMG_Y) as usize];

    // Spawn MAX_JOBS jobs for calculation
    let (tx, rx) = mpsc::channel();
    for i in 1..MAX_JOBS {
        let tx1 = mpsc::Sender::clone(&tx);
        mk_calc_worker(tx1, i);
    }
    mk_calc_worker(tx, 0); // TODO: this doesn't feel right

    // save results sent from the workers
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

    save(&dat, "frac.png");
}

fn mk_calc_worker(tx: mpsc::Sender<(usize, Option<u32>)>, modulo: usize) -> () {
    thread::spawn(move || {
        for i in (modulo..IMG_X * IMG_Y).step_by(MAX_JOBS) {
            tx.send((i, calc_val(get_loc(i)))).unwrap();
        }
    });
}

// i_th pixel corresponds to this square in C
fn get_loc(i: usize) -> Complex<f64> {
    let block_x = f64::from((i % IMG_X) as u32);
    let block_y = f64::from((i / IMG_X) as u32);
    Complex::new(
        XVIEWLEFT + GRID_WIDTH * block_x,
        YVIEWTOP + GRID_HEIGHT * block_y,
    )
}

// n: iterations it took for the sequence to diverge. m: maximum in the picture
fn to_colour(n: u32, m: u32) -> image::Rgba<u8> {
    if n == 0 {
        image::Rgba([0, 0, 0, 220])
    } else {
        // normalised logarithm
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

// None if the sequence doen't diverge there; Some(n) if it diverges in its n-th iteratation
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
fn save(dat: &Vec<u32>, f_name: &str) -> () {
    let img_buf = draw_picture(&dat);
    img_buf.save(f_name).unwrap();
}

fn draw_picture(dat: &Vec<u32>) -> image::ImageBuffer<image::Rgba<u8>, Vec<u8>> {
    let maximum = dat.iter().fold(0, |m, &v| max(m, v));
    if VERBOSE {
        println!("MAX={}", maximum);
    }
    let mut img_buf = image::ImageBuffer::new(IMG_X as u32, IMG_Y as u32);
    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let val = dat[x as usize + y as usize * IMG_X];
        *pixel = to_colour(val, maximum);
    }
    img_buf
}
