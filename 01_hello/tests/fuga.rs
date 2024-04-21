use assert_cmd::Command;

#[test]
fn fuga1() {
    let mut cmd = Command::new("ls");
    let res = cmd.output();
    assert!(res.is_ok());
}
