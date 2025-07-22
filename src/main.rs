use std::fs::File;
use std::io::{self, Read, Write};
use std::path::Path;

use clap::Parser;
use flate2::Compression;
use flate2::write::GzEncoder;
use walkdir::WalkDir;

#[derive(Parser, Debug)]
#[command(version, about = "Toolkit for uploading compressed file/folder.", long_about = None)]
struct Args {
    /// Selected file or folder path
    #[arg(short, long)]
    path: String,
}

fn compress_file(input_path: &str, output_path: &str) -> io::Result<()> {
    let mut input = File::open(input_path)?;
    let output = File::create(output_path)?;
    let mut encoder = GzEncoder::new(output, Compression::default());
    let mut buffer = Vec::new();
    input.read_to_end(&mut buffer)?;
    encoder.write_all(&buffer)?;
    Ok(())
}

fn compress_folder(folder_path: &str, output_dir: &str) -> io::Result<()> {
    for entry in WalkDir::new(folder_path).into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        if path.is_file() {
            let file_name = path.file_name().unwrap().to_string_lossy();
            let output_path = format!("{}/{}.gz", output_dir, file_name);
            compress_file(path.to_str().unwrap(), &output_path)?;
        }
    }
    Ok(())
}

fn main() {
    let args = Args::parse();
    let input_path = &args.path;
    let output_path = format!("{}.gz", input_path);

    if Path::new(input_path).is_file() {
        compress_file(input_path, &output_path).expect("Failed to compress file");
        println!("Compressed file saved to: {}", output_path);
    } else if Path::new(input_path).is_dir() {
        let output_dir = format!("{}.gz", input_path);
        std::fs::create_dir_all(&output_dir).expect("Failed to create output directory");
        compress_folder(input_path, &output_dir).expect("Failed to compress folder");
        println!("Compressed folder saved to: {}", output_dir);
    } else {
        println!("Invalid path: {}", input_path);
    }
}
