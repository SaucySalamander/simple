use clap::{App, SubCommand};

mod init;
fn main() {
    let matches = App::new("Simple")
        .subcommand(SubCommand::with_name("init"))
        .about("Command to init simple repo in directory")
        .get_matches();

    match matches.subcommand_name() {
        Some("init") => init::init_simple(),
        _ => panic!("Subcommand not found"),
    }
}

#[cfg(test)]
mod tests {
    use std::{path::Path, process::Command};

    use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};

    #[test]
    fn init_test_good_arg() {
        let mut cmd = Command::cargo_bin("simple").unwrap();

        cmd.arg("init");
        cmd.assert().success();

        assert!(Path::new("./.simple/simple.toml").exists());

        cleanup();
    }

    fn cleanup(){
        let _ = std::fs::remove_dir_all("./.simple");
    }

    #[test]
    fn init_test_bad_arg() {
        let mut cmd = Command::cargo_bin("simple").unwrap();

        cmd.arg("apples");
        cmd.assert().failure();
    }
}
