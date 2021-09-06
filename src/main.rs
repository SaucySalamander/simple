use clap::{App, SubCommand};

fn main() {
    let matches = App::new("Simple")
        .subcommand(SubCommand::with_name("init"))
        .about("Command to init simple repo in directory")
        .get_matches();

    match matches.subcommand_name() {
        Some("init") => println!("Init simple repo"),
        _ => panic!("Pass a valid command"),
    }
}

#[cfg(test)]
mod tests {
    use std::process::Command;

    use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};

    #[test]
    fn init_test_good_arg() {
        let mut cmd = Command::cargo_bin("simple").unwrap();

        cmd.arg("init");
        cmd.assert()
            .success()
            .stdout(predicates::str::contains("Init simple repo"));
    }

    #[test]
    fn init_test_bad_arg() {
        let mut cmd = Command::cargo_bin("simple").unwrap();

        cmd.arg("apples");
        cmd.assert().failure();
    }
}
