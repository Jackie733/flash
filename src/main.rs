use std::path::Path;

use anyhow::Ok;
use anyhow::{Context, Result};
use clap::Parser;
use log::{error, info, warn};

use flash::compress;
use flash::config::Config;
use flash::input;
use flash::upload;

#[derive(Parser, Debug)]
#[command(version, about = "Toolkit for uploading compressed file/folder.", long_about = None)]
struct Args {
    #[arg(short, long, required_unless_present = "init_config")]
    path: Option<String>,
    #[arg(long)]
    ip: Option<String>,
    #[arg(long)]
    port: Option<u16>,
    #[arg(long)]
    username: Option<String>,
    #[arg(long)]
    password: Option<String>,
    #[arg(long)]
    server: Option<String>,
    #[arg(long, action)]
    init_config: bool,
}

fn main() -> Result<()> {
    env_logger::init();

    let args = Args::parse();

    if args.init_config {
        Config::create_example()?;
        return Ok(());
    }

    let config = Config::load().unwrap_or_else(|e| {
        warn!(
            "Failed to load config: {}. Use --init-config to create one.",
            e
        );
        None
    });

    let (ip, username, mut password, port, remote_path_template) =
        if let Some(server_name) = &args.server {
            match &config {
                Some(cfg) => {
                    if let Some(server) = cfg.servers.get(server_name) {
                        info!("Using configured server: {}", server.name);
                        (
                            server.ip.clone(),
                            server.username.clone(),
                            server.password.clone().unwrap_or_default(),
                            server.port.unwrap_or(22),
                            server
                                .remote_path
                                .clone()
                                .unwrap_or_else(|| format!("/home/{}", server.username)),
                        )
                    } else {
                        error!("Server '{}' not found in config.", server_name);
                        return Err(anyhow::anyhow!(
                            "Server '{}' not found in config.",
                            server_name
                        ));
                    }
                }
                None => {
                    error!("No configuration loaded. Use --init-config to create one.");
                    return Err(anyhow::anyhow!("No configuration loaded."));
                }
            }
        } else if let Some(configured_server) = input::get_server_config(config) {
            let username = configured_server.username.clone();
            let remote_path = configured_server
                .remote_path
                .unwrap_or_else(|| format!("/home/{}", username));
            (
                configured_server.ip,
                username,
                configured_server.password.unwrap_or_default(),
                configured_server.port.unwrap_or(22),
                remote_path,
            )
        } else {
            let ip = args
                .ip
                .unwrap_or_else(|| input::prompt_ip_address("Server IP: "));
            let username = args
                .username
                .unwrap_or_else(|| input::prompt_string("Username: "));
            let password = args
                .password
                .unwrap_or_else(|| input::prompt_secret("Password: "));
            let remote_path = format!("/home/{}", username);
            (ip, username, password, 22, remote_path)
        };

    if password.is_empty() {
        password = input::prompt_secret("Password: ");
    }

    let input_path = match args.path {
        Some(path) => path,
        None => {
            error!("Path is required when not initializing config");
            return Err(anyhow::anyhow!("Path is required"));
        }
    };

    let output_path = format!(
        "{}.zip",
        Path::new(&input_path)
            .file_name()
            .unwrap()
            .to_string_lossy()
    );
    let remote_path = format!("{}/{}", remote_path_template, output_path);

    println!("Start compresssing...");
    if Path::new(&input_path).is_file() {
        compress::compress_file_to_zip(&input_path, &output_path)
            .with_context(|| format!("Failed to compress file: {}", input_path))?;
        info!("Compressed file saved to: {}", output_path);
    } else if Path::new(&input_path).is_dir() {
        compress::compress_folder_to_zip(&input_path, &output_path)
            .with_context(|| format!("Failed to compress folder: {}", input_path))?;
        info!("Compressed folder saved to: {}", output_path);
    } else {
        error!("Invalid path: {}", input_path);
        return Err(anyhow::anyhow!("Invalid path: {}", input_path));
    }

    println!("Start uploading...");
    let max_retries = 3;
    if Path::new(&output_path).exists() {
        upload::upload_via_sftp_with_retry(
            &ip,
            port,
            &username,
            &password,
            &output_path,
            &remote_path,
            max_retries,
        )
        .with_context(|| format!("Failed to upload file: {} to {}", output_path, remote_path))?;
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
