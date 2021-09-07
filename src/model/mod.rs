use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize)]
pub struct SimpleConfig {
    pub name: String,
    pub repos: Option<Vec<Repositories>>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Repositories {
    pub name: String,
}
