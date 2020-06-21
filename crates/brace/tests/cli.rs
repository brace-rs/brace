#![cfg(feature = "cli")]

use std::error::Error;
use std::process::Command;

use assert_cmd::prelude::*;
use predicates::prelude::*;

#[test]
fn test_cli() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("brace")?;

    cmd.arg("--version");
    cmd.assert()
        .success()
        .stdout(predicate::str::contains("brace"));

    Ok(())
}
