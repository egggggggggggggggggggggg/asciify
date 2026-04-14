pub mod character_sets;
use std::{fmt::Display, path::PathBuf};
use image::{DynamicImage, ImageError};

#[derive(Default)]
pub struct Config {
    pub mode: Mode,
    pub color_space: ColorSpace,
}
#[derive(Default)]
pub enum ColorSpace {
    Grayscale,
    Ansi,
    Extended,
    #[default]
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
            width, height
        }
    }
    pub fn write_at(&mut self, x: usize, y: usize, char: char) {
        self.chars[self.width * y + x] = char;
    }
    pub fn to_img(format: image::ImageFormat) {
        //Constructs the 
    }
}


///Holds the characters for rendering in a terminal emulator. 
struct ANSIBuffer {

}



const SUPPORTED_IMAGE_EXTENSIONS: &[&'static str] = &[
    "avif", "bmp", "dds", "exr", "ff", "gif", "hdr", "ico", "jpeg", "png", "pnm", "qoi", "tga",
    "tiff", "webp",
];
pub fn validate_image_path(p: impl Into<PathBuf> + Display + Copy) -> Result<PathBuf, String> {
    let path = p.into();
    if !path.exists() {
        return Err(format!("Not a valid image path: {}",p)); 
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
pub fn validate_font_path(p: impl Into<PathBuf> + Display + Copy) -> Result<PathBuf, String>{
    let path = p.into();
    if !path.exists() {
        return Err(format!("Not a valid font path: {}", p));
    } 
    if let Some(ext) = path.extension().and_then(|e| e.to_str()) {
        let ext = ext.to_ascii_lowercase();
        if !SUPPORTED_FONT_EXTENSIONS
    }
}
pub fn load_image(path: PathBuf) -> Result<DynamicImage, ImageError> {
    let img = image::open(path)?;
    Ok(img)
}

fn image_to_ascii(image: DynamicImage, config: Config) {
    let colorspace = image.color_space();
    match config.mode {

    }
}
fn grayscale_ascii(image: ) {

}

