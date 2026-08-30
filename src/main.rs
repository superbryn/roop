use std::io::{self,Write};
use std::fs;
use std::process;
use std::env;
use base64::{engine::general_purpose,Engine as _};

fn main() {
    // returns a vector with the binarys path and the args path
    let args: Vec<String> = env::args().collect();

    if args.len() > 2{
        println!("Usage: roop path/to/image");
        process::exit(1);
    }

    let path = &args[1];

    // read raw file bytes
    let raw_bytes = match fs::read(path) {
        Ok(bytes) => bytes,
        Err(e) => {
            eprintln!("Failed to read {}: {}",path,e);
            process::exit(1);
        }
    };

    // encode raw bytes to base64
    let encoded = general_purpose::STANDARD.encode(&raw_bytes);

    // chunk into pieces of 4096 bytes
    let c_size = 4096;
    let chucks: Vec<&str> = encoded.as_bytes().chunks(c_size).map(|c| std::str::from_utf8(c).unwrap()).collect();

    let stdout = io::stdout();
    let mut handle = stdout.lock();

    // writing the damn escape sequence
    for (i, chuck) in chucks.iter().enumerate(){
        let is_last = i == chucks.len() - 1;
        let more_flag = if is_last { 0 } else { 1 };

        if i == 0 {
            write!(
                handle,
                "\x1b_Gf=100,a=T,m={};{}\x1b\\",
                more_flag,
                chuck
            ).unwrap();
        } else {
            write!(
                handle,
                "\x1b_Gm={};{}\x1b\\",
                more_flag,
                chuck
            ).unwrap();
        }
    }
}

/*      let img = match image::open(path) {
        Ok(img) => img,
        Err(e) => {
            eprintln!("Failed to Open the file: {}",e);
            process::exit(1);
        }
    };

    println!("Debug: {:?}", args);
    println!("Loaded: {}",path);
    println!("Dimensions: {}x{}", img.width(),img.height());
    println!("Color type: {:?}", img.color());
}

*/