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

#[derive(Default, Clone, Eq, PartialEq, Debug)]
struct OptionalService {
    compose_profile: Option<&'static str>,
    repositories: Option<Vec<&'static str>>,
}

fn main() -> std::io::Result<()> {
    intro(BANNER)?;

    let services = multiselect(
        "Select which optional services to run:\n   (Use <space> to toggle, <enter> to confirm)",
    )
    .required(false)
    .item(
        OptionalService {
            compose_profile: Some("stockfish-play"),
            repositories: vec!["lila-fishnet"].into(),
        },
        "Stockfish (for playing against the computer)",
        "",
    )
    .item(
        OptionalService {
            compose_profile: Some("stockfish-analysis"),
            repositories: None,
        },
        "Stockfish (for requesting computer analysis of games)",
        "",
    )
    .item(
        OptionalService {
            compose_profile: Some("external-engine"),
            repositories: vec!["lila-engine"].into(),
        },
        "External Engine (for connecting a local chess engine to the analysis board)",
        "",
    )
    .item(
        OptionalService {
            compose_profile: Some("search"),
            repositories: vec!["lila-search"].into(),
        },
        "Search (for searching games, forum posts, etc)",
        "",
    )
    .item(
        OptionalService {
            compose_profile: Some("gifs"),
            repositories: vec!["lila-gif"].into(),
        },
        "GIFs (for generating animated GIFs of games)",
        "",
    )
    .item(
        OptionalService {
            compose_profile: Some("thumbnails"),
            repositories: None
        },
        "Thumbnailer (for resizing images)",
        "",
    )
    .item(
        OptionalService {
            compose_profile: Some("api-docs"),
            repositories: vec!["api"].into(),
        },
        "API docs",
        "",
    )
    .item(
        OptionalService {
            compose_profile: Some("chessground"),
            repositories: vec!["chessground"].into(),
        },
        "Chessground board UI (Standalone)",
        "",
    )
    .item(
        OptionalService {
            compose_profile: Some("pgn-viewer"),
            repositories: vec!["pgn-viewer"].into(),
        },
        "PGN Viewer (Standalone)",
        "",
    )
    .item(
        OptionalService {
            compose_profile: None,
            repositories: vec!["scalachess"].into(),
        },
        "Scalachess library",
        "",
    )
    .item(
        OptionalService {
            compose_profile: None,
            repositories: vec!["berserk"].into(),
        },
        "Berserk (Python API client)",
        "",
    )
    .interact()?;

    let setup_database =
        confirm("Do you want to seed the database with test users, games, etc? (Recommended)")
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

    let repos = [
        vec!["lila", "lila-ws", "lila-db-seed", "lifat"],
        services
            .iter()
            .flat_map(|service| service.repositories.clone())
            .flatten()
            .collect::<Vec<_>>(),
    ]
    .concat();

    let profiles = services
        .iter()
        .flat_map(|service| service.compose_profile.clone())
        .collect::<Vec<_>>();

    let env_contents = format!(
        "REPOS={}\nCOMPOSE_PROFILES={}\nSETUP_DB={}\nSU_PASSWORD={}\nPASSWORD={}\n",
        repos.join(","),
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
