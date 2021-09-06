use std::fs;
use std::fs::File;
use std::error::Error;
use assert_cmd::Command;
use tempfile::tempdir;

fn command_in_tmpdir() -> Result<(assert_cmd::Command, tempfile::TempDir), Box<dyn Error>> {
    let tmp = tempdir()?;

    let mut cmd = Command::cargo_bin(assert_cmd::crate_name!())?;

    cmd.current_dir(tmp.path());

    Ok((cmd, tmp))
}


#[test]
fn test_no_yaml() -> Result<(), Box<dyn Error>>{
    // GIVEN
    let (mut cmd, _tmp) = command_in_tmpdir()?;

    // WHEN
    let assert = cmd.assert();

    // THEN
    assert
        .stdout("No test case found. Exiting.\n")
        .success();
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
        .stdout("Running tests for ./test.yaml\n")
        .success();
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
        .stdout("No test case found. Exiting.\n")
        .success();
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
    assert
        .stdout("Running tests for ./root.yaml\n\
                 Running tests for ./subdir/subdir.yaml\n")
        .success();
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
        .stdout("Running tests for subdir/subdir.yaml\n")
        .success();
    Ok(())
}
