use medaltv::MedalAPI;
use std::io::{self, Write};

fn main() {
    let mut id = String::new();
    print!("Enter category ID: ");
    io::stdout().flush().unwrap();
    io::stdin().read_line(&mut id).unwrap();
    let id = id.trim();

    if id.is_empty() {
        eprintln!("No category ID provided.");
        std::process::exit(1);
    }

    println!("Authenticating...");
    let api = MedalAPI::new().unwrap_or_else(|e| {
        eprintln!("Failed to authenticate: {e}");
        std::process::exit(1);
    });

    let category = api.get_category(id).unwrap_or_else(|e| {
        eprintln!("Failed to fetch category: {e}");
        std::process::exit(1);
    });

    println!("{category:#?}");
}
