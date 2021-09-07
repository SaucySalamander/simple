use clap::{App, SubCommand};

mod init;
mod model;
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
    use std::{fs, path::Path, process::Command};

    use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};

    use crate::model::SimpleConfig;

    #[test]
    fn init_test_good_arg() {
        let mut cmd = Command::cargo_bin("simple").unwrap();

        cmd.arg("init");
        cmd.assert().success();

        cleanup();
    }

    #[test]
    fn init_test_good_arg_file_created() {
        let mut cmd = Command::cargo_bin("simple").unwrap();

        cmd.arg("init");
        cmd.assert().success();
        let path = "./.simple/simple.toml";

        assert!(Path::new(path).exists());

        cleanup();
    }

    #[test]
    fn init_test_good_arg_file_contents_validated() {
        let mut cmd = Command::cargo_bin("simple").unwrap();

        cmd.arg("init");
        cmd.assert().success();

        let path = "./.simple/simple.toml";

        assert!(Path::new(path).exists());

        let data = fs::read_to_string(path).unwrap();
        let test_object: SimpleConfig = toml::from_str(data.as_str()).unwrap();

        assert_eq!(test_object.name, "test");
        assert!(test_object.repos.is_none());

        cleanup();
    }

    fn cleanup() {
        let _ = std::fs::remove_dir_all("./.simple");
    }

    #[test]
    fn init_test_bad_arg() {
        let mut cmd = Command::cargo_bin("simple").unwrap();

        cmd.arg("apples");
        cmd.assert().failure();
    }
}
