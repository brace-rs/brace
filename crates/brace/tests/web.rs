#![cfg(all(feature = "cli", feature = "web"))]

use std::error::Error;
use std::process::Command;
use std::str::from_utf8;
use std::thread::sleep;
use std::time::Duration;

use assert_cmd::prelude::*;
use awc::Client;
use predicates::prelude::*;

#[actix_rt::test]
async fn test_web_cli() -> Result<(), Box<dyn Error>> {
    let mut cmd = Command::cargo_bin("brace")?;

    cmd.args(&["web", "start", "--host", "127.0.0.1", "--port", "65080"]);

    let mut process = cmd.spawn()?;

    sleep(Duration::from_millis(1000));

    let client = Client::default();
    let mut res = client.get("http://127.0.0.1:65080").send().await.unwrap();

    assert_eq!(res.status(), 200);

    let body = res.body().await.unwrap();
    let text = from_utf8(&body)?;

    assert!(predicate::str::contains("brace").eval(text));

    process.kill()?;

    Ok(())
}
