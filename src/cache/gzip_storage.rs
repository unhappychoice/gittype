use flate2::{read::GzDecoder, write::GzEncoder, Compression};
use std::fs;
use std::io::{Read, Write};

#[derive(Debug)]
pub struct GzipStorage;

impl GzipStorage {
    pub fn save<T: serde::Serialize>(path: &std::path::Path, data: &T) -> Result<(), String> {
        let binary_data =
            bincode::serialize(data).map_err(|e| format!("Failed to serialize data: {}", e))?;
        let mut encoder = GzEncoder::new(Vec::new(), Compression::default());
        encoder
            .write_all(&binary_data)
            .map_err(|e| format!("Failed to compress data: {}", e))?;
        let compressed_data = encoder
            .finish()
            .map_err(|e| format!("Failed to finish compression: {}", e))?;

        fs::write(path, compressed_data).map_err(|e| format!("Failed to save file: {}", e))
    }

    pub fn load<T: serde::de::DeserializeOwned>(path: &std::path::Path) -> Option<T> {
        let compressed_data = fs::read(path).ok()?;
        let mut decoder = GzDecoder::new(&compressed_data[..]);
        let mut binary_data = Vec::new();

        decoder.read_to_end(&mut binary_data).ok()?;

        bincode::deserialize(&binary_data).ok()
    }
}
