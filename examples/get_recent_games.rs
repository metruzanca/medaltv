use medaltv::MedalAPI;
use std::io::{self, Write};

fn format_time(ms: i64) -> String {
    let secs = ms / 1000;
    let datetime = time::OffsetDateTime::from_unix_timestamp(secs).unwrap();
    datetime
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| format!("{ms}"))
}

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

    let games = api.get_recent_games(&username).unwrap_or_else(|e| {
        eprintln!("Failed to fetch games: {e}");
        std::process::exit(1);
    });

    if games.is_empty() {
        println!("No recent gaming sessions.");
        return;
    }

    for (i, game) in games.iter().enumerate() {
        println!("[{i}] {:<30}  session: {}", game.category, game.session_id);
        println!("    Started:  {}", format_time(game.start_time));
        match &game.end_time {
            serde_json::Value::Number(_) => {
                println!("    Ended:    {}", format_time(game.end_time.as_i64().unwrap()));
            }
            _ => println!("    Ended:    (still active)"),
        }
        println!("    Expires:  {}", format_time(game.expires_at));
        println!();
    }
}
