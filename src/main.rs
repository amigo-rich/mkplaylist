use clap::{App, Arg, SubCommand};
use mkplaylist::{operation::Operation, run};
use std::path::Path;

fn main() {
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
        .get_matches();
    if let Some(matches) = matches.subcommand_matches("index") {
        let path = Path::new(matches.value_of("path").unwrap()).to_path_buf();
        run(Operation::Index(path)).unwrap();
    } else if let Some(matches) = matches.subcommand_matches("playlist") {
        if let Some(filter) = matches.value_of("filter") {
            run(Operation::PlayList(
                Some(filter),
                matches.is_present("shuffle"),
            ))
            .unwrap();
        } else {
            run(Operation::PlayList(None, matches.is_present("shuffle"))).unwrap();
        }
    }
}
