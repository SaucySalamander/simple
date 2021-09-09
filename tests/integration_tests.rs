use simple::core::SimpleConfig;
use assert_cmd::prelude::{CommandCargoExt, OutputAssertExt};
use std::{env, fs, path::Path, process::Command};

#[test]
fn init_test_good_arg() {
    let mut cmd = Command::cargo_bin("simple").unwrap();

    cmd.arg("init");
    cmd.assert().success();

    cleanup();
}

#[test]
fn init_test_init_arg_with_file() {
    let path = env::current_dir().unwrap();
    println!("The current directory is {}", path.display());

    let mut cmd = Command::cargo_bin("simple").unwrap();

    cmd.arg("init").arg("-f").arg("tests/temp.txt");
    cmd.assert()
        .success()
        .stdout(predicates::str::is_match("^tests/temp.txt\n$").unwrap());

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
    assert_eq!(test_object.repos.len(), 0);

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
