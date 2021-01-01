use std::path::Path;

use clap::{App, AppSettings, Arg};

use wyag;

// https://github.com/clap-rs/clap/blob/master/examples/20_subcommands.rs
fn main() {
    let app = App::new("wyag")
        .about("Write your own git")
        .version("0.1")
        .author("Jan-Christoph Klie")
        .subcommand(
            App::new("init")
                .about("Initialize a new, empty repository.")
                .arg(
                    Arg::new("path")
                        .about("Where to create the repository.")
                        .required(true),
                ),
        )
        .setting(AppSettings::ArgRequiredElseHelp);

    let matches = app.get_matches();

    match matches.subcommand() {
        Some(("init", init_matches)) => {
            let raw_path = init_matches.value_of("path").unwrap();
            let path = Path::new(raw_path);
            wyag::GitRepository::init(path).unwrap();
        }
        _ => unreachable!(),
    }
}
