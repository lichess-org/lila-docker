use std::vec;

use cliclack::{intro, multiselect};

const BANNER: &str = r#"
   |\_    _ _      _
   /o \  | (_) ___| |__   ___  ___ ___   ___  _ __ __ _
 (_. ||  | | |/ __| '_ \ / _ \/ __/ __| / _ \| '__/ _` |
   /__\  | | | (__| | | |  __/\__ \__ \| (_) | | | (_| |
  )___(  |_|_|\___|_| |_|\___||___/___(_)___/|_|  \__, |
                                                   |___/
"#;

fn main() -> std::io::Result<()> {
    intro(BANNER)?;

    let profiles = multiselect("Select which services to run")
        .initial_values(vec![""])
        .item("", "Default (lila, lila-ws, mongodb, redis)", "required")
        .item(
            "stockfish",
            "Stockfish (for playing against or analyzing games)",
            "",
        )
        .item("external-engine", "External Engine", "")
        .item("search", "Search (elasticsearch, lila-search)", "")
        .item("images", "Images (for generating gifs and thumbnails)", "")
        .interact()?;

    std::fs::write(
        "../.env",
        format!("COMPOSE_PROFILES={}", profiles.join(",")),
    )?;

    Ok(())
}
