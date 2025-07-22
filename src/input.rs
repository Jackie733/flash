use rpassword::prompt_password;
use std::io::{self, Write};

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
