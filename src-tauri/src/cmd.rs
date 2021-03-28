use std::collections::HashSet;

use serde::Deserialize;

#[derive(Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum BrowseFor {
    Program,
    Package,
}

#[derive(Deserialize, Debug)]
#[serde(tag = "cmd", rename_all = "camelCase")]
pub enum Cmd {
    Browse {
        callback: String,
        error: String,
        browse_for: BrowseFor,
    },
    Startup {
        callback: String,
        error: String,
    },
    Install {
        callback: String,
        error: String,
        features: HashSet<String>,
        options: HashSet<String>,
    },
    Uninstall {
        callback: String,
        error: String,
    },
    Launch,
}
