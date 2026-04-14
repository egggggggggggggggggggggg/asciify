use std::path::PathBuf;

use asciify::{ColorSpace, Config, Mode};
fn main() -> Result<(), String> {
    let mut args = std::env::args().skip(1);
    let mut config = Config::default();
    while let Some(arg) = args.next() {
        match arg.as_str() {
            "--mode" | "-m" => {
                let value = next_arg(&mut args, "--mode")?;
                config.mode = match value.as_str() {
                    "dots" => Mode::Dots,
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
                let path = PathBuf::from(value);
                if !path.exists() {
                    return Err(format!("Invalid font file path: {}"));
                }
            }
            _ => return Err(format!("Unexpected argument: {}", arg)),
        }
    }
    Ok(())
}
fn next_arg(args: &mut impl Iterator<Item = String>, name: &str) -> Result<String, String> {
    args.next().ok_or(format!("Expected value after {}", name))
}
