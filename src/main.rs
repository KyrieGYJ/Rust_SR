extern crate image;

use std::env;
use std::path::Path;
use std::cmp::min; // 引入 min 函数

use image::{ImageBuffer, Rgb, RgbImage};

fn show_current_directory() {
    let current_dir = env::current_dir().expect("Failed to get current directory");
    println!("Current directory: {:?}", current_dir);
}

fn bilinear_interpolate(src_img: ImageBuffer<Rgb<u8>, Vec<u8>>, dst_h: u32, dst_w: u32) {
    let mut dst_buffer: ImageBuffer<Rgb<u8>, Vec<u8>> = RgbImage::new(dst_w, dst_h);
    let (src_w, src_h) = src_img.dimensions();

    // assert src_img is smaller than dst_img

    for dst_x in 0..dst_w {
        for dst_y in 0..dst_h {
            let src_x: f32 = dst_x as f32 * (src_w as f32 / dst_w as f32);
            let src_y: f32 = dst_y as f32 * (src_h as f32 / dst_h as f32);
            
            let x_1: u32 = src_x.floor() as u32;
            let y_1: u32 = src_y.floor() as u32;
            let x_2: u32 = min(x_1 + 1, src_w - 1);
            let y_2: u32 = min(y_1 + 1, src_h - 1);
            // normalize
            let u: f32 = src_x - x_1 as f32;
            let v: f32 = src_y - y_1 as f32;
            
            let p_11 = src_img.get_pixel(x_1, y_1);
            let p_21 = src_img.get_pixel(x_2, y_1);
            let p_12 = src_img.get_pixel(x_1, y_2);
            let p_22 = src_img.get_pixel(x_2, y_2);

            let p_11_weight = (1.0 - u) * (1.0 - v);
            let p_21_weight = u * (1.0 - v);
            let p_12_weight = (1.0 - u) * v;
            let p_22_weight = u * v;

            // let pixel = src_img.get(src_x, src_y).rgb8(); // [r, g, b]
            dst_buffer[(dst_x, dst_y)] = Rgb([
                (p_11[0] as f32 * p_11_weight + p_12[0] as f32 * p_12_weight + p_21[0] as f32 * p_21_weight + p_22[0] as f32 * p_22_weight) as u8,
                (p_11[1] as f32 * p_11_weight + p_12[1] as f32 * p_12_weight + p_21[1] as f32 * p_21_weight + p_22[1] as f32 * p_22_weight) as u8,
                (p_11[2] as f32 * p_11_weight + p_12[2] as f32 * p_12_weight + p_21[2] as f32 * p_21_weight + p_22[2] as f32 * p_22_weight) as u8,
            ]);
        }
    }

    let _ = dst_buffer.save("new_image.jpg");
}

fn main() -> image::ImageResult<()> {
    // 打开一个图像文件
    let path = Path::new(r".\resource\paojie.webp");
    let img: image::DynamicImage = image::open(path)?;
    let img: ImageBuffer<Rgb<u8>, Vec<u8>> = img.to_rgb8();
    let (width, height) = img.dimensions();

    bilinear_interpolate(img, height * , width * 2);
    Ok(())
}