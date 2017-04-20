extern crate futures;
extern crate tokio_core;
extern crate tokio_process;
extern crate prctl;
extern crate clap;

use futures::Future;

use tokio_core::reactor::Core;

use clap::{Arg, App, SubCommand, AppSettings};

fn daemon() {
    use std::process::Command;
    use tokio_process::CommandExt;

    // don't let children escape to init
    prctl::set_child_subreaper(true).unwrap();

    let mut core = Core::new().unwrap();
}

#[derive(Debug)]
enum Command {
    Run(Vec<String>),
}

fn main() {
    let matches = App::new("rund")
                    .version("0.1")
                    .author("Alexander Polakov <plhk@sdf.org>")
                    .subcommand(SubCommand::with_name("run")
                                .about("run command")
                                .setting(AppSettings::TrailingVarArg)
                                .arg(Arg::with_name("command")
                                     .multiple(true)
                                     .required(true)
                                     .takes_value(true)))
                    .get_matches();
    if let Some(matches) = matches.subcommand_matches("run") {
        if let Some(command) = matches.values_of("command") {
            println!("{:?}", command.collect::<Vec<_>>());
        }
    }
    daemon();
}
