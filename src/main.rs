use std::{env, fs::OpenOptions, io::Write, process};

use chrono::Local;
use xdg::BaseDirectories;

fn main() {
    let xdg_dirs = BaseDirectories::with_prefix("note").unwrap();

    let notes_path = xdg_dirs.place_state_file("notes.csv").unwrap();
    // let notes_file = File::create(notes_path);

    let args: Vec<_> = env::args().collect();
    let task = args.get(1).unwrap_or_else(|| {
        eprintln!("no task provided");
        process::exit(2)
    });

    let mut notes_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(notes_path)
        .unwrap();

    // mm/dd/yy hh:mm:ss
    let datetime = Local::now().format("%Y-%m-%d %H:%M:%S");

    writeln!(notes_file, "{},{}", datetime, task).unwrap();
}
