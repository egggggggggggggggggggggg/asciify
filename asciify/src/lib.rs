pub mod character_sets;
pub mod render;
use core::{TtfFont, no_atlas, oneshot};
use image::{DynamicImage, ImageBuffer, ImageError, Rgb};
use std::{collections::HashMap, fmt::Display, path::PathBuf};
#[derive(Default)]
pub struct Config {
    pub mode: Mode,
    pub color_space: ColorSpace,
    ///If a font path isn't specified, it'll pick from a list of sensible defaults. This ranges from
    ///system fonts or a shipped font if a reasonable system font cannot be found.   
    pub font_path: PathBuf,
    pub image_path: PathBuf,
    ///If left blank it'll output the image in the current directory the application was executed
    ///in.
    pub output_path: PathBuf,
    pub font_px: u16,
}

#[derive(Default)]
pub enum ColorSpace {
    #[default]
    Grayscale,
    Ansi,
    Extended,
    True,
}

#[derive(Default)]
pub enum Mode {
    #[default]
    Ascii,
    Blocks,
}

///Holds just characters and utilizies flat indexing. x = 0 means top of the screen, y = 0 means
///left of the screen.
struct Char2DArray {
    chars: Vec<char>,
    width: usize,
    height: usize,
}
impl Char2DArray {
    pub fn with_dims(width: usize, height: usize) -> Self {
        Self {
            chars: Vec::with_capacity(width * height),
            width,
            height,
        }
    }

    pub fn write_at(&mut self, x: usize, y: usize, char: char) {
        self.chars[self.width * y + x] = char;
    }
    pub fn to_img(format: image::ImageFormat) {
        //Constructs the
    }
}
pub struct AnsiBuffer {
    chars: Vec<AnsiCell>,
    width: usize,
    height: usize,
}
pub struct AnsiCell {
    bg: Rgb<u8>,
    fg: Rgb<u8>,
    char: char,
}

use image::{GenericImageView, imageops::FilterType};

pub fn image_to_ascii(image: DynamicImage, config: Config) -> String {
    let char_set = match config.mode {
        Mode::Ascii => character_sets::ASCII,
        Mode::Blocks => character_sets::BLOCKS,
    };
    println!("Selected character set: {:?}", char_set);
    let mut font = TtfFont::new(config.font_path).expect("Failed to properly load the font");
    let char_map = no_atlas(char_set, &mut font, config.font_px);
    let density = character_density(char_map);

    // Split densities + chars
    let (densities, chars): (Vec<f32>, Vec<char>) = density.into_iter().unzip();
    // Resize image (important: characters are taller than wide)
    let (width, height) = image.dimensions();
    let aspect_ratio = height as f32 / width as f32;
    let new_width = 300; // configurable later
    let new_height = (new_width as f32 * aspect_ratio) as u32;
    let resized = image.resize(new_width, new_height, FilterType::CatmullRom);
    let grayscale = resized.to_luma8();
    let mut output = String::with_capacity((new_width * new_height) as usize);
    let max_density = densities[0];
    let min_density = densities[densities.len() - 1];
    let range = max_density - min_density;
    for (i, pixel) in grayscale.pixels().enumerate() {
        let brightness = pixel[0] as f32;
        let density_value = 255.0 - brightness;
        let normalized = (density_value - min_density) / range;
        let idx =
            (normalized * (chars.len() - 1) as f32).clamp(0.0, (chars.len() - 1) as f32) as usize;
        output.push(chars[idx]);

        if (i as u32 + 1) % new_width == 0 {
            output.push('\n');
        }
    }
    output
}
fn safe_char(c: char) -> String {
    if c.is_alphanumeric() {
        c.to_string()
    } else {
        format!("u{:04x}", c as u32) // unicode fallback
    }
}
pub fn render_ascii() {}

pub fn character_density(
    char_map: HashMap<char, ImageBuffer<Rgb<u8>, Vec<u8>>>,
) -> Vec<(f32, char)> {
    let mut density_ascii: Vec<(f32, char)> = Vec::new();
    for (ch, image) in char_map {
        let safe = safe_char(ch);
        println!(
            "image size: {} height * {} width for : {}",
            image.height(),
            image.width(),
            ch
        );
        image.save(format!("letters/letter_{}.png", safe)).unwrap();
        let grayscale = DynamicImage::from(image).to_luma8();
        let mut total: u64 = 0;
        let mut count: u64 = 0;
        for pixel in grayscale.pixels() {
            total += pixel[0] as u64; // grayscale value
            count += 1;
        }
        let avg_brightness = total as f32 / count as f32;
        let density = 255.0 - avg_brightness;

        density_ascii.push((density, ch));
    }
    density_ascii.sort_by(|a, b| b.0.partial_cmp(&a.0).unwrap());
    println!("density ascii: {:?}", density_ascii);
    density_ascii
}
///Holds the characters for rendering in a terminal emulator.
struct ANSIBuffer {}

const SUPPORTED_IMAGE_EXTENSIONS: &[&'static str] = &[
    "avif", "bmp", "dds", "exr", "ff", "gif", "hdr", "ico", "jpeg", "png", "pnm", "qoi", "tga",
    "tiff", "webp",
];
pub fn validate_image_path(p: impl Into<PathBuf> + Display + Copy) -> Result<PathBuf, String> {
    let path = p.into();
    if !path.exists() {
        return Err(format!("Not a valid image path: {}", p));
    }
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let ext = ext.to_ascii_lowercase();
        if !SUPPORTED_IMAGE_EXTENSIONS.contains(&ext.as_str()) {
            return Err(format!("Not a supported image extension: {}", ext));
        }
    } else {
        return Err(format!("No specified image extension"));
    }
    Ok(path)
}
const SUPPORTED_FONT_EXTENSIONS: &[&'static str] = &["ttf"];
pub fn validate_font_path(p: impl Into<PathBuf> + Display + Copy) -> Result<PathBuf, String> {
    let path = p.into();
    if !path.exists() {
        return Err(format!("Not a valid font path: {}", p));
    }
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let ext = ext.to_ascii_lowercase();
        if !SUPPORTED_FONT_EXTENSIONS.contains(&ext.as_str()) {
            return Err(format!("Not a supported font extension: {}", ext));
        }
    } else {
        return Err(format!("No specified font extension"));
    }
    Ok(path)
}
pub fn load_image(path: PathBuf) -> Result<DynamicImage, ImageError> {
    let img = image::open(path)?;
    Ok(img)
}
