use assert_cmd::Command;
use predicates::str::contains;

#[test]
fn cli_too_many_args() {
    Command::cargo_bin("rlox")
        .unwrap()
        .args(&["one", "two"])
        .assert()
        .code(64)
        .failure();
}

#[test]
fn cli_one_arg_with_invalid_filepath() {
    Command::cargo_bin("rlox")
        .unwrap()
        .args(&["./tests/i-do-not-exist.txt"])
        .assert()
        .failure();
}

#[test]
fn cli_one_arg_with_valid_filepath() {
    Command::cargo_bin("rlox")
        .unwrap()
        .args(&["./tests/test_script.txt"])
        .assert()
        .stdout(contains("print \"hello, world\";"))
        .success();
}

#[test]
fn cli_no_arg() {
    Command::cargo_bin("rlox")
        .unwrap()
        .write_stdin("hello\nexit")
        .assert()
        .stdout(contains("> hello"));
}
