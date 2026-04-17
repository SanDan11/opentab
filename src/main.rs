use std::env;
use std::fs;
use std::process;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        eprintln!("Usage: opentab <file.gp5>");
        process::exit(1);
    }

    let path = &args[1];
    println!("OpenTab — Guitar Pro file reader");
    println!("File: {}", path);
    println!();

    let bytes = match fs::read(path) {
        Ok(b) => b,
        Err(e) => {
            eprintln!("Error reading file: {}", e);
            process::exit(1);
        }
    };

    println!("File size: {} bytes", bytes.len());

    // Detect format by extension for now
    if path.ends_with(".gp5") {
        parse_gp5(&bytes);
    } else if path.ends_with(".xml") || path.ends_with(".musicxml") {
        println!("MusicXML support coming soon.");
    } else {
        eprintln!("Unsupported file format. Supported: .gp5");
        process::exit(1);
    }
}

/// Entry point for .gp5 parsing
fn parse_gp5(bytes: &[u8]) {
    println!("Parsing .gp5 file...");
    println!();

    let mut cursor = 0;

    // --- Version string ---
    // GP5 files start with a version string: 1 byte length + string bytes
    match read_version_string(bytes, &mut cursor) {
        Some(version) => println!("Version : {}", version),
        None => {
            eprintln!("Failed to read version string — may not be a valid GP5 file.");
            return;
        }
    }

    // TODO: Parse header block (tempo, key, time signature)
    // TODO: Parse track definitions (instrument, tuning, string count)
    // TODO: Parse measure data (notes, effects, dynamics)
    // TODO: Print track/tab data to terminal

    println!();
    println!("Parser skeleton in place — implementation in progress.");
}

/// Reads the version string from the start of the file.
/// GP format: byte[0] = string length, followed by that many ASCII bytes,
/// padded to 31 bytes total.
fn read_version_string(bytes: &[u8], cursor: &mut usize) -> Option<String> {
    if bytes.len() < 32 {
        return None;
    }

    let len = bytes[0] as usize;
    if len > 31 {
        return None;
    }

    let version_bytes = &bytes[1..=len];
    let version = String::from_utf8_lossy(version_bytes).to_string();

    // GP5 header is always 31 bytes — advance past it
    *cursor = 31;

    Some(version)
}
