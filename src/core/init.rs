use std::fs::File;
use std::io::{Write, BufReader, BufRead};
use crate::core::{SimpleConfig, SourceRepo};
use crate::core::io::add_source_repository;
use clap::ArgMatches;

pub fn init_simple(matches: &ArgMatches) {
    create_simple_repository(matches);
}

fn create_simple_repository(matches: &ArgMatches) {
    let path = std::path::Path::new("./.simple/simple.toml");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();

    let mut file = File::create(path).unwrap();

    let simple_config = SimpleConfig {
        name: "test".to_string(),
        directory: "./.simple/simple.toml".to_string(),
        repos: Vec::new(),
    };

    let data = toml::to_string(&simple_config).unwrap();

    let _ = write!(file, "{}", data);

    if matches.is_present("file") {
        println!("{}", matches.value_of("file").unwrap());

        let temp_string = "./".to_string() + ( matches.value_of("file").unwrap());
        let path = std::path::Path::new(&temp_string);
        let file = File::open(path).unwrap();
        let reader = BufReader::new(file);
    
        for (_, line) in reader.lines().enumerate() {
            let line = line.unwrap();
            let new_repo = SourceRepo{
                name: line,
                url: "".to_string(),
                hosting_platform: "GitHub".to_string(),
                version_control_system: "git".to_string(),
            };

            add_source_repository(path, new_repo);
        }
    }
}
