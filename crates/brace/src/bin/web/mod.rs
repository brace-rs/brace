#![cfg(feature = "web")]

use std::process::exit;

use clap::{App, AppSettings, ArgMatches};

pub mod start;

pub fn command<'a>() -> App<'a> {
    App::new("web")
        .about("The brace web server")
        .subcommand(self::start::command())
        .setting(AppSettings::SubcommandRequired)
}

pub async fn matched(matches: &ArgMatches) -> anyhow::Result<()> {
    let subcommand = matches.subcommand();

    if let ("start", Some(args)) = subcommand {
        return self::start::matched(args).await;
    }

    exit(1);
}
