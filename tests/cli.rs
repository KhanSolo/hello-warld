use std::process::Command;

#[test]
fn sample(){
    let mut command = Command::new("ls");
    let output = command.output();
    assert!(output.is_ok());
}