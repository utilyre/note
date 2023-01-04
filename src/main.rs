use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use chrono::Local;
use clap::Parser;
use xdg::BaseDirectories;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    action: Action,
}

#[derive(clap::Subcommand)]
enum Action {
    Add(AddArgs),
    Ls,
}

#[derive(Parser)]
struct AddArgs {
    /// Task to be noted
    task: String,
}

fn main() {
    let xdg_dirs = BaseDirectories::with_prefix("note").unwrap();
    let notes_path = xdg_dirs.place_state_file("notes.csv").unwrap();

    let args = Args::parse();
    match args.action {
        Action::Add(add_args) => {
            let mut notes_file = OpenOptions::new()
                .create(true)
                .append(true)
                .open(notes_path)
                .unwrap();

            let datetime = Local::now().format("%Y-%m-%d %H:%M:%S");
            writeln!(notes_file, "{},{}", datetime, add_args.task).unwrap();
        }

        Action::Ls => {
            let contents = fs::read_to_string(notes_path).unwrap();
            println!("{}", contents);
        }
    }
}
