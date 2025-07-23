use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use std::net::TcpStream;
use std::path::Path;
use std::thread;
use std::time::Duration;

use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use sha2::{Digest, Sha256};
use ssh2::{OpenFlags, Session};

fn calc_file_sha256(path: &str) -> anyhow::Result<String> {
    let mut file = File::open(path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];
    loop {
        let n = file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

fn calc_remote_sha256(sftp: &ssh2::Sftp, remote_path: &str) -> anyhow::Result<String> {
    let mut remote_file = sftp.open(remote_path)?;
    let mut hasher = Sha256::new();
    let mut buffer = [0u8; 8192];
    loop {
        let n = remote_file.read(&mut buffer)?;
        if n == 0 {
            break;
        }
        hasher.update(&buffer[..n]);
    }
    Ok(format!("{:x}", hasher.finalize()))
}

pub fn upload_via_sftp(
    ip: &str,
    port: u16,
    username: &str,
    password: &str,
    local_zip: &str,
    remote_path: &str,
) -> Result<()> {
    let tcp = TcpStream::connect(format!("{}:{}", ip, port))?;
    let mut session = Session::new()?;
    session.set_tcp_stream(tcp);
    session.handshake()?;

    session.userauth_password(username, password)?;
    if !session.authenticated() {
        return Err(anyhow::anyhow!("Authentication failed"));
    }

    let sftp = session.sftp()?;
    let remote_file_path = Path::new(remote_path);

    // Ensure remote directory exists
    if let Some(parent_dir) = remote_file_path.parent() {
        if let Err(_) = sftp.stat(parent_dir) {
            println!("Creating remote directory: {:?}", parent_dir);
            sftp.mkdir(parent_dir, 0o755)?;
        }
    }

    let remote_size = match sftp.stat(remote_file_path) {
        Ok(stat) => stat.size.unwrap_or(0),
        Err(_) => 0, // If the file doesn't exist, start from 0
    };

    let mut file = File::open(local_zip)?;
    let file_size = file.metadata()?.len();

    let (start_pos, open_flags) = if remote_size == file_size && file_size > 0 {
        match calc_remote_sha256(&sftp, remote_path) {
            Ok(remote_hash) => {
                let local_hash = calc_file_sha256(local_zip)?;
                if local_hash == remote_hash {
                    println!("File already exists and is identical. Skipping upload.");
                    return Ok(());
                } else {
                    println!("File exists but differs. Overwriting...");
                    (
                        0,
                        OpenFlags::WRITE | OpenFlags::CREATE | OpenFlags::TRUNCATE,
                    )
                }
            }
            Err(_) => {
                println!("Failed to calculate remote file hash, overwriting...");
                (
                    0,
                    OpenFlags::WRITE | OpenFlags::CREATE | OpenFlags::TRUNCATE,
                )
            }
        }
    } else if remote_size > 0 && remote_size < file_size {
        println!("Resuming upload from position: {}", remote_size);
        (remote_size, OpenFlags::WRITE | OpenFlags::APPEND)
    } else {
        println!("Starting fresh upload");
        (
            0,
            OpenFlags::WRITE | OpenFlags::CREATE | OpenFlags::TRUNCATE,
        )
    };

    file.seek(SeekFrom::Start(start_pos))?;

    // progress bar
    let pb = ProgressBar::new(file_size);
    pb.set_position(start_pos);
    pb.set_style(ProgressStyle::default_bar().template(
        "{spinner:.green} [{elapsed_precise}] [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})"
    ).unwrap().progress_chars("#>-"));

    let mut remote_file =
        sftp.open_mode(remote_file_path, open_flags, 0o644, ssh2::OpenType::File)?;
    let mut buffer = [0u8; 8192];
    let mut total_written = start_pos;

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

pub fn upload_via_sftp_with_retry(
    ip: &str,
    port: u16,
    username: &str,
    password: &str,
    local_zip: &str,
    remote_path: &str,
    max_retries: u32,
) -> anyhow::Result<()> {
    let mut last_err = None;
    for attempt in 1..=max_retries {
        match upload_via_sftp(ip, port, username, password, local_zip, remote_path) {
            Ok(()) => return Ok(()),
            Err(e) => {
                eprintln!("Attempt {} failed: {}", attempt, e);
                last_err = Some(e);
                if attempt < max_retries {
                    eprintln!("Retrying after 2 seconds...");
                    thread::sleep(Duration::from_secs(2));
                }
            }
        }
    }
    Err(anyhow::anyhow!(
        "Failed to upload after {} attempts: {}",
        max_retries,
        last_err.unwrap()
    ))
}
