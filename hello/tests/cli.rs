use assert_cmd::{Command};

#[test]
fn sample() {
    let mut command = std::process::Command::new("ls");
    let output = command.output();
    assert!(output.is_ok());
}

#[test]
fn runs() {
    let mut cmd = Command::cargo_bin("hello").unwrap();    
    cmd.assert().success().stdout("Hello, world!\n"); 
}


#[test]
fn true_ok(){
    let mut cmd = Command::cargo_bin("true").unwrap();
    cmd.assert().success();
}

#[test]
fn false_not_ok(){
    let mut cmd = Command::cargo_bin("false").unwrap();
    cmd.assert().failure();
}