use std::io;
use std::process;
use std::env;
use image::{GenericImageView, ImageFormat};
use ureq;

mod encode;
mod protocol;

const CHUNK_SIZE: usize = 4096;
const FORMAT_CODE_RGBA: u8 = 32; // for images with alpha channel
const FORMAT_CODE_RGB: u8 = 24;  // for images without alpha channel 

fn load_bytes(source: &str) -> Result<Vec<u8>, String> {
    if source.starts_with("https://") || source.starts_with("http://") {
        let mut response = ureq::get(source).call().map_err(|e| format!("HTTP request failed: {}", e))?;
        
        response.body_mut().with_config().limit(50 * 1024 * 1024).read_to_vec().map_err(|e| format!("Failed to read response stream: {}",e))
    } else {
        std::fs::read(source).map_err(|e| e.to_string())
    }
}

fn detect_format_from_byte(bytes: &[u8]) -> Option<ImageFormat> {
    image::guess_format(bytes).ok()
}

fn main() {
    // collects the binarys path and command line arguments (here, path to image) into a vector
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 || args[1] == "--version" {
        println!("roop version 0.0.5 (Apple)");
        process::exit(1);
    }

    if args.len() < 2 || args[1] == "--help" {
        println!("Usage: roop path/to/image or roop <url>");
        process::exit(1);
    }

    let source = &args[1];

    let raw_bytes = match load_bytes(source) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Failed To Load {}: {}", source, e);
            process::exit(1);
        }
    };

    let detected_format = match detect_format_from_byte(&raw_bytes) {
        Some(fmt) => fmt,
        None => {
            eprintln!("Could not detect the image format of {}",source);
            process::exit(1);
        }
    };

    // checks the file format and loads the image payload
    // KGP supports PNG as an encoded image format
    // but other formats must be decoded into raw pixel data before trasmission
    let (payload, format_code, width, height) = match detected_format {
        ImageFormat::Png => {
            (raw_bytes, 100u8, None,None)
        }
        
        ImageFormat::Jpeg => {
            let img = image::load_from_memory_with_format(&raw_bytes, ImageFormat::Jpeg).unwrap_or_else(|e| {
                eprintln!("Failed To Decode {}: {}",source,e);
                process::exit(1);
            });
            let (w, h) = img.dimensions();
            let raw_pixels = img.to_rgb8().into_raw();
            (raw_pixels, FORMAT_CODE_RGB, Some(w),Some(h))
        }
        
        // renderers every image format supported by image crate
        any_format => {
            let img = image::load_from_memory_with_format(&raw_bytes, any_format).unwrap_or_else(|e| {
                eprintln!("Failed to decode {}: {}",source, e);
                process::exit(1);
            });
            
            let (w,h) = img.dimensions();
            let raw_pixels = img.into_rgba8().into_raw();
            (raw_pixels, FORMAT_CODE_RGBA, Some(w),Some(h))
        }
    };
    
    let chunks = encode::encode_and_chunk(&payload, CHUNK_SIZE);

    let stdout = io::stdout();
    let mut handle = stdout.lock();
    
    protocol::write_sequence_image(&mut handle, &chunks, format_code, width, height);
}