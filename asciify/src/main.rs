use core::TtfFont;
use std::path::PathBuf;

use asciify::{ColorSpace, Config, Mode, validate_font_path};
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
            "--font" | "-f" => {
                let value = next_arg(&mut args, "font")?;
                validate_font_path(&value)?;
            }
            _ => return Err(format!("Unexpected argument: {}", arg)),
        }
    }
    Ok(())
}
fn next_arg(args: &mut impl Iterator<Item = String>, name: &str) -> Result<String, String> {
    args.next().ok_or(format!("Expected value after {}", name))
}
pub fn load_font(path: PathBuf) -> TtfFont {
    TtfFont::new(path.to_str().expect("Malformed path")).expect("Malformed font file")
}
