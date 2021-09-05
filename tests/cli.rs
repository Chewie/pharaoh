use assert_cmd::Command;
use assert_fs::prelude::*;

fn command_in_tmpdir() -> (assert_cmd::Command, assert_fs::TempDir) {
    let tmp = assert_fs::TempDir::new().unwrap();

    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!())
        .unwrap();

    cmd.current_dir(tmp.path());

    (cmd, tmp)
}


#[test]
fn test_no_yaml() {
    // GIVEN
    let (mut cmd, _tmp) = command_in_tmpdir();

    // WHEN


    // THEN
    cmd.assert()
        .stdout("No test case found. Exiting.\n")
        .success();
}

#[test]
fn test_empty_yaml() {
    // GIVEN
    let (mut cmd, tmp) = command_in_tmpdir();

    let _yaml = tmp.child("test.yaml")
        .touch()
        .unwrap();

    // WHEN

    // THEN
    cmd.assert()
        .stdout("Running tests for ./test.yaml\n")
        .success();
}

#[test]
fn test_non_yaml_are_ignored() {
    // GIVEN
    let (mut cmd, tmp) = command_in_tmpdir();

    let _not_a_yaml = tmp.child("foo.bar")
        .touch()
        .unwrap();

    // WHEN

    // THEN
    cmd.assert()
        .stdout("No test case found. Exiting.\n")
        .success();
}
