extern crate image;
extern crate num;
use num::complex::Complex;
use std::cmp::max;

const MAX_ITER: u32 = 20000;
const IMG_X: usize = 2000; // image width in pixels
const IMG_Y: usize = 2000; // image height in pixels
const XVIEWWIDTH: f64 = 3.0; // we take -1.5 to 1.5
const YVIEWWIDTH: f64 = 3.0; // we take -1.5 to 1.5
const XLEFTMOST: f64 = -XVIEWWIDTH / 2.0;
const YUPPERMOST: f64 = -YVIEWWIDTH / 2.0;
const GRID_WIDTH: f64 = XVIEWWIDTH / (IMG_X as f64);
const GRID_HEIGHT: f64 = YVIEWWIDTH / (IMG_Y as f64);
const OUTSIDE: f64 = 4.0; // we take r^2 > 4.0 as outside in judging the convergence

fn main() {
    let mut dat: Vec<u32> = vec![0; (IMG_X * IMG_Y) as usize];
    for (i, p) in dat.iter_mut().enumerate() {
        if let Some(n) = calc_val(get_loc(i)) {
            *p = n;
        }
    }
    save(&dat);
}

fn get_loc(i: usize) -> Complex<f64> {
    let block_x = f64::from((i % IMG_X) as u32);
    let block_y = f64::from((i / IMG_X) as u32);
    Complex::new(
        XLEFTMOST + GRID_WIDTH * block_x,
        YUPPERMOST + GRID_HEIGHT * block_y,
    )
}

fn to_colour(n: u32, m: u32) -> image::Rgba<u8> {
    if n == 0 {
        image::Rgba([0, 63, 125, 125])
    } else {
        image::Rgba([0, 0, 0, 255])
    }
}

fn calc_val(c: Complex<f64>) -> Option<u32> {
    let mut z = Complex::new(0.0, 0.0);
    for iteration in 0..MAX_ITER {
        z = z * z + c;
        if z.norm_sqr() > OUTSIDE {
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
    let mut img_buf = image::ImageBuffer::new(IMG_X as u32, IMG_Y as u32);
    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        let val = dat[x as usize + y as usize * IMG_X];
        *pixel = to_colour(val, maximum);
    }
    img_buf
}
