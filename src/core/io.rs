use crate::core::SimpleConfig;
use std::fs::{self, File, OpenOptions};
use std::io::prelude::*;
use std::path::Path;

use super::SourceRepo;

pub fn add_source_repository(path: &Path, repo: SourceRepo) {
    let contents =
        fs::read_to_string("/home/sevenofnine/Workspace/simple/.simple/simple.toml").unwrap();
    let mut temp_config: SimpleConfig = toml::from_str(contents.as_str()).unwrap();
    temp_config.repos.push(repo);

    let modified_file_data = toml::to_string(&temp_config).unwrap();

    let mut file = OpenOptions::new().write(true).open("/home/sevenofnine/Workspace/simple/.simple/simple.toml").unwrap();
    write!(file, "{}", modified_file_data).unwrap();
}

fn _modify_simple_repo_name() {
    //TODO create function modify_simple_repo_name
}

fn _remove_source_repository() {
    //TODO create function remove_source_repository
}

fn _modify_source_repository() {
    //TODO create function modify_source_repository
}
