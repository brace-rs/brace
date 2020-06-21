use clap::{crate_version, App};

fn main() {
    App::new("brace")
        .about("The brace application framework")
        .version(crate_version!())
        .get_matches();
}
