pub mod arnold;
mod gui;
use gui::CatApp;

use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

// print("Usage: decode.py original_image watermarked_image output_image arnold_dx arnold_dy arnold_rd")
#[derive(Debug, Clone, Eq, PartialEq, Parser)]
enum Args {
    Gui {
        #[arg(long, default_value_t = String::from("planet_orig.png"))]
        orig_img: String,

        #[arg(long, default_value_t = String::from("planet_fixed.png"))]
        watermarked_img: String,
    },
    Cli {
        #[arg(long)]
        orig_img: PathBuf,

        #[arg(long)]
        watermarked_img: PathBuf,

        #[arg(long)]
        output_img: PathBuf,

        #[arg(long = "dx", default_value_t = 1)]
        arnold_dx: i32,

        #[arg(long = "dy", default_value_t = 2)]
        arnold_dy: i32,

        #[arg(long = "rd", default_value_t = 1)]
        arnold_rd: i32,
    },
}

fn main() -> Result<()> {
    let args = dbg!(Args::parse());

    match args {
        Args::Gui {
            orig_img,
            watermarked_img,
        } => {
            let native_options = eframe::NativeOptions::default();
            eframe::run_native(
                "My egui App",
                native_options,
                Box::new(|cc| {
                    Box::new(CatApp::new(cc, orig_img.into(), watermarked_img.into()).unwrap())
                }),
            )
            .unwrap();
        }
        Args::Cli {
            orig_img,
            watermarked_img,
            output_img,
            arnold_dx,
            arnold_dy,
            arnold_rd,
        } => {
            let private_key = (arnold_dx, arnold_dy, arnold_rd);
            arnold::extract_watermark(orig_img, watermarked_img, output_img, private_key)?;
        }
    }
    Ok(())
}
