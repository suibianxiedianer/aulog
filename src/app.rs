use std::env;

use clap::{Arg, ArgAction, Command};

pub fn app() -> Command {
    Command::new(clap::crate_name!())
        .author(clap::crate_authors!())
        .about(clap::crate_description!())
        .version(clap::crate_version!())
        .arg(
            Arg::new("list")
                .long("list")
                .short('l')
                .action(ArgAction::SetTrue)
                .help("list all audit log files")
        )
        .arg(
            Arg::new("show")
                .long("show")
                .short('s')
                .action(ArgAction::Set)
                .help("display the targeted audit log file, in less mode")
        )
        .arg(
            Arg::new("tail")
                .long("tail")
                .short('t')
                .action(ArgAction::SetTrue)
                .help("output appended data as the audit.log grows")
       )
}
