use medaltv::MedalAPI;
use std::io::{self, Write};

fn main() {
    let mut username = String::new();
    print!("Enter Medal.tv username: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut username).unwrap();
    let username = username.trim();

    if username.is_empty() {
        eprintln!("No username provided.");
        std::process::exit(1);
    }

    let api = MedalAPI::new().unwrap_or_else(|e| {
        eprintln!("Failed to authenticate: {e}");
        std::process::exit(1);
    });

    let users = api.get_user(&username).unwrap_or_else(|e| {
        eprintln!("Failed to fetch user: {e}");
        std::process::exit(1);
    });

    match users.first() {
        Some(user) => println!("{user:#?}"),
        None => println!("User '{username}' not found."),
    }
}
