use anyhow::{ensure, Result};
use image::io::Reader;
use image::{ImageBuffer, Rgb, RgbImage};
use rayon::prelude::*;
use std::path::PathBuf;

pub fn arnold_cat_map_inner(
    image: &RgbImage,
    out_image: &mut RgbImage,
    offset_x: i32,
    offset_y: i32,
) {
    let height = image.height();
    let width = image.width();
    let offset_x = offset_x as i64;
    let offset_y = offset_y as i64;

    let ou = out_image as *mut _ as usize;

    (0..height as i64).into_par_iter().for_each(|x| {
        let oi = unsafe { &mut *(ou as *mut RgbImage) };
        for y in 0..width as i64 {
            // safety: trust me bro
            let (mut _x, mut _y) = (x, y);
            _y = (_y + offset_x * _x).rem_euclid(width as i64);
            _x = (_x + offset_y * _y).rem_euclid(height as i64);
            oi[(_y as u32, _x as u32)] = image[(y as u32, x as u32)];
        }
    });
    // for x in 0..height as i64 {
    //     for y in 0..width as i64 {
    //         let (mut _x, mut _y) = (x, y);
    //         _y = (_y + offset_x * _x).rem_euclid(width as i64);
    //         _x = (_x + offset_y * _y).rem_euclid(height as i64);
    //         out_image[(_y as u32, _x as u32)] = image[(y as u32, x as u32)];
    //     }
    // }
}

pub fn xor_images(a: &RgbImage, b: &RgbImage) -> RgbImage {
    let mut out = ImageBuffer::new(a.width(), a.height());

    for (op, (ap, bp)) in out.pixels_mut().zip(a.pixels().zip(b.pixels())) {
        let [r1, g1, b1] = ap.0;
        let [r2, g2, b2] = bp.0;
        *op = Rgb([r1 ^ r2, g1 ^ g2, b1 ^ b2]);
    }

    out
}

pub fn extract_watermark_inner(
    xorred: &RgbImage,
    out_image: &mut RgbImage,
    offset_x: i32,
    offset_y: i32,
) {
    arnold_cat_map_inner(xorred, out_image, offset_x, offset_y);

    let height = xorred.height();
    let width = xorred.width();
    let ou = out_image as *mut _ as usize;
    (0..height).into_par_iter().for_each(|x| {
        // safety: tee hee
        let oi = unsafe { &mut *(ou as *mut RgbImage) };
        for y in 0..width {
            let px = &mut oi[(y, x)];
            let [mut r, mut g, mut b] = px.0;

            if r > 0 || g > 0 || b > 0 {
                r = 0;
                g = 0;
                b = 0;
            } else {
                r = 255;
                g = 255;
                b = 255;
            }
            *px = Rgb([r, g, b]);
        }
    });
    // for px in out_image.pixels_mut() {
    //     let [mut r, mut g, mut b] = px.0;

    //     if r > 0 || g > 0 || b > 0 {
    //         r = 0;
    //         g = 0;
    //         b = 0;
    //     } else {
    //         r = 255;
    //         g = 255;
    //         b = 255;
    //     }
    //     *px = Rgb([r, g, b]);
    // }
}

pub fn extract_watermark(
    orig_path: PathBuf,
    wmk_path: PathBuf,
    out_path: PathBuf,
    private_key: (i32, i32, i32),
) -> Result<()> {
    let orig = Reader::open(orig_path)?.decode()?.to_rgb8();
    let watermarked = Reader::open(wmk_path)?.decode()?.to_rgb8();

    ensure!(
        orig.height() == watermarked.height(),
        "Image height must be the same"
    );
    ensure!(
        orig.width() == watermarked.width(),
        "Image height must be the same"
    );

    let xorred = xor_images(&orig, &watermarked);
    let mut out_image = ImageBuffer::new(xorred.width(), xorred.height());

    let (offset_x, offset_y, _) = private_key;
    extract_watermark_inner(&xorred, &mut out_image, offset_x, offset_y);

    out_image.save(out_path)?;

    Ok(())
}
