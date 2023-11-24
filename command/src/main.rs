use cliclack::{confirm, input, intro, log, multiselect, spinner};
use colored::Colorize;
use serde::{Deserialize, Serialize};
use std::{io::Error, path::Path};
use strum::{EnumIter, EnumString, IntoEnumIterator};

const BANNER: &str = r"
   |\_    _ _      _
   /o \  | (_) ___| |__   ___  ___ ___   ___  _ __ __ _
 (_. ||  | | |/ __| '_ \ / _ \/ __/ __| / _ \| '__/ _` |
   /__\  | | | (__| | | |  __/\__ \__ \| (_) | | | (_| |
  )___(  |_|_|\___|_| |_|\___||___/___(_)___/|_|  \__, |
                                                   |___/
";

#[derive(Debug, Serialize, Deserialize)]
struct Config {
    profiles: Vec<String>,
    setup_database: bool,
    su_password: String,
    password: String,
}

impl Config {
    fn to_env(&self) -> String {
        let mut env = String::new();

        env.push_str(&format!("COMPOSE_PROFILES={}\n", self.profiles.join(",")));
        env.push_str(&format!("SETUP_DATABASE={}\n", self.setup_database));
        env.push_str(&format!("SU_PASSWORD={}\n", self.su_password));
        env.push_str(&format!("PASSWORD={}\n", self.password));

        env
    }
}

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
    #[strum(serialize = "lichess-org/lila")]
    Lila,
    #[strum(serialize = "lichess-org/lila-ws")]
    LilaWs,
    #[strum(serialize = "lichess-org/lila-db-seed")]
    LilaDbSeed,
    #[strum(serialize = "lichess-org/lifat")]
    Lifat,
    #[strum(serialize = "lichess-org/lila-fishnet")]
    LilaFishnet,
    #[strum(serialize = "lichess-org/lila-engine")]
    LilaEngine,
    #[strum(serialize = "lichess-org/lila-search")]
    LilaSearch,
    #[strum(serialize = "lichess-org/lila-gif")]
    LilaGif,
    #[strum(serialize = "lichess-org/api")]
    Api,
    #[strum(serialize = "lichess-org/chessground")]
    Chessground,
    #[strum(serialize = "lichess-org/pgn-viewer")]
    PgnViewer,
    #[strum(serialize = "lichess-org/scalachess")]
    Scalachess,
    #[strum(serialize = "lichess-org/dartchess")]
    Dartchess,
    #[strum(serialize = "lichess-org/berserk")]
    Berserk,
    #[strum(serialize = "cyanfish/bbpPairings")]
    BbpPairings,
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    assert!(args.len() > 1, "Missing command");

    match args[1].as_str() {
        "setup" => setup(),
        "gitpod-welcome" => gitpod_welcome(),
        _ => panic!("Unknown command"),
    }
}

fn setup() -> std::io::Result<()> {
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

    let config = Config {
        profiles: services
            .iter()
            .filter_map(|service| service.compose_profile.clone())
            .map(|profile| profile.to_string())
            .collect(),
        setup_database,
        su_password,
        password,
    };

    // Create a placeholder directory for each of the repos
    // otherwise the directories will be created by Docker
    // when the volumes are mounted and they may be owned by root
    Repository::iter()
        .map(|repo| repo.to_string())
        .for_each(|repo| {
            let folder = Path::new(&repo).file_name().unwrap();
            let clone_path = Path::new("repos").join(folder);
            std::fs::create_dir_all(clone_path).unwrap();
        });

    let default_repos: Vec<String> = vec![
        Repository::Lila.to_string(),
        Repository::LilaWs.to_string(),
        Repository::LilaDbSeed.to_string(),
        Repository::Lifat.to_string(),
    ];
    let optional_repos = services
        .iter()
        .filter_map(|service| service.repositories.clone())
        .flatten()
        .map(|repo| repo.to_string())
        .collect::<Vec<String>>();
    let repos_to_clone = default_repos
        .iter()
        .chain(optional_repos.iter())
        .collect::<Vec<&String>>();

    for repo in repos_to_clone {
        let repo_url = format!("https://github.com/{repo}.git");
        let mut progress = spinner();
        progress.start(&format!("Cloning {repo}"));
        let folder = Path::new(&repo).file_name().unwrap();
        let clone_path = Path::new("repos").join(folder);

        if clone_path.read_dir()?.next().is_some() {
            progress.stop(format!("Clone {repo} ✓"));
            continue;
        }

        let mut cmd = std::process::Command::new("git");
        cmd.arg("clone")
            .arg("--origin")
            .arg("upstream")
            .arg("--depth")
            .arg("1")
            .arg("--recurse-submodules")
            .arg(&repo_url)
            .arg(&clone_path);

        let output = cmd.output().unwrap();
        assert!(
            output.status.success(),
            "Failed to clone {repo} - {output:?}"
        );

        progress.stop(format!("Clone {repo} ✓"));
    }

    std::fs::write(".env", config.to_env())?;
    log::success("Wrote .env")?;

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
        "Image uploads + thumbnails",
        "for blog/coach/streamer images",
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
    .item(
        OptionalService {
            compose_profile: None,
            repositories: vec![Repository::BbpPairings].into(),
        },
        "Swiss Pairings",
        "bbpPairings tool",
    )
    .interact()
}

fn gitpod_welcome() -> std::io::Result<()> {
    println!("{}", "################".green());
    println!(
        "{}",
        "Your Lichess development environment is starting!".green()
    );
    println!(
        "{}",
        "Monitor the progress in the 'lila' container with the command:".green()
    );
    println!("{}", " docker compose logs lila --follow".green().bold());
    println!(
        "{}",
        "For full documentation, see: https://lichess-org.github.io/lila-gitpod/".green()
    );
    Ok(())
}
