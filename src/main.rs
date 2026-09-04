use std::io;
use std::fs;
use std::process;
use std::env;
use image::{GenericImageView, ImageFormat};

mod format;
mod encode;
mod protocol;

const CHUNK_SIZE: usize = 4096;
const FORMAT_CODE_RGBA: u8 = 32; // for images with alpha channel
const FORMAT_CODE_RGB: u8 = 24;  // for images without alpha channel 

fn main() {
    // collects the binarys path and command line arguments (here, path to image) into a vector
    let args: Vec<String> = env::args().collect();

    if args.len() < 2{
        println!("Usage: roop path/to/image");
        process::exit(1);
    }

    let path = &args[1];

    let format_detected = match format::detect_format(path) {
        Some(fmt) => fmt,
        None => {
            eprintln!("Could not detect the image format of {}",path);
            process::exit(1);
        }
    };

    // checks the file format and loads the image payload
    // KGP supports PNG as an encoded image format
    // but other formats must be decoded into raw pixel data before trasmission
    let (payload, format_code, width, height) = match format_detected {
        ImageFormat::Png => {
            let raw_bytes = match fs::read(path) {
                Ok(bytes) => bytes,
                Err(e) => {
                    eprintln!("Failed to read {}: {}",path,e);
                    process::exit(1);
                }
            };
            (raw_bytes, 100u8, None,None)
        }
        ImageFormat::Jpeg => {
            let img = match image::open(path) {
                Ok(img) => img,
                Err(e) => {
                    eprintln!("Failed to read {}: {}",path,e);
                    process::exit(1);
                }
            };
            let (w, h) = img.dimensions();
            let raw_pixels = img.to_rgb8().into_raw();
            (raw_pixels, FORMAT_CODE_RGB, Some(w),Some(h))
        }
        
        // renderers every image format supported by image crate
        _ => {
            let img = image::open(path).unwrap_or_else(|e| {
                eprintln!("Failed to read or decode {}: {}",path,e);
                process::exit(1);
            });
            
            let (w,h) = img.dimensions();
            let raw_pixels = img.into_rgba8().into_raw();
            (raw_pixels, FORMAT_CODE_RGBA, Some(w),Some( h))
        }
    };
    
    let chunks = encode::encode_and_chunk(&payload, CHUNK_SIZE);

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    protocol::write_sequence_image(&mut handle, &chunks, format_code,width,height);
}