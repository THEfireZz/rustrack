use assert_cmd::cargo::*;
use assert_fs::prelude::FileWriteStr;
use predicates::prelude::*;

#[test]
fn missing_second_argument() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("rustrack");

    cmd.arg("foo");
    cmd.assert().failure().stderr(predicate::str::contains(
        "the following required arguments were not provided:\n  <TARGET_APTH>",
    ));

    Ok(())
}

#[test]
fn missing_both_arguments() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("rustrack");

    cmd.assert().failure().stderr(predicate::str::contains(
        "the following required arguments were not provided:\n  <SOURCE_PATH>\n  <TARGET_APTH>",
    ));

    Ok(())
}
#[test]
fn source_file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let target = assert_fs::NamedTempFile::new("target.txt")?;

    let mut cmd = cargo_bin_cmd!("rustrack");

    cmd.arg("file/missing/").arg(target.path());
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[test]
fn target_file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let source = assert_fs::NamedTempFile::new("source.txt")?;

    let mut cmd = cargo_bin_cmd!("rustrack");

    cmd.arg(source.path()).arg("file/missing/");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[test]
fn both_files_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = cargo_bin_cmd!("rustrack");

    cmd.arg("file/missing/").arg("file/missing/");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("could not read file"));

    Ok(())
}

#[test]
fn both_files_exist() -> Result<(), Box<dyn std::error::Error>> {
    let source = assert_fs::NamedTempFile::new("source.txt")?;
    let target = assert_fs::NamedTempFile::new("target.txt")?;
    source.write_str("source")?;
    target.write_str("target")?;

    let mut cmd = cargo_bin_cmd!("rustrack");

    cmd.arg(source.path()).arg(target.path());
    cmd.assert().success();

    Ok(())
}
