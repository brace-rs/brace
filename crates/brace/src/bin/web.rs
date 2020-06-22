#![cfg(feature = "web")]

use std::net::Ipv4Addr;

use brace::util::{hook, FutureResult};
use clap::{App, AppSettings, Arg, ArgMatches};

#[hook(crate::subcommand)]
pub fn command() -> App<'static> {
    App::new("web")
        .about("The brace web server")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            App::new("start")
                .about("Starts the brace web server")
                .arg(
                    Arg::new("host")
                        .short('h')
                        .long("host")
                        .value_name("string")
                        .about("Sets the host")
                        .takes_value(true),
                )
                .arg(
                    Arg::new("port")
                        .short('p')
                        .long("port")
                        .value_name("integer")
                        .about("Sets the port")
                        .takes_value(true),
                ),
        )
}

#[hook(crate::subcommand_matched)]
pub fn command_matched(
    name: &str,
    matches: &ArgMatches,
) -> FutureResult<'static, (), anyhow::Error> {
    if name == "web" {
        let subcommand = matches.subcommand();

        if let ("start", Some(args)) = subcommand {
            let host = args
                .value_of("host")
                .unwrap_or("127.0.0.1")
                .parse::<Ipv4Addr>();
            let port = args.value_of("port").unwrap_or("8080").parse::<u16>();

            return FutureResult::future(async {
                let host = host?;
                let port = port?;

                brace::web::server::start(host, port).await
            });
        }
    }

    FutureResult::done()
}
