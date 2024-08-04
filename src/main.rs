extern crate image;
mod utils;
mod pipeline;

use std::path::Path;

use image::{ImageBuffer, Rgb};
use utils::bilinear_interpolate;

fn main() -> image::ImageResult<()> {
    // 打开一个图像文件
    let path = Path::new(r".\resource\paojie.webp");
    let img: image::DynamicImage = image::open(path)?;
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = img.to_rgb8();
    let (width, height) = img.dimensions();

    bilinear_interpolate(img, width / 2, height / 2);
    Ok(())
}