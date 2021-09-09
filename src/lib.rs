use clap::{App, Arg, SubCommand};

pub mod core;

pub fn run_simple_repo() {
    let matches = App::new("Simple")
        .subcommand(
            SubCommand::with_name("init")
                .about("Command to init simple repo in directory")
                .arg(
                    Arg::with_name("file")
                        .short("f")
                        .required(false)
                        .takes_value(true)
                        .help("Optional file to initialize the simple repo with"),
                ),
        )
        .subcommand(
            SubCommand::with_name("add")
                .about("Command to add source repository to the simple repo"),
        )
        .get_matches();

    match matches.subcommand_name() {
        Some("init") => core::init::init_simple(matches.subcommand_matches("init").unwrap()),
        Some("add") => core::io::add_source_repository(),
        _ => panic!("Subcommand not found"),
    }
}
