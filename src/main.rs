#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate serde_json;

extern crate clap;
extern crate console;
extern crate indicatif;
extern crate notify;
extern crate regex;

use clap::{crate_version, App, Arg, SubCommand};

mod add;
mod conf;
mod info;
mod problem;
mod watch;

fn main() {
    // TODO add more error info
    info::info();
    let matches = App::new("leetcode")
        .version(crate_version!())
        .author("whale")
        .about("auto performer exercises unit test and format file by template")
        .subcommand(
            SubCommand::with_name("watch")
                .alias("w")
                .about("Reruns `test` when files were edited"),
        )
        .subcommand(
            SubCommand::with_name("add")
                .alias("a")
                .about("Add a template file by specify id from leetcode")
                .arg(
                    Arg::with_name("id")
                        .short("i")
                        .long("id")
                        .takes_value(true)
                        .required(true)
                        .value_name("ID")
                        .help("Problem id"),
                ),
        )
        .get_matches();
    if let Some(_) = matches.subcommand_matches("watch") {
        watch::watch();
    }

    if let Some(matcher) = matches.subcommand_matches("add") {
        add::run(matcher);
    }
}
