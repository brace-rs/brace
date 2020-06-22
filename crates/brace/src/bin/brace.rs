use brace::util::{hook, FutureResult};
use clap::{crate_version, App, AppSettings, ArgMatches};

mod web;

#[hook]
fn subcommand() -> App<'static> {}

#[hook]
fn subcommand_matched(name: &str, args: &ArgMatches) -> FutureResult<'static, (), anyhow::Error> {}

#[actix_rt::main]
async fn main() -> anyhow::Result<()> {
    let app = App::new("brace")
        .about("The brace application framework")
        .version(crate_version!())
        .setting(AppSettings::SubcommandRequired)
        .subcommands(hook::invoke(subcommand::with()));

    if let (name, Some(args)) = app.get_matches().subcommand() {
        for res in hook::invoke(subcommand_matched::with(name, args)) {
            res.await?;
        }
    }

    Ok(())
}
