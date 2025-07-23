use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::path::Path;
use std::thread;
use std::time::Duration;

use anyhow::Result;
use indicatif::{ProgressBar, ProgressStyle};
use ssh2::Session;

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
