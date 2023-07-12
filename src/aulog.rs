use std::process::Command;

use crate::LOG_PATH;

pub fn run(mut app: clap::Command) {
    let _args = app.clone().get_matches();

    // list all audit log files & exit
    if *_args.get_one::<bool>("list").unwrap() {
        list();
        return
    }

    // cat targeted audit logs & exit
    if let Some(mut log) = _args.get_many::<String>("show") {
        let mut logs = vec![];
        while let Some(_log) = log.next() {
            logs.push(_log.into());
        }
        show(logs);
        return
    }

    // tail -f the newest audit.log & exit
    if *_args.get_one::<bool>("tail").unwrap() {
        tail();
        return
    }

    // show help & exit 1
    let _ = app.print_help();
    std::process::exit(1);
}

fn list() {
    let mut ls = Command::new("ls")
        .arg(LOG_PATH)
        .env("LESSCHARSET", "UTF-8")
        .spawn()
        .expect("Failed to list audit log dir");

    let _ = ls.wait();
}

fn show(logs: Vec<String>) {
    let mut less = Command::new("less")
        .current_dir(LOG_PATH)
        .args(logs)
        .env("LESSCHARSET", "UTF-8")
        .spawn()
        .expect("Failed to run less command");

    let _ = less.wait();
}

fn tail() {
    let mut tail = Command::new("tail")
        .current_dir(LOG_PATH)
        .arg("-f")
        .arg("audit.log")
        .env("LESSCHARSET", "UTF-8")
        .spawn()
        .expect("Failed to run tail command");

    let _ = tail.wait();
}
