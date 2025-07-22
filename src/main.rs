use std::path::Path;

use clap::Parser;

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

fn main() {
    let args = Args::parse();

    let ip = args
        .ip
        .unwrap_or_else(|| input::prompt_string("Server IP: "));
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

    if Path::new(input_path).is_file() {
        compress::compress_file_to_zip(input_path, &output_path).expect("Failed to compress file");
        println!("Compressed file saved to: {}", output_path);
    } else if Path::new(input_path).is_dir() {
        compress::compress_folder_to_zip(input_path, &output_path)
            .expect("Failed to compress folder");
        println!("Compressed folder saved to: {}", output_path);
    } else {
        println!("Invalid path: {}", input_path);
    }

    if Path::new(&output_path).exists() {
        upload::upload_via_sftp(&ip, port, &username, &password, &output_path, &remote_path)
            .expect("Failed to upload file");
    } else {
        println!("Local file does not exist: {}", output_path);
    }
}
