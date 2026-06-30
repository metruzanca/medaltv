# medaltv

Unofficial API client for [medal.tv](https://medal.tv) in Rust.

Ported from the Python [medal-api](https://github.com/gianklug/medal-api) by Gian Klug.

## Usage

```rust
use medaltv::MedalAPI;

fn main() -> Result<(), medaltv::MedalError> {
    // Create a client (auto-authenticates as guest)
    let api = MedalAPI::new()?;

    // Or use a pre-existing token: MedalAPI::with_token(token)?

    // Fetch a user
    let users = api.get_user("Test")?;
    println!("{users:#?}");

    // Fetch a category
    let category = api.get_category("0")?;
    println!("{category:#?}");

    // Fetch recent games for a user
    let games = api.get_recent_games("Test")?;
    println!("{games:#?}");

    Ok(())
}
```

Add to your `Cargo.toml`:

```toml
[dependencies]
medaltv = "0.1"
```
