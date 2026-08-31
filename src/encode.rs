use base64::{engine::general_purpose, Engine as _};

// encodes raw bytes and slice them into chunks of size chunk size
pub fn encode_and_chunk(payload: &[u8], chunk_size: usize) -> Vec<String> {
    let encoded = general_purpose::STANDARD.encode(payload);

    encoded.as_bytes().chunks(chunk_size).map(|c| std::str::from_utf8(c).unwrap().to_string()).collect()
}