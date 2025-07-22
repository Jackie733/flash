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
