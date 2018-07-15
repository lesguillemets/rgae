extern crate image;
extern crate imageproc;
use std::fs::File;
use std::path::Path;

use image::GenericImage;
use imageproc::drawing::{draw_filled_rect_mut, draw_line_segment_mut};
use imageproc::rect::Rect;

fn main() {
    let imgx = 400;
    let imgy = 400;
    let mut img_buf = image::ImageBuffer::new(imgx, imgy);

    for (x, y, pixel) in img_buf.enumerate_pixels_mut() {
        *pixel = image::Rgb([(x * 256 / imgx) as u8, (y * 256 / imgy) as u8, 3]);
    }
    draw_line_segment_mut(
        &mut img_buf,
        (10.0, 10.0),
        (50.0, 50.0),
        image::Rgb([0, 0, 245]),
    );
    draw_filled_rect_mut(
        &mut img_buf,
        Rect::at(60, 60).of_size(50, 50),
        image::Rgb([10, 20, 50]),
    );
    img_buf.save("example.png").unwrap();
}
