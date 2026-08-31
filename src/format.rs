use std::fs;
use image::ImageFormat;

pub fn detect_format(path: &str) -> Option<ImageFormat> {
    let bytes = fs::read(path).ok()?;
    image::guess_format(&bytes).ok()
}