use std::{env,process};

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() > 2{
        println!("Usage: roop path/to/image");
        process::exit(1);
    }

    let path = &args[1];

    let img = match image::open(path) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Failed to Open the file: {}",e);
            process::exit(1);
        }
    };

    println!("Loaded: {}",path);
    println!("Dimensions: {}x{}", img.width(),img.height());
    println!("Color type: {:?}", img.color());
}
