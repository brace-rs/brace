use std::process::exit;

use clap::{crate_version, App, AppSettings};

mod web;

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    let mut app = App::new("brace")
        .about("The brace application framework")
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequired);

    #[cfg(feature = "web")]
    {
        app = app.subcommand(self::web::command());
    }

    let matches = app.get_matches();
    let subcommand = matches.subcommand();

    #[cfg(feature = "web")]
    {
        if let ("web", Some(args)) = subcommand {
            return self::web::matched(args).await;
        }
    }

    exit(1);
}
