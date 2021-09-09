use std::fs::File;
use std::io::Write;
use crate::core::SimpleConfig;

pub fn init_simple() {
    create_simple_directory();
}

fn create_simple_directory() {
    let path = std::path::Path::new("./.simple/simple.toml");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();

    let simple_config = SimpleConfig {
        name: "test".to_string(),
        repos: None,
    };

    let mut file = File::create(path).unwrap();

    let data = toml::to_string(&simple_config).unwrap();

    let _ = write!(file, "{}", data);
}
