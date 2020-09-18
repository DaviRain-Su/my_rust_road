use assert_cmd::prelude::*; // Add methods on commands
use predicates::prelude::*; // Used for writing assertions
use std::process::Command;  // Run programs
use std::io::{self, Write}; 
use tempfile::NamedTempFile;

#[test]
fn file_doesnt_exist() -> Result<(), Box<dyn std::error::Error>> {
    let mut cmd = Command::cargo_bin("grrs")?;
    
    cmd.arg("main").arg("src/nofile");
    cmd.assert()
        .failure()
        .stderr(predicate::str::contains("No such file or directory"));

        Ok(())
}

#[test]
fn find_content_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("test").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("test\nAnother test"));
    
    
    Ok(())

}

#[test]
fn find_empty_char_in_file() -> Result<(), Box<dyn std::error::Error>> {
    let mut file = NamedTempFile::new()?;
    writeln!(file, "A test\nActual content\nMore content\nAnother test")?;

    let mut cmd = Command::cargo_bin("grrs")?;
    cmd.arg("").arg(file.path());
    cmd.assert()
        .success()
        .stdout(predicates::str::contains("A test\nActual content\nMore content\nAnother test"));
    
    
    Ok(())

}
