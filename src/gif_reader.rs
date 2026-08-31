use image::{AnimationDecoder,RgbaImage};
use std::fs::File;
use image::codecs::gif::GifDecoder;
use std::io::BufReader;

pub struct GifFrame {
    pub buffer: RgbaImage,
    pub delay: u32,
}

pub fn decode_gif(path: &str) -> Result<Vec<GifFrame>, image::ImageError> {
    let file = File::open(path)?;
    let reader = BufReader::new(file);
    let decoder = GifDecoder::new(reader)?;

    let mut frames = Vec::new();

    for frame_result in decoder.into_frames(){
        let frame = frame_result?;

        let (numer, denom) = frame.delay().numer_denom_ms();
        let delay = numer/denom.max(1);

        let buffer = frame.into_buffer();

        frames.push(GifFrame {buffer, delay});
    }
    Ok(frames)
}