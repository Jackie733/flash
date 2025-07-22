use std::path::Path;

use anyhow::{Context, Result};
use clap::Parser;
use log::{error, info};

mod compress;
mod input;
mod upload;

#[derive(Parser, Debug)]
#[command(version, about = "Toolkit for uploading compressed file/folder.", long_about = None)]
struct Args {
    #[arg(short, long)]
    path: String,
    #[arg(long)]
    ip: Option<String>,
    #[arg(long)]
    username: Option<String>,
    #[arg(long)]
    password: Option<String>,
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    let ip = args
        .ip
        .unwrap_or_else(|| input::prompt_ip_address("Server IP: "));
    let port = 22;
    let username = args
        .username
        .unwrap_or_else(|| input::prompt_string("Username: "));
    let password = args
        .password
        .unwrap_or_else(|| input::prompt_secret("Password: "));

    let input_path = &args.path;
    let output_path = format!(
        "{}.zip",
        Path::new(input_path).file_name().unwrap().to_string_lossy()
    );
    let remote_path = format!("/home/{}/{}", username, output_path);

    println!("Start compresssing...");
    if Path::new(input_path).is_file() {
        compress::compress_file_to_zip(input_path, &output_path)
            .with_context(|| format!("Failed to compress file: {}", input_path))?;
        info!("Compressed file saved to: {}", output_path);
    } else if Path::new(input_path).is_dir() {
        compress::compress_folder_to_zip(input_path, &output_path)
            .with_context(|| format!("Failed to compress folder: {}", input_path))?;
        info!("Compressed folder saved to: {}", output_path);
    } else {
        error!("Invalid path: {}", input_path);
        return Err(anyhow::anyhow!("Invalid path: {}", input_path));
    }

    println!("Start uploading...");
    if Path::new(&output_path).exists() {
        upload::upload_via_sftp(&ip, port, &username, &password, &output_path, &remote_path)
            .with_context(|| {
                format!("Failed to upload file: {} to {}", output_path, remote_path)
            })?;
        info!("File uploaded successfully to ({}) {}", ip, remote_path);
    } else {
        error!("Local file does not exist: {}", output_path);
        return Err(anyhow::anyhow!(
            "Local file does not exist: {}",
            output_path
        ));
    }

    Ok(())
}
