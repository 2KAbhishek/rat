use assert_cmd::prelude::*;
use assert_fs::prelude::*;
use predicates::prelude::*;
use std::process::Command;

#[test]
fn file_prints_successfully() -> Result<(), Box<dyn std::error::Error>> {
    let file = assert_fs::NamedTempFile::new("test.txt")?;
    file.write_str("A test\nActual content")?;

    let mut cmd = Command::cargo_bin("rat")?;
    cmd.arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("A test\nActual content"));

    Ok(())
}

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("rat")?;

    cmd.arg("doesnt_exist.txt");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("Failed to read file"));

    Ok(())
}
