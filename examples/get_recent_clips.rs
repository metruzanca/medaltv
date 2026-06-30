use medaltv::{Clip, MedalAPI};
use std::io::{self, Write};

fn format_time(ms: i64) -> String {
    let secs = ms / 1000;
    let datetime = time::OffsetDateTime::from_unix_timestamp(secs).unwrap();
    datetime
        .format(&time::format_description::well_known::Rfc3339)
        .unwrap_or_else(|_| format!("{ms}"))
}

fn format_duration(secs: f64) -> String {
    let m = (secs / 60.0) as u32;
    let s = (secs % 60.0) as u32;
    format!("{m}:{s:02}")
}

fn print_clip(i: usize, clip: &Clip) {
    println!("  [{i}] {:<40} {}", clip.content_title, clip.category);
    println!("      ID:       {}", clip.content_id);
    println!("      Duration: {}", format_duration(clip.video_length_seconds));
    println!("      Views:    {}  |  Likes: {}  |  Comments: {}", clip.views, clip.likes, clip.comments);
    println!("      URL:      {}", clip.content_url_720p);
    println!("      Published:{}", format_time(clip.published_at));
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

    let users = api.get_user(&username).unwrap_or_else(|e| {
        eprintln!("Failed to fetch user: {e}");
        std::process::exit(1);
    });

    let user = match users.first() {
        Some(user) => user,
        None => {
            println!("User '{username}' not found.");
            return;
        }
    };

    println!();
    println!("══════════════════════════════════════");
    println!("  User Profile");
    println!("══════════════════════════════════════");
    println!("  Username:    {}", user.user_name);
    println!("  User ID:     {}", user.user_id);
    println!("  Sessions:    {}", user.game_sessions.len());
    println!("══════════════════════════════════════");
    println!();

    let clips = api.get_recent_clips(&username, 5).unwrap_or_else(|e| {
        eprintln!("Failed to fetch clips: {e}");
        std::process::exit(1);
    });

    if clips.is_empty() {
        println!("No clips found.");
        return;
    }

    println!();
    println!("═════════════════════════════════════════════════════════════════════════════");
    println!("  Recent Clips");
    println!("═════════════════════════════════════════════════════════════════════════════");

    for (i, clip) in clips.iter().enumerate() {
        print_clip(i, clip);
        println!();
    }

    println!("═════════════════════════════════════════════════════════════════════════════");
    println!("  Total: {} clips", clips.len());
    println!("═════════════════════════════════════════════════════════════════════════════");
}
