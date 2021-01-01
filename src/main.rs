use std::path::Path;
use std::{io, io::Write};

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
        .subcommand(
            App::new("cat-file")
                .about("Provide content of repository objects.")
                .arg(
                    Arg::new("object")
                        .about("Hash of the object to display.")
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
        Some(("cat-file", init_matches)) => {
            let object = init_matches.value_of("object").unwrap();
            let cwd = std::env::current_dir().unwrap();
            let path = wyag::find_repository(cwd).unwrap();

            let repository = wyag::GitRepository::from_existing(&path).unwrap();

            let git_object = repository.read_object(object).unwrap();

            match git_object {
                wyag::GitObject::GitBlob { data } => {
                    io::stdout().write(&data).unwrap();
                }
                _ => unimplemented!(),
            }
        }

        _ => unreachable!(),
    }
}
