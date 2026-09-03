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
const FORMAT_CODE_RGBA: u8 = 32;
const FORMAT_CODE_RGB: u8 = 24;

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
        ImageFormat::Gif => {
            let frames = match gif_reader::decode_gif(path) {
                Ok(f) => f,
                Err(e) => {
                    eprintln!("Failed to decode GIF '{}':{}",path,e);
                    process::exit(1);
                }
            };

            if frames.is_empty() {
                eprintln!("This GIF has no frames");
                process::exit(1);
            }

            let image_id: u32 = std::process::id();

            let stdout = io::stdout();
            let mut handle = stdout.lock();

            let first = &frames[0];
            let (w,h) = (first.buffer.width(), first.buffer.height());
            let first_payload = first.buffer.clone().into_raw();
            let first_chunk = encode::encode_and_chunk(&first_payload, CHUNK_SIZE);

            protocol::write_animation_first_frame(&mut handle, &first_chunk, FORMAT_CODE_RGBA, w, h, image_id);

            protocol::set_frame_delay(&mut handle, first.delay, image_id);

            for frame in frames.iter().skip(1) {
                let payload = frame.buffer.clone().into_raw();
                let chunks = encode::encode_and_chunk(&payload, CHUNK_SIZE);
                protocol::write_animation(&mut handle, &chunks, FORMAT_CODE_RGBA, w, h, image_id, frame.delay);
            }

            protocol::start_animation(&mut handle, image_id);
            
            process::exit(0);
        }
        
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

    protocol::write_sequence_image(&mut handle, &chunks, format_code, width, height);
}