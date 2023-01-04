use std::{fs::OpenOptions, io::Write};

use chrono::Local;
use clap::Parser;
use xdg::BaseDirectories;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    /// Task to be noted
    task: String,
}

fn main() {
    let xdg_dirs = BaseDirectories::with_prefix("note").unwrap();

    let notes_path = xdg_dirs.place_state_file("notes.csv").unwrap();
    // let notes_file = File::create(notes_path);

    let mut notes_file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(notes_path)
        .unwrap();

    // mm/dd/yy hh:mm:ss
    let datetime = Local::now().format("%Y-%m-%d %H:%M:%S");
    let args = Args::parse();

    writeln!(notes_file, "{},{}", datetime, args.task).unwrap();
}
