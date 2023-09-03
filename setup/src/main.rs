use std::vec;

use cliclack::{intro, multiselect, MultiSelect};

const BANNER: &str = r#"
   |\_    _ _      _
   /o \  | (_) ___| |__   ___  ___ ___   ___  _ __ __ _
 (_. ||  | | |/ __| '_ \ / _ \/ __/ __| / _ \| '__/ _` |
   /__\  | | | (__| | | |  __/\__ \__ \| (_) | | | (_| |
  )___(  |_|_|\___|_| |_|\___||___/___(_)___/|_|  \__, |
                                                   |___/
"#;

#[derive(Debug)]
struct Profile {
    name: &'static str,
    compose_profile: &'static str,
}

const PROFILES: &[Profile; 5] = &[
    Profile {
        name: "Default (lila, lila-ws, mongodb, redis)",
        compose_profile: "",
    },
    Profile {
        name: "Stockfish (for playing against or analyzing games)",
        compose_profile: "stockfish",
    },
    Profile {
        name: "External Engine",
        compose_profile: "external-engine",
    },
    Profile {
        name: "Search (elasticsearch, lila-search)",
        compose_profile: "search",
    },
    Profile {
        name: "Images (for generating gifs and thumbnails)",
        compose_profile: "images",
    },
];

fn main() -> std::io::Result<()> {
    intro(BANNER)?;

    let mut additional_tools: MultiSelect<&str> =
        multiselect("Select which services to run").initial_values(vec![""]);

    for profile in PROFILES {
        additional_tools = additional_tools.item(profile.compose_profile, profile.name, "");
    }

    let result = additional_tools.interact()?;

    std::fs::write("../.env", format!("COMPOSE_PROFILES={}", result.join(",")))?;

    Ok(())
}
