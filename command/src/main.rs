use cliclack::{confirm, input, intro, log, multiselect, spinner};
use colored::Colorize;
use std::{io::Error, path::Path};

const BANNER: &str = r"
   |\_    _ _      _
   /o \  | (_) ___| |__   ___  ___ ___   ___  _ __ __ _
 (_. ||  | | |/ __| '_ \ / _ \/ __/ __| / _ \| '__/ _` |
   /__\  | | | (__| | | |  __/\__ \__ \| (_) | | | (_| |
  )___(  |_|_|\___|_| |_|\___||___/___(_)___/|_|  \__, |
                                                   |___/
";

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
struct OptionalService<'a> {
    compose_profile: Option<Vec<&'a str>>,
    repositories: Option<Vec<&'a str>>,
}

const REPOSITORIES: [&str; 15] = [
    "lichess-org/lila",
    "lichess-org/lila-ws",
    "lichess-org/lila-db-seed",
    "lichess-org/lifat",
    "lichess-org/lila-fishnet",
    "lichess-org/lila-engine",
    "lichess-org/lila-search",
    "lichess-org/lila-gif",
    "lichess-org/api",
    "lichess-org/chessground",
    "lichess-org/pgn-viewer",
    "lichess-org/scalachess",
    "lichess-org/dartchess",
    "lichess-org/berserk",
    "cyanfish/bbpPairings",
];

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    assert!(args.len() > 1, "Missing command");

    match args[1].as_str() {
        "setup" => setup(),
        "gitpod-welcome" => {
            gitpod_welcome();
            Ok(())
        }
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
            .flatten()
            .map(std::string::ToString::to_string)
            .collect(),
        setup_database,
        su_password,
        password,
    };

    // Create a placeholder directory for each of the repos
    // otherwise the directories will be created by Docker
    // when the volumes are mounted and they may be owned by root
    for repo in &REPOSITORIES {
        let folder = Path::new(&repo).file_name().unwrap();
        let clone_path = Path::new("repos").join(folder);
        std::fs::create_dir_all(clone_path)?;
    }

    let mut repos_to_clone: Vec<&str> = [
        "lichess-org/lila",
        "lichess-org/lila-ws",
        "lichess-org/lila-db-seed",
        "lichess-org/lifat",
    ]
    .to_vec();

    let optional_repos: Vec<&str> = services
        .iter()
        .filter_map(|service| service.repositories.clone())
        .flatten()
        .collect();

    repos_to_clone.extend(optional_repos);

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
    log::success("Wrote .env")
}

#[allow(clippy::too_many_lines)]
fn prompt_for_optional_services() -> Result<Vec<OptionalService<'static>>, Error> {
    multiselect(
        "Select which optional services to include:\n    (Use arrows, <space> to toggle, <enter> to continue)\n",
    )
    .required(false)
    .item(
        OptionalService {
            compose_profile: vec!["stockfish-play"].into(),
            repositories: vec!["lichess-org/lila-fishnet"].into(),
        },
        "Stockfish Play",
        "for playing against the computer",
    )
    .item(
        OptionalService {
            compose_profile: vec!["stockfish-analysis"].into(),
            repositories: None,
        },
        "Stockfish Analysis",
        "for requesting computer analysis of games",
    )
    .item(
        OptionalService {
            compose_profile: vec!["external-engine"].into(),
            repositories: vec!["lichess-org/lila-engine"].into(),
        },
        "External Engine",
        "for connecting a local chess engine to the analysis board",
    )
    .item(
        OptionalService {
            compose_profile: vec!["search"].into(),
            repositories: vec!["lichess-org/lila-search"].into(),
        },
        "Search",
        "for searching games, forum posts, etc",
    )
    .item(
        OptionalService {
            compose_profile: vec!["gifs"].into(),
            repositories: vec!["lichess-org/lila-gif"].into(),
        },
        "GIFs",
        "for generating animated GIFs of games",
    )
    .item(
        OptionalService {
            compose_profile: vec!["thumbnails"].into(),
            repositories: None,
        },
        "Image uploads + thumbnails",
        "for blog/coach/streamer images",
    )
    .item(
        OptionalService {
            compose_profile: vec!["api-docs"].into(),
            repositories: vec!["lichess-org/api"].into(),
        },
        "API docs",
        "standalone API documentation",
    )
    .item(
        OptionalService {
            compose_profile: vec!["chessground"].into(),
            repositories: vec!["lichess-org/chessground"].into(),
        },
        "Chessground",
        "standalone board UI",
    )
    .item(
        OptionalService {
            compose_profile: vec!["pgn-viewer"].into(),
            repositories: vec!["lichess-org/pgn-viewer"].into(),
        },
        "PGN Viewer",
        "standalone PGN viewer",
    )
    .item(
        OptionalService {
            compose_profile: None,
            repositories: vec!["lichess-org/scalachess"].into(),
        },
        "Scalachess",
        "standalone chess logic library",
    )
    .item(
        OptionalService {
            compose_profile: None,
            repositories: vec!["lichess-org/dartchess"].into(),
        },
        "Dartchess",
        "standalone chess library for mobile platforms",
    )
    .item(
        OptionalService {
            compose_profile: None,
            repositories: vec!["lichess-org/berserk"].into(),
        },
        "Berserk",
        "Python API client",
    )
    .item(
        OptionalService {
            compose_profile: None,
            repositories: vec!["cyanfish/bbpPairings"].into(),
        },
        "Swiss Pairings",
        "bbpPairings tool",
    )
    .interact()
}

fn gitpod_welcome() {
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
}
