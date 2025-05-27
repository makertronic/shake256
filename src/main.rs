/* ------------------------------------------------------------------
// SHAKE256 Hashing Tool
//
// Makertronic c 2025
//
// This tool computes the SHAKE256 hash of a file, which is a
// post-quantum secure hash function. It allows specifying the
// output length of the hash in bytes.
// Usage:
//   cargo run -- --input <file_path> --output_len <length>
// Example:
//   cargo run -- --input example.txt --output_len 64
// ------------------------------------------------------------------
*/

use clap::Parser;
use sha3::{Shake256, digest::{Update, ExtendableOutput}};
use std::fs::File;
use std::io::{BufReader, Read};
use std::path::PathBuf;

/// Hash a file using SHAKE256 (post-quantum secure)
#[derive(Parser, Debug)]
struct Args {
    /// Path to the input file
    #[arg(short, long)]
    input: PathBuf,

    /// Length of the output hash in bytes (default: 64)
    #[arg(short, long, default_value_t = 64)]
    output_len: usize,
}

fn shake256_hash_file(path: &PathBuf, output_len: usize) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
    let mut hasher = Shake256::default();
    let file = File::open(path)?;
    let mut reader = BufReader::new(file);
    let mut buffer = [0u8; 4096];

    while let Ok(n) = reader.read(&mut buffer) {
        if n == 0 { break; }
        hasher.update(&buffer[..n]);
    }

    let mut xof = hasher.finalize_xof();
    let mut result = vec![0u8; output_len];
    std::io::Read::read_exact(&mut xof, &mut result)?;
    Ok(result)
}


fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let hash = shake256_hash_file(&args.input, args.output_len)?;
    println!("SHAKE256 hash ({} bytes): {}", args.output_len, hex::encode(hash));
    Ok(())
}
