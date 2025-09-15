#![warn(clippy::pedantic)]

use cliclack::{
    confirm, input, intro,
    log::{error, info, step, success, warning},
    multiselect, note, outro, select, spinner,
};
use local_ip_address::local_ip;
use serde::{Deserialize, Serialize};
use std::{
    format,
    io::Error,
    path::{Path, PathBuf},
    process::Command,
};

const BANNER: &str = r"
   |\_    _ _      _
   /o \  | (_) ___| |__   ___  ___ ___   ___  _ __ __ _
 (_. ||  | | |/ __| '_ \ / _ \/ __/ __| / _ \| '__/ _` |
   /__\  | | | (__| | | |  __/\__ \__ \| (_) | | | (_| |
  )___(  |_|_|\___|_| |_|\___||___/___(_)___/|_|  \__, |
                                                   |___/
";

const DEFAULT_PASSWORD: &str = "password";

#[derive(Serialize, Deserialize, Debug, Default)]
struct Config {
    quick_setup: Option<bool>,
    compose_profiles: Option<Vec<String>>,
    lila_ws_container: Option<String>,
    setup_database: Option<bool>,
    setup_bbppairings: Option<bool>,
    mock_email: Option<bool>,
    enable_monitoring: Option<bool>,
    enable_rate_limiting: Option<bool>,
    su_password: Option<String>,
    password: Option<String>,
    setup_api_tokens: Option<bool>,
    lila_domain: Option<String>,
    lila_url: Option<String>,
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
            quick_setup,
            compose_profiles,
            lila_ws_container,
            setup_database,
            setup_bbppairings,
            mock_email,
            enable_monitoring,
            enable_rate_limiting,
            su_password,
            password,
            setup_api_tokens,
            lila_domain,
            lila_url,
        } = self;
        let compose_profiles_string = compose_profiles
            .clone()
            .map(|v| v.join(","))
            .unwrap_or_default();

        vec![
            to_env!(quick_setup),
            to_env!(compose_profiles, compose_profiles_string),
            to_env!(lila_ws_container),
            to_env!(setup_database),
            to_env!(setup_bbppairings),
            to_env!(mock_email),
            to_env!(enable_monitoring),
            to_env!(enable_rate_limiting),
            to_env!(su_password),
            to_env!(password),
            to_env!(setup_api_tokens),
            to_env!(lila_domain),
            to_env!(lila_url),
        ]
        .iter()
        .filter(|line| !line.is_empty())
        .map(std::string::ToString::to_string)
        .collect::<Vec<String>>()
        .join("\n")
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
    workspace_context: GitpodWorkspaceContext,
}

#[derive(Debug, Default, Deserialize, PartialEq)]
struct GitpodWorkspaceContext {
    envvars: Option<Vec<GitpodEnvVar>>,
}

#[derive(Debug, Deserialize, PartialEq)]
struct GitpodEnvVar {
    name: String,
    value: String,
}

impl Gitpod {
    fn load() -> Self {
        let workspace_url =
            std::env::var("GITPOD_WORKSPACE_URL").expect("Missing env GITPOD_WORKSPACE_URL");

        let workspace_context: GitpodWorkspaceContext = serde_json::from_str(
            &std::env::var("GITPOD_WORKSPACE_CONTEXT")
                .expect("Missing env GITPOD_WORKSPACE_CONTEXT"),
        )
        .expect("Failed to parse GITPOD_WORKSPACE_CONTEXT");

        Self {
            domain: workspace_url.replace("https://", "8080-"),
            url: workspace_url.replace("https://", "https://8080-"),
            workspace_context,
        }
    }

    fn is_host() -> bool {
        std::env::var("GITPOD_WORKSPACE_URL").is_ok()
    }

    fn get_context_for(&self, name: &str) -> Option<&str> {
        self.workspace_context
            .envvars
            .as_ref()
            .and_then(|envvars| envvars.iter().find(|envvar| envvar.name == name))
            .map(|envvar| envvar.value.as_str())
    }
}

#[derive(Default, Clone, Eq, PartialEq, Debug)]
struct OptionalService<'a> {
    compose_profile: Option<Vec<&'a str>>,
    repositories: Option<Vec<Repository>>,
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum Setting {
    SetupDatabase,
    EnableRateLimiting,
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    assert!(args.len() > 1, "Missing command");

    let config = Config::load();

    match args[1].as_str() {
        "setup" => setup(config, true, std::env::var("NONINTERACTIVE").is_ok()),
        "add_services" => setup(config, false, false),
        "hostname" => hostname(config),
        "welcome" => welcome(config),
        "gitpod_public" => gitpod_public(),
        _ => panic!("Unknown command"),
    }
}

fn pwd_input(user_type: &str) -> std::io::Result<String> {
    input(format!(
        "Choose a password for {user_type} users (blank for 'password')"
    ))
    .placeholder(DEFAULT_PASSWORD)
    .default_input(DEFAULT_PASSWORD)
    .required(false)
    .interact()
}

#[derive(Debug, Clone, Eq, PartialEq)]
enum SetupMethod {
    Quick,
    Advanced,
}

#[allow(clippy::too_many_lines)]
fn setup(mut config: Config, first_setup: bool, noninteractive: bool) -> std::io::Result<()> {
    if first_setup {
        intro(BANNER)?;
    } else {
        intro("Adding services...")?;
        warning(
            "NOTE: This will not remove any existing services that may be running.\nOnly the newly selected ones will be added."
        )?;
    }

    let mut services: Vec<OptionalService<'static>> = vec![];

    info(
        [
            "Your Lichess development site will include a basic instance of Lichess.",
            "You will be able to use the site, play games, and test most of the main functionality.",
            "If you want additional features, you can select them here.",
        ]
        .join("\n"),
    )?;
    let is_quick_setup = noninteractive
        || select("Choose a setup method:")
            .item(
                SetupMethod::Quick,
                "Quick",
                "If you just want a basic lila instance without making any code changes",
            )
            .item(
                SetupMethod::Advanced,
                "Advanced",
                "If you want to make changes or test specific features",
            )
            .interact()?
            == SetupMethod::Quick;
    config.quick_setup = Some(is_quick_setup);

    if noninteractive || is_quick_setup {
        config.password = Some(DEFAULT_PASSWORD.to_string());
        config.su_password = Some(DEFAULT_PASSWORD.to_string());
        config.setup_api_tokens = Some(true);
        config.enable_rate_limiting = Some(true);
        config.setup_database = Some(true);
    } else {
        if has_git_lfs() {
            success("✓ Git LFS is installed")?;
        } else {
            assert!(confirm(
                [
                    "Git LFS is not installed. It is used to manage large files in Git repositories.",
                    "Some optional features may not work without it.",
                    "You can read about it at https://git-lfs.com/",
                    "Do you want to continue anyway?",
                ]
                .join("\n"),
            )
            .initial_value(true)
            .interact()?, "Cancelled setup");
        }

        services = prompt_for_services()?;

        let options = prompt_for_options(first_setup)?;

        let (su_password, password) = if options.contains(&Setting::SetupDatabase) {
            (pwd_input("admin")?, pwd_input("regular")?)
        } else {
            (DEFAULT_PASSWORD.to_string(), DEFAULT_PASSWORD.to_string())
        };

        config.setup_api_tokens = Some(
            options.contains(&Setting::SetupDatabase)
                && if password != "password" || su_password != "password" {
                    confirm("Do you want to setup default API tokens for the admin and regular users? Will be created with `lip_{username}` format")
            .interact()?
                } else {
                    true
                },
        );

        config.setup_database = Some(options.contains(&Setting::SetupDatabase));
        config.enable_rate_limiting = Some(options.contains(&Setting::EnableRateLimiting));
        config.su_password = Some(su_password);
        config.password = Some(password);

        config.setup_bbppairings = Some(
            services
                .iter()
                .any(|service| service.compose_profile == Some(vec!["swiss-pairings"])),
        );

        config.mock_email = Some(
            !services
                .iter()
                .any(|service| service.compose_profile == Some(vec!["email"])),
        );

        config.enable_monitoring = Some(
            services
                .iter()
                .any(|service| service.compose_profile == Some(vec!["monitoring"])),
        );

        config.lila_ws_container = Some(
            if services
                .iter()
                .any(|service| service.compose_profile == Some(vec!["lila-ws-build"]))
            {
                "build".to_string()
            } else {
                "image".to_string()
            },
        );
    }

    if Gitpod::is_host()
        && !noninteractive
        && confirm("By default, only this browser session can access your Lichess development site.\nWould you like it to be accessible to other clients?")
            .initial_value(false)
            .interact()?
    {
        gitpod_public()?;
    }

    let selected_profiles: Vec<String> = services
        .iter()
        .filter_map(|service| service.compose_profile.as_ref())
        .flatten()
        .map(ToString::to_string)
        .collect();

    let mut profiles: Vec<String> = selected_profiles;
    if !first_setup {
        profiles.extend(config.compose_profiles.unwrap_or_default());
    }
    if is_quick_setup {
        profiles.push("quick".to_string());
    } else {
        profiles.push("base".to_string());
    }
    profiles.sort();
    profiles.dedup();

    config.compose_profiles = Some(profiles);

    if Gitpod::is_host() {
        let gitpod = Gitpod::load();
        config.lila_domain = Some(gitpod.domain);
        config.lila_url = Some(gitpod.url);
    }

    config.save()?;

    if !is_quick_setup {
        create_placeholder_dirs();

        let mut repos_to_clone: Vec<Repository> = vec![Repository::new("lichess-org", "lila")];

        if config.setup_database.unwrap_or_default() {
            repos_to_clone.push(Repository::new("lichess-org", "lila-db-seed"));
        }

        let optional_repos: Vec<Repository> = services
            .iter()
            .filter_map(|service| service.repositories.clone())
            .flatten()
            .collect();

        repos_to_clone.extend(optional_repos);

        for repo in repos_to_clone {
            let progress = spinner();
            progress.start(format!("Cloning {}...", repo.full_name()));

            if repo.clone_path().read_dir()?.next().is_some() {
                progress.stop(format!("✓ Already cloned {}", repo.full_name()));
                continue;
            }

            let mut cmd = Command::new("git");
            cmd.arg("clone")
                .arg("--origin")
                .arg("upstream")
                .arg("--depth")
                .arg("1")
                .arg("--recurse-submodules")
                .arg(repo.url())
                .arg(repo.clone_path());

            let output = cmd.output()?;
            assert!(
                output.status.success(),
                "Failed to clone repo: {} - {output:?}",
                repo.full_name()
            );

            progress.stop(format!("✓ Cloned {}", repo.full_name()));
        }
    }

    if Gitpod::is_host() {
        gitpod_checkout_pr()?;
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
        Repository::new("lichess-org", "lila-fishnet"),
        Repository::new("lichess-org", "lila-engine"),
        Repository::new("lichess-org", "lila-gif"),
        Repository::new("lichess-org", "lila-push"),
        Repository::new("lichess-org", "api"),
        Repository::new("lichess-org", "chessground"),
        Repository::new("lichess-org", "pgn-viewer"),
        Repository::new("lichess-org", "scalachess"),
        Repository::new("lichess-org", "berserk"),
        Repository::new("cyanfish", "bbpPairings"),
    ]
    .iter()
    .map(Repository::clone_path)
    .for_each(|path| {
        std::fs::create_dir_all(path).unwrap();
    });
}

fn gitpod_checkout_pr() -> std::io::Result<()> {
    let gitpod = Gitpod::load();

    let Some(pr_no) = gitpod.get_context_for("LILA_PR") else {
        return step("No lila PR specified, using default branch");
    };

    let pr_url = format!("https://github.com/lichess-org/lila/pull/{pr_no}");
    let branch_name = format!("pr-{pr_no}");

    let progress = spinner();
    progress.start(format!("Checking out lila PR #{pr_no}: {pr_url}..."));

    let mut cmd = Command::new("git");
    cmd.current_dir("repos/lila")
        .arg("fetch")
        .arg("upstream")
        .arg(format!("pull/{pr_no}/head:{branch_name}"))
        .arg("--depth")
        .arg("25")
        .arg("--recurse-submodules");

    let output = cmd.output()?;
    assert!(
        output.status.success(),
        "Failed to fetch upstream PR #{pr_no} - {output:?}",
    );

    let mut cmd = Command::new("git");
    cmd.current_dir("repos/lila")
        .arg("checkout")
        .arg(&branch_name);

    let output = cmd.output()?;
    assert!(
        output.status.success(),
        "Failed to checkout PR branch {branch_name} - {output:?}",
    );

    progress.stop(format!("✓ Checked out PR #{pr_no} - {pr_url}"));
    Ok(())
}

#[allow(clippy::too_many_lines)]
fn prompt_for_services() -> Result<Vec<OptionalService<'static>>, Error> {
    multiselect(
        "Select which optional services to include:\n    (Use arrows, <space> to toggle, <enter> to continue)\n",
    )
    .required(false)
    .item(
        OptionalService {
            compose_profile: vec!["mongo-express"].into(),
            repositories: None,
        },
        "Database admin interface",
        "Mongo Express for viewing database structure and data",
    )
    .item(
        OptionalService {
            compose_profile: vec!["lila-ws-build"].into(),
            repositories: vec![Repository::new("lichess-org", "lila-ws")].into(),
        },
        "Websocket source code",
        "Only needed if you want to make changes, otherwise a prebuilt lila-ws image will be used",
    )
    .item(
        OptionalService {
            compose_profile: vec!["email"].into(),
            repositories: None,
        },
        "Outbound email testing",
        "for capturing and debugging outbound email",
    )
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
        "Stockfish Game Analysis",
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
            compose_profile: vec!["search"].into(),
            repositories: None,
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
            compose_profile: vec!["push"].into(),
            repositories: vec![Repository::new("lichess-org", "lila-push")].into(),
        },
        "Push server",
        "for Lichess notifications",
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

fn prompt_for_options(first_setup: bool) -> Result<Vec<Setting>, Error> {
    multiselect("Select options:\n")
        .required(false)
        .item(
            Setting::SetupDatabase,
            if first_setup {
                "Seed the database with test users, games, etc. (Recommended)"
            } else {
                "Re-seed the database with test users, games, etc."
            },
            "",
        )
        .item(
            Setting::EnableRateLimiting,
            "Enable rate limiting",
            "To be prod-like. Can be disabled for development/testing purposes",
        )
        .initial_values(if first_setup {
            vec![Setting::SetupDatabase, Setting::EnableRateLimiting]
        } else {
            vec![Setting::EnableRateLimiting]
        })
        .interact()
}

fn hostname(mut config: Config) -> std::io::Result<()> {
    if Gitpod::is_host() {
        return error("Setting of hostname not available on Gitpod");
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

fn welcome(config: Config) -> std::io::Result<()> {
    intro("Your Lichess instance is starting!")?;

    note(
        "The site will be available at:",
        config
            .lila_url
            .unwrap_or("http://localhost:8080".to_owned()),
    )?;

    if Gitpod::is_host() {
        note(
            "For full documentation, see:",
            "https://lichess-org.github.io/lila-gitpod/",
        )?;
    } else {
        note(
            "For full documentation, see:",
            "https://github.com/lichess-org/lila-docker",
        )?;
    }

    note("To monitor the progress:", "./lila-docker logs")?;

    outro("🚀")
}

fn gitpod_public() -> std::io::Result<()> {
    if !Gitpod::is_host() {
        return Err(std::io::Error::other(
            "This command is only available on Gitpod",
        ));
    }

    let progress = spinner();
    progress.start("Making http port 8080 publicly accessible...");

    let mut cmd = Command::new("gp");
    cmd.arg("ports").arg("visibility").arg("8080:public");

    let output = cmd.output().expect("Command failed");
    let stdout = String::from_utf8(output.stdout).expect("Failed to parse stdout");

    if !stdout.contains("port 8080 is now public") {
        return Err(std::io::Error::other("Failed to make port 8080 public"));
    }

    progress.stop("✓ Port 8080 is now publicly accessible");
    outro(Gitpod::load().url)
}

fn has_git_lfs() -> bool {
    Command::new("git")
        .arg("lfs")
        .arg("version")
        .output()
        .map(|output| output.status.success())
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;

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
            quick_setup: Some(true),
            compose_profiles: Some(vec!["foo".to_string(), "bar".to_string()]),
            lila_ws_container: Some("image".to_string()),
            setup_database: Some(true),
            setup_bbppairings: Some(false),
            mock_email: Some(false),
            enable_monitoring: Some(false),
            enable_rate_limiting: Some(true),
            su_password: Some("foo".to_string()),
            password: Some("bar".to_string()),
            setup_api_tokens: Some(false),
            lila_domain: Some("baz:8080".to_string()),
            lila_url: Some("http://baz:8080".to_string()),
        }
        .to_env();

        assert_eq!(
            contents,
            vec![
                "QUICK_SETUP=true",
                "COMPOSE_PROFILES=foo,bar",
                "LILA_WS_CONTAINER=image",
                "SETUP_DATABASE=true",
                "SETUP_BBPPAIRINGS=false",
                "MOCK_EMAIL=false",
                "ENABLE_MONITORING=false",
                "ENABLE_RATE_LIMITING=true",
                "SU_PASSWORD=foo",
                "PASSWORD=bar",
                "SETUP_API_TOKENS=false",
                "LILA_DOMAIN=baz:8080",
                "LILA_URL=http://baz:8080",
            ]
            .join("\n")
        );
    }

    #[test]
    fn test_env_removes_empty_lines() {
        let contents = Config {
            quick_setup: None,
            compose_profiles: None,
            lila_ws_container: None,
            setup_database: None,
            setup_bbppairings: None,
            mock_email: None,
            enable_monitoring: None,
            enable_rate_limiting: None,
            su_password: None,
            password: None,
            setup_api_tokens: None,
            lila_domain: Some("baz:8080".to_string()),
            lila_url: Some("http://baz:8080".to_string()),
        }
        .to_env();

        assert_eq!(
            contents,
            vec![
                "COMPOSE_PROFILES=",
                "LILA_DOMAIN=baz:8080",
                "LILA_URL=http://baz:8080"
            ]
            .join("\n")
        );
    }

    #[test]
    fn test_gitpod_lila_url() {
        std::env::set_var(
            "GITPOD_WORKSPACE_URL",
            "https://lichessorg-liladocker-abc123.ws-us123.gitpod.io",
        );
        std::env::set_var("GITPOD_WORKSPACE_CONTEXT", "{}");

        let gitpod = Gitpod::load();
        assert_eq!(
            gitpod.domain,
            "8080-lichessorg-liladocker-abc123.ws-us123.gitpod.io"
        );
        assert_eq!(
            gitpod.url,
            "https://8080-lichessorg-liladocker-abc123.ws-us123.gitpod.io"
        );
        assert_eq!(gitpod.workspace_context, GitpodWorkspaceContext::default());
        assert_eq!(gitpod.get_context_for("LILA_PR"), None);
    }

    #[test]
    fn test_gitpod_lila_url_with_pr_context() {
        std::env::set_var(
            "GITPOD_WORKSPACE_URL",
            "https://lichessorg-liladocker-abc123.ws-us123.gitpod.io",
        );
        std::env::set_var(
            "GITPOD_WORKSPACE_CONTEXT",
            r#"{"envvars":[{"name":"LILA_PR","value":"12345"}]}"#,
        );

        let gitpod = Gitpod::load();

        assert_eq!(
            gitpod.workspace_context,
            GitpodWorkspaceContext {
                envvars: Some(vec![GitpodEnvVar {
                    name: "LILA_PR".to_string(),
                    value: "12345".to_string(),
                }])
            }
        );

        assert_eq!(gitpod.get_context_for("LILA_PR"), Some("12345"));
    }

    #[test]
    fn test_gitpod_lila_url_with_context_but_no_pr() {
        std::env::set_var(
            "GITPOD_WORKSPACE_URL",
            "https://lichessorg-liladocker-abc123.ws-us123.gitpod.io",
        );
        std::env::set_var(
            "GITPOD_WORKSPACE_CONTEXT",
            r#"{"envvars":[{"name":"FOO","value":"BAR"}]}"#,
        );

        let gitpod = Gitpod::load();

        assert_eq!(
            gitpod.workspace_context,
            GitpodWorkspaceContext {
                envvars: Some(vec![GitpodEnvVar {
                    name: "FOO".to_string(),
                    value: "BAR".to_string(),
                }])
            }
        );

        assert_eq!(gitpod.get_context_for("FOO"), Some("BAR"));
        assert_eq!(gitpod.get_context_for("LILA_PR"), None);
    }
}
