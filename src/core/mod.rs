pub mod init;
pub mod io;

use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleConfig {
    pub name: String,
    pub directory: String,
    pub repos: Vec<SourceRepo>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SourceRepo {
    pub name: String,
    pub url: String,
    pub version_control_system: String,
    pub hosting_platform: String,
}
