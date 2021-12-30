use assert_cmd::prelude::*;
use predicates::str::contains;
use std::process::Command;

#[test]
fn cli_too_many_args() {
    Command::cargo_bin("rlox")
        .unwrap()
        .args(&["one", "two"])
        .assert()
        .code(64);
}

#[test]
fn cli_one_arg() {
    Command::cargo_bin("rlox")
        .unwrap()
        .args(&["./tests/test_script.txt"])
        .assert()
        .stdout(contains("print \"hello, world\";"));
}
