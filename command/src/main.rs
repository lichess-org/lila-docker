use std::io::Error;

use cliclack::{confirm, input, intro, multiselect};
use strum::{EnumIter, EnumString, IntoEnumIterator};

const BANNER: &str = r"
   |\_    _ _      _
   /o \  | (_) ___| |__   ___  ___ ___   ___  _ __ __ _
 (_. ||  | | |/ __| '_ \ / _ \/ __/ __| / _ \| '__/ _` |
   /__\  | | | (__| | | |  __/\__ \__ \| (_) | | | (_| |
  )___(  |_|_|\___|_| |_|\___||___/___(_)___/|_|  \__, |
                                                   |___/
";

const ENV_PATH: &str = "/.env";

#[derive(Default, Clone, Eq, PartialEq, Debug)]
struct OptionalService {
    compose_profile: Option<ComposeProfile>,
    repositories: Option<Vec<Repository>>,
}

#[derive(Debug, Clone, PartialEq, EnumString, strum::Display, Eq, EnumIter)]
#[strum(serialize_all = "kebab-case")]
enum ComposeProfile {
    StockfishPlay,
    StockfishAnalysis,
    ExternalEngine,
    Search,
    Gifs,
    Thumbnails,
    ApiDocs,
    Chessground,
    PgnViewer,
}

#[derive(Debug, Clone, PartialEq, EnumString, strum::Display, Eq, EnumIter)]
#[strum(serialize_all = "kebab-case")]
enum Repository {
    Lila,
    LilaWs,
    LilaDbSeed,
    Lifat,
    LilaFishnet,
    LilaEngine,
    LilaSearch,
    LilaGif,
    Api,
    Chessground,
    PgnViewer,
    Scalachess,
    Dartchess,
    Berserk,
}

fn main() -> std::io::Result<()> {
    intro(BANNER)?;

    let services = prompt_for_optional_services()?;

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
        (String::new(), String::new())
    };

    let env_contents = [
        format!(
            "DIRS={}",
            Repository::iter()
                .map(|repo| repo.to_string())
                .collect::<Vec<_>>()
                .join(",")
        ),
        format!(
            "REPOS={}",
            [
                vec![
                    Repository::Lila,
                    Repository::LilaWs,
                    Repository::LilaDbSeed,
                    Repository::Lifat,
                ],
                services
                    .iter()
                    .filter_map(|service| service.repositories.clone())
                    .flatten()
                    .collect::<Vec<_>>(),
            ]
            .concat()
            .iter()
            .map(std::string::ToString::to_string)
            .collect::<Vec<_>>()
            .join(",")
        ),
        format!(
            "COMPOSE_PROFILES={}",
            services
                .iter()
                .filter_map(|service| service.compose_profile.clone())
                .collect::<Vec<_>>()
                .iter()
                .map(std::string::ToString::to_string)
                .collect::<Vec<_>>()
                .join(",")
        ),
        format!("SETUP_DB={setup_database}"),
        format!("SU_PASSWORD={su_password}"),
        format!("PASSWORD={password}"),
    ]
    .join("\n");

    match std::fs::metadata(ENV_PATH) {
        Ok(_) => std::fs::write(ENV_PATH, env_contents)?,
        Err(_) => println!(".env contents:\n{env_contents}"),
    }

    Ok(())
}

fn prompt_for_optional_services() -> Result<Vec<OptionalService>, Error> {
    multiselect(
        "Select which optional services to include:\n    (Use arrows, <space> to toggle, <enter> to continue)\n",
    )
    .required(false)
    .item(
        OptionalService {
            compose_profile: Some(ComposeProfile::StockfishPlay),
            repositories: vec![Repository::LilaFishnet].into(),
        },
        "Stockfish Play",
        "for playing against the computer",
    )
    .item(
        OptionalService {
            compose_profile: Some(ComposeProfile::StockfishAnalysis),
            repositories: None,
        },
        "Stockfish Analysis",
        "for requesting computer analysis of games",
    )
    .item(
        OptionalService {
            compose_profile: Some(ComposeProfile::ExternalEngine),
            repositories: vec![Repository::LilaEngine].into(),
        },
        "External Engine",
        "for connecting a local chess engine to the analysis board",
    )
    .item(
        OptionalService {
            compose_profile: Some(ComposeProfile::Search),
            repositories: vec![Repository::LilaSearch].into(),
        },
        "Search",
        "for searching games, forum posts, etc",
    )
    .item(
        OptionalService {
            compose_profile: Some(ComposeProfile::Gifs),
            repositories: vec![Repository::LilaGif].into(),
        },
        "GIFs",
        "for generating animated GIFs of games",
    )
    .item(
        OptionalService {
            compose_profile: Some(ComposeProfile::Thumbnails),
            repositories: None,
        },
        "Thumbnail generator",
        "for resizing blog/streamer images",
    )
    .item(
        OptionalService {
            compose_profile: Some(ComposeProfile::ApiDocs),
            repositories: vec![Repository::Api].into(),
        },
        "API docs",
        "standalone API documentation",
    )
    .item(
        OptionalService {
            compose_profile: Some(ComposeProfile::Chessground),
            repositories: vec![Repository::Chessground].into(),
        },
        "Chessground",
        "standalone board UI",
    )
    .item(
        OptionalService {
            compose_profile: Some(ComposeProfile::PgnViewer),
            repositories: vec![Repository::PgnViewer].into(),
        },
        "PGN Viewer",
        "standalone PGN viewer",
    )
    .item(
        OptionalService {
            compose_profile: None,
            repositories: vec![Repository::Scalachess].into(),
        },
        "Scalachess",
        "standalone chess logic library",
    )
    .item(
        OptionalService {
            compose_profile: None,
            repositories: vec![Repository::Dartchess].into(),
        },
        "Dartchess",
        "standalone chess library for mobile platforms",
    )
    .item(
        OptionalService {
            compose_profile: None,
            repositories: vec![Repository::Berserk].into(),
        },
        "Berserk",
        "Python API client",
    )
    .interact()
}
