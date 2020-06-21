use std::net::Ipv4Addr;

use clap::{App, Arg, ArgMatches};

pub fn command<'a>() -> App<'a> {
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
        )
}

pub async fn matched(args: &ArgMatches) -> anyhow::Result<()> {
    let host: Ipv4Addr = args.value_of("host").unwrap_or("127.0.0.1").parse()?;
    let port: u16 = args.value_of("port").unwrap_or("8080").parse()?;

    brace::web::server::start(host, port).await
}
