use asciify::{
    ColorSpace, Config, Mode, image_to_ascii, load_image, validate_font_path, validate_image_path,
};
use core::TtfFont;
use std::path::PathBuf;
fn main() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let mut config = Config::default();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--mode" | "-m" => {
                let value = next_arg(&mut args, "--mode")?;
                config.mode = match value.as_str() {
                    "blocks" => Mode::Blocks,
                    _ => return Err(format!("Invalid mode: {}", value)),
                }
            }
            "--color" | "-c" => {
                let value = next_arg(&mut args, "--color")?;
                config.color_space = match value.as_str() {
                    "grayscale" => ColorSpace::Grayscale,
                    _ => return Err(format!("Invalid colorspace: {}", value)),
                }
            }
            "--image" | "-i" => {
                let value = next_arg(&mut args, "--image")?;
                config.image_path = validate_image_path(&value)?;
            }
            "--font" | "-f" => {
                let value = next_arg(&mut args, "font")?;
                config.font_path = validate_font_path(&value)?;
            }
            _ => return Err(format!("Unexpected argument: {}", arg)),
        }
    }
    let img = match load_image(config.image_path.clone()) {
        Ok(img) => img,
        Err(e) => {
            panic!("Image loading error: {}", e.to_string());
        }
    };
    config.font_px = 20;
    let out = image_to_ascii(img, config);
    println!("Output: {}", out);
    Ok(())
}
fn next_arg(args: &mut impl Iterator<Item = String>, name: &str) -> Result<String, String> {
    args.next().ok_or(format!("Expected value after {}", name))
}

pub fn load_font(path: PathBuf) -> Result<TtfFont, core::error::Error> {
    TtfFont::new(path)
}
