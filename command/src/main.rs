#![warn(clippy::pedantic)]

use cliclack::{
    confirm, input, intro,
    log::{self, info},
    multiselect, note, outro, select, spinner,
};
use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::{
    format,
    io::Error,
    path::{Path, PathBuf},
};

const BANNER: &str = r"
   |\_    _ _      _
   /o \  | (_) ___| |__   ___  ___ ___   ___  _ __ __ _
 (_. ||  | | |/ __| '_ \ / _ \/ __/ __| / _ \| '__/ _` |
   /__\  | | | (__| | | |  __/\__ \__ \| (_) | | | (_| |
  )___(  |_|_|\___|_| |_|\___||___/___(_)___/|_|  \__, |
                                                   |___/
";

#[derive(Serialize, Deserialize, Debug, Default)]
struct Config {
    compose_profiles: Option<Vec<String>>,
    setup_database: Option<bool>,
    setup_bbppairings: Option<bool>,
    enable_monitoring: Option<bool>,
    su_password: Option<String>,
    password: Option<String>,
    lila_domain: Option<String>,
    lila_url: Option<String>,
    phone_ip: Option<String>,
    connection_port: Option<u16>,
    pairing_code: Option<u32>,
    pairing_port: Option<u16>,
}

macro_rules! to_env {
    ($name_opt:ident) => {
        $name_opt
            .clone()
            .map(|v| format!("{}={}", stringify!($name_opt).to_uppercase(), v.to_string()))
            .unwrap_or_default()
    };
    ($key:ident, $value:expr) => {
        format!("{}={}", stringify!($key).to_uppercase(), $value)
    };
}

impl Config {
    const SETTINGS_TOML: &'static str = "settings.toml";
    const SETTINGS_ENV: &'static str = "settings.env";

    fn load() -> Self {
        std::fs::read_to_string(Self::SETTINGS_TOML).map_or_else(
            |_| Self::default(),
            |contents| toml::from_str(&contents).unwrap(),
        )
    }

    fn save(&self) -> std::io::Result<()> {
        std::fs::write(Self::SETTINGS_TOML, self.to_toml())?;
        std::fs::write(Self::SETTINGS_ENV, self.to_env())
    }

    fn to_toml(&self) -> String {
        toml::to_string(&self).unwrap()
    }

    fn to_env(&self) -> String {
        let Self {
            compose_profiles,
            setup_database,
            setup_bbppairings,
            enable_monitoring,
            su_password,
            password,
            lila_domain,
            lila_url,
            phone_ip,
            connection_port,
            pairing_code,
            pairing_port,
        } = self;
        let compose_profiles_string = compose_profiles
            .clone()
            .map(|v| v.join(","))
            .unwrap_or_default();
        format!(
            "{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n{}\n",
            to_env!(compose_profiles, compose_profiles_string),
            to_env!(setup_database),
            to_env!(setup_bbppairings),
            to_env!(enable_monitoring),
            to_env!(su_password),
            to_env!(password),
            to_env!(lila_domain),
            to_env!(lila_url),
            to_env!(phone_ip),
            to_env!(connection_port),
            to_env!(pairing_code),
            to_env!(pairing_port)
        )
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct Repository {
    org: String,
    project: String,
}

impl Repository {
    fn new(org: &str, project: &str) -> Self {
        Self {
            org: org.to_string(),
            project: project.to_string(),
        }
    }

    fn full_name(&self) -> String {
        format!("{}/{}", self.org, self.project)
    }

    fn url(&self) -> String {
        format!("https://github.com/{}/{}", self.org, self.project)
    }

    fn clone_path(&self) -> PathBuf {
        Path::new("repos").join(&self.project)
    }
}

struct Gitpod {
    domain: String,
    url: String,
}

impl Gitpod {
    fn load() -> Self {
        let workspace_url = std::env::var("GITPOD_WORKSPACE_URL").expect("Not running in Gitpod");

        Self {
            domain: workspace_url.replace("https://", "8080-"),
            url: workspace_url.replace("https://", "https://8080-"),
        }
    }

    fn is_host() -> bool {
        std::env::var("GITPOD_WORKSPACE_URL").is_ok()
    }
}

#[derive(Default, Clone, Eq, PartialEq, Debug)]
struct OptionalService<'a> {
    compose_profile: Option<Vec<&'a str>>,
    repositories: Option<Vec<Repository>>,
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() > 1, "Missing command");

    let config = Config::load();

    match args[1].as_str() {
        "setup" => setup(config),
        "hostname" => hostname(config),
        "mobile" => mobile_setup(config),
        "welcome" => welcome(),
        _ => panic!("Unknown command"),
    }
}

fn pwd_input(user_type: &str) -> std::io::Result<String> {
    input(format!(
        "Choose a password for {user_type} users (blank for 'password')"
    ))
    .placeholder("password")
    .default_input("password")
    .required(false)
    .interact()
}

fn setup(mut config: Config) -> std::io::Result<()> {
    intro(BANNER)?;

    let services = prompt_for_optional_services()?;

    let setup_database =
        confirm("Do you want to seed the database with test users, games, etc? (Recommended)")
            .initial_value(true)
            .interact()?;

    let (su_password, password) = if setup_database {
        (pwd_input("admin")?, pwd_input("regular")?)
    } else {
        (String::new(), String::new())
    };

    config.compose_profiles = Some(
        services
            .iter()
            .filter_map(|service| service.compose_profile.clone())
            .flatten()
            .map(std::string::ToString::to_string)
            .collect(),
    );
    config.setup_database = Some(setup_database);

    config.setup_bbppairings = Some(
        services
            .iter()
            .any(|service| service.compose_profile == Some(vec!["swiss-pairings"])),
    );

    config.enable_monitoring = Some(
        services
            .iter()
            .any(|service| service.compose_profile == Some(vec!["monitoring"])),
    );
    config.su_password = Some(su_password);
    config.password = Some(password);

    if Gitpod::is_host() {
        let gitpod = Gitpod::load();
        config.lila_domain = Some(gitpod.domain);
        config.lila_url = Some(gitpod.url);
    }

    config.save()?;

    create_placeholder_dirs();

    let mut repos_to_clone: Vec<Repository> = vec![
        Repository::new("lichess-org", "lila"),
        Repository::new("lichess-org", "lila-ws"),
    ];

    if setup_database {
        repos_to_clone.push(Repository::new("lichess-org", "lila-db-seed"));
    }

    let optional_repos: Vec<Repository> = services
        .iter()
        .filter_map(|service| service.repositories.clone())
        .flatten()
        .collect();

    repos_to_clone.extend(optional_repos);

    for repo in repos_to_clone {
        let mut progress = spinner();
        progress.start(&format!("Cloning {}...", repo.full_name()));

        if repo.clone_path().read_dir()?.next().is_some() {
            progress.stop(format!("Already cloned {} ✓", repo.full_name()));
            continue;
        }

        let mut cmd = std::process::Command::new("git");
        cmd.arg("clone")
            .arg("--origin")
            .arg("upstream")
            .arg("--depth")
            .arg("1")
            .arg("--recurse-submodules")
            .arg(repo.url())
            .arg(repo.clone_path());

        let output = cmd.output().unwrap();
        assert!(
            output.status.success(),
            "Failed to clone repo: {} - {:?}",
            repo.full_name(),
            output
        );

        progress.stop(format!("Cloned {} ✓", repo.full_name()));
    }
    
    // checkout a pr branch in repos/lila
    let mut cmd = std::process::Command::new("git");

    // print all the env vars to the console
    for (key, value) in std::env::vars() {
        println!("{}: {}", key, value);
    }

    let workspace_context = match std::env::var("GITPOD_WORKSPACE_CONTEXT") {
        Ok(value) => value,
        Err(_) => {
            return outro("Environment variable GITPOD_WORKSPACE_CONTEXT is not set, skipping PR checkout\n Starting services...");
        }
    };
    
    // parse the workspace context as JSON
    let workspace_context: Value = serde_json::from_str(&workspace_context)
        .expect("Failed to parse GITPOD_WORKSPACE_CONTEXT as JSON");
    
    // get the pr no from the workspace context
    let pr_no = workspace_context.get("envvars").and_then(|envvars| {
        envvars
            .as_array()
            .and_then(|array| {
                array
                    .iter()
                    .find(|envvar| envvar.get("name").map_or(false, |name| name == "GITPOD_GIT_PR_NUMBER"))
                    .and_then(|envvar| envvar.get("value").and_then(Value::as_str))
            })
    }).unwrap_or("");
    
    // dont checkout if the pr_no is empty
    if pr_no.is_empty() {
        return outro("No PR number found, skipping PR checkout\n Starting services...");
        
    }
    cmd.current_dir("repos/lila")
    .arg("fetch")
    .arg("upstream")
    .arg(format!("pull/{}/head:pr-{}", pr_no, pr_no));

    let status = cmd.status().unwrap();
    if !status.success() {
        // If the checkout failed, checkout to the master branch
        let mut cmd = std::process::Command::new("git");
        cmd.current_dir("repos/lila")
            .arg("checkout")
            .arg("master")
            .status()
            .expect("Failed to checkout to master branch");
    }

    outro("Starting services...")
}

fn create_placeholder_dirs() {
    // Create a placeholder directory for each of the repos
    // otherwise the directories will be created by Docker
    // when the volumes are mounted and they may be owned by root
    [
        Repository::new("lichess-org", "lila"),
        Repository::new("lichess-org", "lila-ws"),
        Repository::new("lichess-org", "lila-db-seed"),
        Repository::new("lichess-org", "lifat"),
        Repository::new("lichess-org", "lila-fishnet"),
        Repository::new("lichess-org", "lila-engine"),
        Repository::new("lichess-org", "lila-search"),
        Repository::new("lichess-org", "lila-gif"),
        Repository::new("lichess-org", "api"),
        Repository::new("lichess-org", "chessground"),
        Repository::new("lichess-org", "pgn-viewer"),
        Repository::new("lichess-org", "scalachess"),
        Repository::new("lichess-org", "mobile"),
        Repository::new("lichess-org", "dartchess"),
        Repository::new("lichess-org", "berserk"),
        Repository::new("cyanfish", "bbpPairings"),
    ]
    .iter()
    .map(Repository::clone_path)
    .for_each(|path| {
        std::fs::create_dir_all(path).unwrap();
    });
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
            repositories: vec![Repository::new("lichess-org", "lila-fishnet")].into(),
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
            repositories: vec![Repository::new("lichess-org", "lila-engine")].into(),
        },
        "External Engine",
        "for connecting a local chess engine to the analysis board",
    )
    .item(
        OptionalService {
            compose_profile: None,
            repositories: vec![Repository::new("lichess-org", "lifat")].into(),
        },
        "Larger static assets",
        "Analysis board engines, background images, voice move models, etc",
    )
    .item(
        OptionalService {
            compose_profile: vec!["search"].into(),
            repositories: vec![Repository::new("lichess-org", "lila-search")].into(),
        },
        "Search",
        "for searching games, forum posts, etc",
    )
    .item(
        OptionalService {
            compose_profile: vec!["gifs"].into(),
            repositories: vec![Repository::new("lichess-org", "lila-gif")].into(),
        },
        "GIF + image generation",
        "for generating animated GIFs and screenshots of games",
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
            repositories: vec![Repository::new("lichess-org", "api")].into(),
        },
        "API docs",
        "standalone API documentation",
    )
    .item(
        OptionalService {
            compose_profile: vec!["chessground"].into(),
            repositories: vec![Repository::new("lichess-org", "chessground")].into(),
        },
        "Chessground",
        "standalone board UI",
    )
    .item(
        OptionalService {
            compose_profile: vec!["pgn-viewer"].into(),
            repositories: vec![Repository::new("lichess-org", "pgn-viewer")].into(),
        },
        "PGN Viewer",
        "standalone PGN viewer",
    )
    .item(
        OptionalService {
            compose_profile: None ,
            repositories: vec![Repository::new("lichess-org", "scalachess")].into(),
        },
        "Scalachess",
        "standalone chess logic library",
    )
    .item(
        OptionalService {
            compose_profile: vec!["mobile"].into(),
            repositories: vec![Repository::new("lichess-org", "mobile")].into(),
        },
        "Mobile app",
        "Flutter-based mobile app",
    )
    .item(
        OptionalService {
            compose_profile: None,
            repositories: vec![Repository::new("lichess-org", "dartchess")].into(),
        },
        "Dartchess",
        "standalone chess library for mobile platforms",
    )
    .item(
        OptionalService {
            compose_profile: None,
            repositories: vec![Repository::new("lichess-org", "berserk")].into(),
        },
        "Berserk",
        "Python API client",
    )
    .item(
        OptionalService {
            compose_profile: vec!["monitoring"].into(),
            repositories: None,
        },
        "Monitoring",
        "Metric collection using InfluxDB",
    )
    .item(
        OptionalService {
            compose_profile: vec!["swiss-pairings"].into(),
            repositories: vec![Repository::new("cyanfish", "bbpPairings")].into(),
        },
        "Swiss Pairings",
        "bbpPairings tool",
    )
    .interact()
}

fn hostname(mut config: Config) -> std::io::Result<()> {
    if Gitpod::is_host() {
        return log::error("Setting of hostname not available on Gitpod");
    }

    let local_ip = match local_ip() {
        Ok(ip) => ip.to_string(),
        _ => "127.0.0.1".to_string(),
    };

    let hostname: String = match select("Select a hostname to access your local Lichess instance:")
        .initial_value("localhost")
        .item("localhost", "localhost", "default")
        .item(
            local_ip.as_str(),
            local_ip.as_str(),
            "Your private IP address, for accessing from other devices on your local network",
        )
        .item(
            "10.0.2.2",
            "10.0.2.2",
            "For accessing from an Android emulator running on this machine",
        )
        .item("other", "Other", "Enter a custom hostname")
        .interact()?
    {
        "other" => input("Enter a custom hostname:  (It must be resolvable)").interact()?,
        selection => selection.to_string(),
    };

    config.lila_domain = Some(format!("{hostname}:8080"));
    config.lila_url = Some(format!("http://{hostname}:8080"));
    config.save()?;

    outro(format!("✔ Local Lichess URL set to http://{hostname}:8080"))
}

fn mobile_setup(mut config: Config) -> std::io::Result<()> {
    intro("On your Android phone, open Developer Options > Wireless Debugging")?;

    let phone_ip = match config.phone_ip {
        Some(ip) => input("Your phone's private IP address")
            .default_input(&ip)
            .interact()?,
        None => input("Your phone's private IP address")
            .placeholder("192.168.x.x or 10.x.x.x")
            .interact()?,
    };

    let connection_port: u16 = input("Connection port")
        .validate(|input: &String| validate_string_length(input, 5))
        .interact()?;

    info("Tap `Pair device with pairing code`")?;

    let pairing_code: u32 = input("Pairing code")
        .validate(|input: &String| validate_string_length(input, 6))
        .interact()?;
    let pairing_port: u16 = input("Pairing port")
        .validate(|input: &String| validate_string_length(input, 5))
        .interact()?;

    config.phone_ip = Some(phone_ip);
    config.connection_port = Some(connection_port);
    config.pairing_code = Some(pairing_code);
    config.pairing_port = Some(pairing_port);
    config.save()?;

    outro("Pairing and connecting to phone...")
}

fn validate_string_length(input: &str, length: usize) -> Result<(), String> {
    match input.len() {
        len if len == length => Ok(()),
        _ => Err(format!("Value should be {length} digits in length")),
    }
}

fn welcome() -> std::io::Result<()> {
    intro("Your Lichess development environment is starting!")?;

    if Gitpod::is_host() {
        info("For full documentation, see: https://lichess-org.github.io/lila-gitpod/")?;
    } else {
        info("For full documentation, see: https://github.com/lichess-org/lila-docker")?;
    }

    note(
        "To monitor the progress:",
        "docker compose logs lila --follow",
    )?;

    outro("🚀")
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    #[test]
    fn test_repository() {
        let repo = Repository::new("lichess-org", "lila");
        assert_eq!(repo.org, "lichess-org");
        assert_eq!(repo.project, "lila");
        assert_eq!(repo.full_name(), "lichess-org/lila");
        assert_eq!(repo.url(), "https://github.com/lichess-org/lila");
        assert_eq!(repo.clone_path(), Path::new("repos/lila"));
    }

    #[test]
    fn test_to_env_proc() {
        let foo = Some("test");
        assert_eq!(to_env!(foo), "FOO=test");
    }

    #[test]
    fn test_set_env_vars_from_struct() {
        let contents = Config {
            compose_profiles: Some(vec!["foo".to_string(), "bar".to_string()]),
            setup_database: Some(true),
            setup_bbppairings: Some(false),
            enable_monitoring: Some(false),
            su_password: Some("foo".to_string()),
            password: Some("bar".to_string()),
            lila_domain: Some("baz:8080".to_string()),
            lila_url: Some("http://baz:8080".to_string()),
            phone_ip: Some("1.2.3.4".to_string()),
            connection_port: Some(1234),
            pairing_code: Some(901234),
            pairing_port: Some(5678),
        }
        .to_env();

        let vars = contents
            .split("\n")
            .flat_map(|line| line.split_once("="))
            .collect::<HashMap<&str, &str>>();

        assert_eq!(vars["COMPOSE_PROFILES"], "foo,bar");
        assert_eq!(vars["SETUP_DATABASE"], "true");
        assert_eq!(vars["SETUP_BBPPAIRINGS"], "false");
        assert_eq!(vars["ENABLE_MONITORING"], "false");
        assert_eq!(vars["SU_PASSWORD"], "foo");
        assert_eq!(vars["PASSWORD"], "bar");
        assert_eq!(vars["LILA_DOMAIN"], "baz:8080");
        assert_eq!(vars["LILA_URL"], "http://baz:8080");
        assert_eq!(vars["PHONE_IP"], "1.2.3.4");
        assert_eq!(vars["CONNECTION_PORT"], "1234");
        assert_eq!(vars["PAIRING_CODE"], "901234");
        assert_eq!(vars["PAIRING_PORT"], "5678");
    }

    #[test]
    fn test_gitpod_lila_url() {
        std::env::set_var(
            "GITPOD_WORKSPACE_URL",
            "https://lichessorg-liladocker-abc123.ws-us123.gitpod.io",
        );

        let gitpod = Gitpod::load();
        assert_eq!(
            gitpod.domain,
            "8080-lichessorg-liladocker-abc123.ws-us123.gitpod.io"
        );
        assert_eq!(
            gitpod.url,
            "https://8080-lichessorg-liladocker-abc123.ws-us123.gitpod.io"
        );
    }
}
