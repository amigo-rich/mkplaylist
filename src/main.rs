use clap::{App, Arg, SubCommand};
use mkplaylist::{error::Error, operation::Operation, run};
use std::path::Path;

fn main() -> Result<(), Error> {
    let matches = App::new("mkplaylist")
        .subcommand(
            SubCommand::with_name("index").arg(
                Arg::with_name("path")
                    .short("p")
                    .long("path")
                    .required(true)
                    .takes_value(true),
            ),
        )
        .subcommand(
            SubCommand::with_name("playlist")
                .arg(
                    Arg::with_name("filter")
                        .short("f")
                        .long("filter")
                        .required(false)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("shuffle")
                        .short("s")
                        .long("shuffle")
                        .required(false)
                        .takes_value(false),
                ),
        )
        .subcommand(
            SubCommand::with_name("rate")
                .arg(
                    Arg::with_name("music_id")
                        .short("m")
                        .long("music_id")
                        .required(true)
                        .takes_value(true),
                )
                .arg(
                    Arg::with_name("rating")
                        .short("r")
                        .long("rating")
                        .required(true)
                        .takes_value(true),
                ),
        )
        .get_matches();
    let operation = if let Some(matches) = matches.subcommand_matches("index") {
        let path = Path::new(matches.value_of("path").unwrap()).to_path_buf();
        Operation::Index(path)
    } else if let Some(matches) = matches.subcommand_matches("playlist") {
        if let Some(filter) = matches.value_of("filter") {
            Operation::PlayList(Some(filter), matches.is_present("shuffle"))
        } else {
            Operation::PlayList(None, matches.is_present("shuffle"))
        }
    } else if let Some(matches) = matches.subcommand_matches("rate") {
        let music_id: i64 = matches.value_of("music_id").unwrap().parse().unwrap();
        let rating: i64 = matches.value_of("rating").unwrap().parse().unwrap();
        Operation::Rate(music_id, rating)
    } else {
        return Err(Error::Usage);
    };
    run(operation)
}
