//use std::process::Command;
use assert_cmd::{Command, assert::OutputAssertExt};

#[test]
fn sample() {
    let mut command = std::process::Command::new("ls");
    let output = command.output();
    assert!(output.is_ok());
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();
    cmd.assert().success();
}

#[test]
fn true_ok(){
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}