use std::io;
use std::fs;
use std::process;
use std::env;
use image::{GenericImageView, ImageFormat};

mod format;
mod encode;
mod protocol;
mod gif_reader;

const CHUNK_SIZE: usize = 4096;

fn main() {
    // returns a vector with the binarys path and the args path
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

    
    // checking file format and loading the payload and meta
    // kitty supports PNG as an encoded image image format
    // whereas other formats need to be decoded into raw pixel data before trasmitting
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
            (raw_pixels, 24u8, Some(w),Some(h))
        }
        ImageFormat::Gif => {
            let frames = match gif_reader::decode_gif(path) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Failed to decode GIF '{}':{}",path,e);
                    process::exit(1);
                }
            };

            println!("Decoded {} frames", frames.len());
            for (i,f) in frames.iter().enumerate(){
                println!("Frame {}: {}x{}, delay {}ms",i,f.buffer.width(),f.buffer.height(),f.delay);
            }
            process::exit(1);
        }
        _ => {
            eprintln!("Unsupported file format: {}",path);
            eprintln!("If possible, contribute @github.com/superbryn/roop");
            process::exit(1);
        }
    };
    
    let chucks = encode::encode_and_chuck(&payload, CHUNK_SIZE);

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    protocol::write_sequence(&mut handle, &chucks, format_code, width, height);
}