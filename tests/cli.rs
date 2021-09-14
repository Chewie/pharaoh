use assert_cmd::Command;
use indoc::indoc;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use tempfile::tempdir;

fn command_in_tmpdir() -> Result<(assert_cmd::Command, tempfile::TempDir), Box<dyn Error>> {
    let tmp = tempdir()?;

    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!())?;

    cmd.current_dir(tmp.path());

    Ok((cmd, tmp))
}

#[test]
fn test_no_yaml() -> Result<(), Box<dyn Error>> {
    // GIVEN
    let (mut cmd, _tmp) = command_in_tmpdir()?;

    // WHEN
    let assert = cmd.assert();

    // THEN
    assert
        .success()
        .stderr("")
        .stdout("No test case found. Exiting.\n");
    Ok(())
}

#[test]
fn test_empty_yaml() -> Result<(), Box<dyn Error>> {
    // GIVEN
    let (mut cmd, tmp) = command_in_tmpdir()?;

    File::create(tmp.path().join("test.yaml"))?;

    // WHEN
    let assert = cmd.assert();

    // THEN
    assert
        .success()
        .stderr("")
        .stdout("Running tests for test\n");
    Ok(())
}

#[test]
fn test_non_yaml_are_ignored() -> Result<(), Box<dyn Error>> {
    // GIVEN
    let (mut cmd, tmp) = command_in_tmpdir()?;

    File::create(tmp.path().join("foo.bar"))?;

    // WHEN
    let assert = cmd.assert();

    // THEN
    assert
        .success()
        .stderr("")
        .stdout("No test case found. Exiting.\n");
    Ok(())
}

#[test]
fn test_subdirs_are_searched() -> Result<(), Box<dyn Error>> {
    // GIVEN
    let (mut cmd, tmp) = command_in_tmpdir()?;

    File::create(tmp.path().join("root.yaml"))?;
    fs::create_dir(tmp.path().join("subdir"))?;
    File::create(tmp.path().join("subdir/subdir.yaml"))?;

    // WHEN
    let assert = cmd.assert();

    // THEN
    assert.success().stderr("").stdout(indoc! {r#"
            Running tests for root
            Running tests for subdir/subdir
            "#});
    Ok(())
}

#[test]
fn test_search_in_specific_directory() -> Result<(), Box<dyn Error>> {
    // GIVEN
    let (mut cmd, tmp) = command_in_tmpdir()?;

    File::create(tmp.path().join("root.yaml"))?;
    fs::create_dir(tmp.path().join("subdir"))?;
    File::create(tmp.path().join("subdir/subdir.yaml"))?;

    cmd.arg("subdir");

    // WHEN
    let assert = cmd.assert();

    // THEN
    assert
        .success()
        .stderr("")
        .stdout("Running tests for subdir\n");
    Ok(())
}

#[test]
fn test_trivial_yaml() -> Result<(), Box<dyn Error>> {
    // GIVEN
    let (mut cmd, tmp) = command_in_tmpdir()?;

    let mut file = File::create(tmp.path().join("foo.yaml"))?;
    file.write_all(
        indoc! {r#"
        name: trivial1
        cmd: echo -n
        ---
        name: trivial2
        cmd: echo -n
    "#}
        .as_bytes(),
    )?;
    // WHEN
    let assert = cmd.assert();

    // THEN
    assert.success().stderr("").stdout(indoc! {r#"
            Running tests for foo
            test foo::trivial1 ... OK
            test foo::trivial2 ... OK
         "#});
    Ok(())
}

#[test]
fn test_failing_testcase() -> Result<(), Box<dyn Error>> {
    // GIVEN
    let (mut cmd, tmp) = command_in_tmpdir()?;

    let mut file = File::create(tmp.path().join("foo.yaml"))?;
    file.write_all(
        indoc! {r#"
        name: failure
        cmd: echo foo
    "#}
        .as_bytes(),
    )?;
    // WHEN
    let assert = cmd.assert();

    // THEN
    assert.success().stderr("").stdout(indoc! {r#"
            Running tests for foo
            test foo::failure ... FAILED

            failures:

            ---- foo::failure ----
            stdout differs:
            --- expected
            +++ actual
            +foo

         "#});
    Ok(())
}

#[test]
fn test_stdin() -> Result<(), Box<dyn Error>> {
    // GIVEN
    let (mut cmd, tmp) = command_in_tmpdir()?;

    let mut file = File::create(tmp.path().join("foo.yaml"))?;
    file.write_all(
        indoc! {r#"
        name: cat should work
        cmd: cat
        stdin: |
          this is a line
        stdout: |
          this is a line
    "#}
        .as_bytes(),
    )?;
    // WHEN
    let assert = cmd.assert();

    // THEN
    assert.success().stderr("").stdout(indoc! {r#"
            Running tests for foo
            test foo::cat should work ... OK
         "#});
    Ok(())
}
