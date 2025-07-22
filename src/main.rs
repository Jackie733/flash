use std::fs::File;
use std::io::{self, Read, Write};
use std::net::TcpStream;
use std::path::Path;

use clap::Parser;
use indicatif::{ProgressBar, ProgressStyle};
use ssh2::Session;
use walkdir::WalkDir;
use zip::CompressionMethod;
use zip::write::FileOptions;

#[derive(Parser, Debug)]
#[command(version, about = "Toolkit for uploading compressed file/folder.", long_about = None)]
struct Args {
    /// Selected file or folder path
    #[arg(short, long)]
    path: String,
}

fn compress_file_to_zip(input_path: &str, output_path: &str) -> io::Result<()> {
    let path = Path::new(input_path);
    let file = File::create(output_path)?;
    let mut zip = zip::ZipWriter::new(file);

    let mut buffer = Vec::new();
    let mut f = File::open(path)?;
    f.read_to_end(&mut buffer)?;

    let file_name = path.file_name().unwrap().to_string_lossy();
    zip.start_file(
        file_name,
        FileOptions::default().compression_method(CompressionMethod::Deflated),
    )?;
    zip.write_all(&buffer)?;
    zip.finish()?;

    Ok(())
}

fn compress_folder_to_zip(folder_path: &str, output_zip: &str) -> io::Result<()> {
    let file = File::create(output_zip)?;
    let mut zip = zip::ZipWriter::new(file);

    let walkdir = WalkDir::new(folder_path);
    let folder_path = Path::new(folder_path);

    for entry in walkdir.into_iter().filter_map(|e| e.ok()) {
        let path = entry.path();
        let name = path.strip_prefix(folder_path).unwrap();

        if path.is_file() {
            let mut f = File::open(path)?;
            let mut buffer = Vec::new();
            f.read_to_end(&mut buffer)?;

            zip.start_file(
                name.to_string_lossy(),
                FileOptions::default().compression_method(CompressionMethod::Deflated),
            )?;
            zip.write_all(&buffer)?;
        } else if path.is_dir() {
            let dir_name = format!("{}/", name.to_string_lossy());
            zip.add_directory(
                dir_name,
                FileOptions::default().compression_method(CompressionMethod::Deflated),
            )?;
        }
    }
    zip.finish()?;

    Ok(())
}

fn upload_via_sftp(
    ip: &str,
    port: u16,
    username: &str,
    password: &str,
    local_zip: &str,
    remote_path: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    let tcp = TcpStream::connect(format!("{}:{}", ip, port))?;
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake()?;

    session.userauth_password(username, password)?;
    if !session.authenticated() {
        return Err("Authentication failed".into());
    }

    let sftp = session.sftp()?;

    let mut file = File::open(local_zip)?;
    let file_size = file.metadata()?.len();

    // progress bar
    let pb = ProgressBar::new(file_size);
    pb.set_style(ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})"
    ).unwrap().progress_chars("#>-"));

    let mut remote_file = sftp.create(Path::new(remote_path))?;
    let mut buffer = [0u8; 8192];
    let mut total_written = 0u64;

    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        remote_file.write_all(&buffer[..n])?;
        total_written += n as u64;
        pb.set_position(total_written);
    }

    pb.finish_with_message("Upload complete");
    println!("File uploaded successfully to ({}) {}", ip, remote_path);
    Ok(())
}

fn main() {
    let args = Args::parse();
    let input_path = &args.path;
    let output_path = format!(
        "{}.zip",
        Path::new(input_path).file_name().unwrap().to_string_lossy()
    );

    let ip = "192.168.1.109";
    let port = 22;
    let username = "amax";
    let password = "Haichuang";
    let remote_path = format!("/home/{}/{}", username, output_path);

    if Path::new(input_path).is_file() {
        compress_file_to_zip(input_path, &output_path).expect("Failed to compress file");
        println!("Compressed file saved to: {}", output_path);
    } else if Path::new(input_path).is_dir() {
        compress_folder_to_zip(input_path, &output_path).expect("Failed to compress folder");
        println!("Compressed folder saved to: {}", output_path);
    } else {
        println!("Invalid path: {}", input_path);
    }

    if Path::new(&output_path).exists() {
        upload_via_sftp(ip, port, username, password, &output_path, &remote_path)
            .expect("Failed to upload file");
    } else {
        println!("Local file does not exist: {}", output_path);
    }
}
