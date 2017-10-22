extern crate image;

use image::{GenericImage, ImageBuffer};
use std::env;
use std::error::Error;
use std::path::Path;

// each tuple: (x index, y index, hue rotation amount)
const GRID: [(u32, u32, i32); 9] = [
  (0, 0, 180), (1, 0, 340), (2, 0, 260),
  (0, 1, 60),  (1, 1, 20),  (2, 1, 100),
  (0, 2, 300), (1, 2, 220), (2, 2, 140),
];

const CONTRAST_AMOUNT: f32 = 100.;

fn warholify<I, O>(src_path: I, dst_path: O) -> Result<(), Box<Error>>
    where I: AsRef<Path>, O: AsRef<Path>
{
    let img = image::open(src_path)?;
    let (src_width, src_height) = img.dimensions();
    let mut img_buf = ImageBuffer::new(src_width * 3, src_height * 3);
    for &(x, y, hue) in GRID.iter() {
        img_buf.copy_from(
            &img.adjust_contrast(CONTRAST_AMOUNT).huerotate(hue),
            src_width * x,
            src_height * y,
        );
    }
    img_buf.save(dst_path)?;
    Ok(())
}

fn main() {
    let mut args = env::args_os().skip(1);
    let src = args.next().expect("usage: warholify <input path> <output path>");
    let dst = args.next().expect("usage: warholify <input path> <output path>");
    warholify(src, dst).unwrap();
}
