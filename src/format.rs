use std::fs;

#[derive(Debug, PartialEq)]

// types 
pub enum ImageFormat {
    Png,
    Jpeg,
    Gif,
    Unknown,
}

// Detects the format of the file by analysing the magic bytes, regardless of the files extension
pub fn detect_format(path: &str) -> ImageFormat {
    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(_) => return ImageFormat::Unknown,
    };

    if bytes.starts_with(&[0x89, 0x50, 0x4E, 0x47, 0x0D, 0x0A, 0x1A, 0x0A]){
        ImageFormat::Png
    } else if bytes.starts_with(&[0xFF, 0xD8, 0xFF]) {
        ImageFormat::Jpeg
    } else if bytes.starts_with(b"GIF87a") || bytes.starts_with(b"GIF87b") {
        ImageFormat::Gif
    } else {
        ImageFormat::Unknown
    }
}