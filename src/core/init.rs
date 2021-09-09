use std::fs::File;
use std::io::Write;
use crate::core::SimpleConfig;
use clap::ArgMatches;

pub fn init_simple(matches: &ArgMatches) {
    create_simple_repository(matches);
}

fn create_simple_repository(matches: &ArgMatches) {
    let path = std::path::Path::new("./.simple/simple.toml");
    let prefix = path.parent().unwrap();
    std::fs::create_dir_all(prefix).unwrap();

    let mut file = File::create(path).unwrap();

    if matches.is_present("file") {
        println!("{}", matches.value_of("file").unwrap());
    } else {
        println!("Could not find input file");
    }

    let simple_config = SimpleConfig {
        name: "test".to_string(),
        repos: None,
    };

    let data = toml::to_string(&simple_config).unwrap();

    let _ = write!(file, "{}", data);
}
