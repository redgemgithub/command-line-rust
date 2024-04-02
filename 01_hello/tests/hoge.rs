use std::process::Command;

#[test]
fn hoge1() {
    assert!(true);
}

#[test]
fn hoge2() {
    let mut cmd = Command::new("ls");
    let res = cmd.output();
    assert!(res.is_ok());
}
