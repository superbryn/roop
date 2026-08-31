use std::io;
use std::fs;
use std::process;
use std::env;
use format::ImageFormat;
use image::GenericImageView;

mod format;
mod encode;
mod protocol;

const CHUNK_SIZE: usize = 4096;

fn main() {
    // collects the binarys path and command line arguments (here, path to image) into a vector
    let args: Vec<String> = env::args().collect();

    if args.len() < 2{
        println!("Usage: roop path/to/image");
        process::exit(1);
    }

    let path = &args[1];

    // checks the file format and loads the image payload
    // KGP supports PNG as an encoded image format
    // but other formats must be decoded into raw pixel data before trasmission
    let (payload, format_code, width, height) = match format::detect_format(path) {
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
            eprintln!("GIF Support coming soon guys");
            process::exit(1);
        }
        ImageFormat::Unknown => {
            eprintln!("Unsupported file format: {}",path);
            eprintln!("If possible, contribute @github.com/superbryn/roop");
            process::exit(1);
        }
    };
    
    let chunks = encode::encode_and_chunk(&payload, CHUNK_SIZE);

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    protocol::write_sequence(&mut handle, &chunks, format_code, width, height);
}