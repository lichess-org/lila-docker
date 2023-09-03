use std::vec;

use cliclack::{confirm, input, intro, multiselect};

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

    let setup_database = confirm("Do you want to seed the database with test users, games, etc?")
        .initial_value(true)
        .interact()?;

    let (su_password, password) = if setup_database {
        let su_password: String = input("Choose a password for admin users (blank for 'password')")
            .placeholder("password")
            .default_input("password")
            .required(false)
            .interact()?;

        let password: String = input("Choose a password for regular users (blank for 'password')")
            .placeholder("password")
            .default_input("password")
            .required(false)
            .interact()?;

        (su_password, password)
    } else {
        (String::from(""), String::from(""))
    };

    std::fs::write(
        "/.env",
        format!(
            "COMPOSE_PROFILES={}\nSETUP_DB={}\nSU_PASSWORD={}\nPASSWORD={}",
            profiles.join(","),
            setup_database,
            su_password,
            password
        ),
    )?;

    Ok(())
}
