use assert_cmd::Command;

#[test]
fn test_main_output() {
    Command::cargo_bin("isotopes")
        .unwrap()
        .assert()
        .success()
        .stdout("Hello, world!\n");
}