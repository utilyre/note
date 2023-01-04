use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use chrono::{DateTime, Local};
use chrono_humanize::HumanTime;
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

            let dt = Local::now();
            writeln!(notes_file, "{},{}", dt.to_rfc3339(), add_args.task).unwrap();
        }

        Action::Ls => {
            let content = fs::read_to_string(notes_path).unwrap();
            for (i, line) in content.lines().into_iter().enumerate() {
                let (rfc3339, task) = line.split_once(",").unwrap();

                let dt = DateTime::parse_from_rfc3339(rfc3339).unwrap();
                let ht = HumanTime::from(dt);

                println!("{}. {} ({})", i + 1, task, ht);
            }
        }
    }
}
