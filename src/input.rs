use crate::config::{Config, ServerConfig};
use rpassword::prompt_password;
use std::{
    io::{self, Write},
    net::IpAddr,
};

pub fn prompt_string(prompt: &str) -> String {
    print!("{}", prompt);
    io::stdout().flush().unwrap();
    let mut value = String::new();
    io::stdin().read_line(&mut value).unwrap();
    value.trim().to_string()
}

pub fn prompt_secret(prompt: &str) -> String {
    prompt_password(prompt).unwrap()
}

pub fn prompt_ip_address(prompt: &str) -> String {
    loop {
        let input = prompt_string(prompt);
        if input.parse::<IpAddr>().is_ok() {
            return input;
        } else {
            println!("Invalid IP address format. Please try again.");
        }
    }
}

pub fn prompt_server_selection(config: &Config) -> Option<ServerConfig> {
    let servers = config.list_servers();

    if servers.is_empty() {
        println!("No servers configured.");
        return None;
    }

    println!("Available servers:");
    for (i, (_, server)) in servers.iter().enumerate() {
        println!(
            " {}. {} ({}@{}:{})",
            i + 1,
            server.name,
            server.username,
            server.ip,
            server.port.unwrap_or(22)
        );
    }
    loop {
        print!(
            "Select a server by number (1-{}) or 'c' for custom: ",
            servers.len()
        );
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();
        let input = input.trim();

        if input.eq_ignore_ascii_case("c") || input.eq_ignore_ascii_case("custom") {
            return None; // user manually input
        }

        if let Ok(choice) = input.parse::<usize>() {
            if choice >= 1 && choice <= servers.len() {
                let (_, server) = &servers[choice - 1];
                return Some((*server).clone());
            }
        }
        print!("Invalid choice. Please try again.\n");
    }
}

pub fn get_server_config(config_opt: Option<Config>) -> Option<ServerConfig> {
    match config_opt {
        Some(config) => {
            if let Some(default_server) = config.get_default_server() {
                println!(
                    "Using default server: {} ({}@{})",
                    default_server.name, default_server.username, default_server.ip
                );

                let use_default = prompt_string("Use this server? (y/n): ");
                if use_default.to_lowercase().starts_with('y') {
                    return Some(default_server.clone());
                }
            }
            prompt_server_selection(&config)
        }
        None => None,
    }
}
