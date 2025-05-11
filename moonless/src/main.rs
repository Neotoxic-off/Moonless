use std::fs::{File, write};
use std::io::prelude::*;
use std::path::Path;

use clap::Parser;
use logs::{info, error};
use lz_str::{compress_to_base64, decompress_from_base64};

mod arguments;

fn init_logger() {
    env_logger::Builder::from_env(
        env_logger::Env::default().default_filter_or("info")
    ).init();
}

fn encode(input: &str) -> String {
    compress_to_base64(input)
}

fn decode_base64_lz(data: &str) -> Result<String, String> {
    let utf16_data = decompress_from_base64(data)
        .ok_or_else(|| "Decompression failed".to_string())?;

    String::from_utf16(&utf16_data)
        .map_err(|e| format!("UTF-16 conversion failed: {}", e))
}

fn main() -> std::io::Result<()> {
    init_logger();
    let args = arguments::Arguments::parse();

    if args.encode {
        let encoded = encode(&args.savefile);
        info!("Encoded: {}", encoded);
        return Ok(());
    }

    let path = Path::new(&args.savefile);
    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;

    let result = if path.extension().map_or(false, |ext| ext == "rpgsave") {
        decode_base64_lz(&contents)
    } else if path.extension().map_or(false, |ext| ext == "json") {
        decode_base64_lz(&contents)
    } else {
        Err("Unknown file type. Use --encode or provide .rpgsave/.json file.".to_string())
    };

    match result {
        Ok(decoded) => {
            info!("Decoded:\n{}", decoded);
            if let Err(e) = write("decoded.json", decoded) {
                error!("Failed to write to decoded.json: {}", e);
            } else {
                info!("Decoded output saved to decoded.json");
            }
        }
        Err(e) => error!("Error: {}", e),
    }

    Ok(())
}
