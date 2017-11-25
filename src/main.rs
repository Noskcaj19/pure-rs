extern crate clap;
extern crate colored;
extern crate git2;

use clap::{App, AppSettings, Arg, SubCommand};


mod prompt;

fn main() {
    let matches = App::new("pure-rs")
        .setting(AppSettings::SubcommandRequired)
        .subcommand(
            SubCommand::with_name("prompt").arg(
                Arg::with_name("previous_return_code")
                    .short("r")
                    .takes_value(true),
            ),
        )
        .subcommand(SubCommand::with_name("precmd"))
        .get_matches();

    match matches.subcommand() {
        ("prompt", Some(sub_matches)) => prompt::render(sub_matches),
        _ => (),
    }
}
