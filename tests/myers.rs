use assert_cmd::cargo::*;
use assert_fs::prelude::FileWriteStr;
use predicates::prelude::*;

#[test]
fn both_file_empty() -> Result<(), Box<dyn std::error::Error>>{
    let source = assert_fs::NamedTempFile::new("source.txt")?;
    let target = assert_fs::NamedTempFile::new("target.txt")?;
    source.write_str("")?;
    target.write_str("")?;

    let mut cmd = cargo_bin_cmd!("rustrack");

    cmd.arg(source.path()).arg(target.path());
    cmd.assert().success().stderr(predicate::str::contains("Files identical"));

    Ok(())
}

#[test]
fn both_file_identical() -> Result<(), Box<dyn std::error::Error>>{
    let source = assert_fs::NamedTempFile::new("source.txt")?;
    let target = assert_fs::NamedTempFile::new("target.txt")?;
    source.write_str("identical")?;
    target.write_str("identical")?;

    let mut cmd = cargo_bin_cmd!("rustrack");

    cmd.arg(source.path()).arg(target.path());
    cmd.assert().success().stderr(predicate::str::contains("Files identical"));

    Ok(())
}

#[test]
fn both_file_not_identical() -> Result<(), Box<dyn std::error::Error>>{
    let source = assert_fs::NamedTempFile::new("source.txt")?;
    let target = assert_fs::NamedTempFile::new("target.txt")?;
    source.write_str("A\nB\nC\nA\nB\nB\nA")?;
    target.write_str("C\nB\nA\nB\nA\nC")?;

    let mut cmd = cargo_bin_cmd!("rustrack");

    cmd.arg(source.path()).arg(target.path());
    cmd.assert().success().stderr(predicate::str::contains("- A\n- B\n  C\n+ B\n  A\n  B\n- B\n  A\n+ C"));

    Ok(())
}