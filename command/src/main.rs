use cliclack::{confirm, input, intro, multiselect};

const BANNER: &str = r#"
   |\_    _ _      _
   /o \  | (_) ___| |__   ___  ___ ___   ___  _ __ __ _
 (_. ||  | | |/ __| '_ \ / _ \/ __/ __| / _ \| '__/ _` |
   /__\  | | | (__| | | |  __/\__ \__ \| (_) | | | (_| |
  )___(  |_|_|\___|_| |_|\___||___/___(_)___/|_|  \__, |
                                                   |___/
"#;

const ENV_PATH: &str = "/.env";

fn main() -> std::io::Result<()> {
    intro(BANNER)?;

    let profiles = multiselect(
        "Select which optional services to run:\n   (Use <space> to toggle, <enter> to confirm)",
    )
    .required(false)
    .item(
        "stockfish-play",
        "Stockfish (for playing against the computer)",
        "",
    )
    .item(
        "stockfish-analysis",
        "Stockfish (for requesting computer analysis of games)",
        "",
    )
    .item(
        "external-engine",
        "External Engine (for connecting a local chess engine to the analysis board)",
        "",
    )
    .item(
        "search",
        "Search (for searching games, forum posts, etc)",
        "",
    )
    .item("gifs", "GIFs (for generating animated GIFs of games)", "")
    .item("thumbnails", "Thumbnailer (for resizing images)", "")
    .item("api-docs", "API docs", "")
    .item("pgn-viewer", "PGN Viewer (Standalone)", "")
    .interact()?;

    let setup_database = confirm("Do you want to seed the database with test users, games, etc?")
        .initial_value(true)
        .interact()?;

    let (su_password, password) = if setup_database {
        (
            input("Choose a password for admin users (blank for 'password')")
                .placeholder("password")
                .default_input("password")
                .required(false)
                .interact()?,
            input("Choose a password for regular users (blank for 'password')")
                .placeholder("password")
                .default_input("password")
                .required(false)
                .interact()?,
        )
    } else {
        (String::from(""), String::from(""))
    };

    let env_contents = format!(
        "COMPOSE_PROFILES={}\nSETUP_DB={}\nSU_PASSWORD={}\nPASSWORD={}\n",
        profiles.join(","),
        setup_database,
        su_password,
        password
    );

    match std::fs::metadata(ENV_PATH) {
        Ok(_) => {
            std::fs::write(ENV_PATH, env_contents)?;
        }
        Err(_) => {
            println!(".env contents:\n{}", env_contents);
        }
    }

    Ok(())
}
