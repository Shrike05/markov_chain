use crate::markov::MarkovChain;
use bevy::prelude::*;
use bevy_ratatui::RatatuiPlugins;
use clap::Parser;
use std::path::PathBuf;

mod markov;
mod sim;
mod tui;

fn main() {
    let args = Args::parse();

    if !args.path.exists() {
        eprintln!("Error: The file at {:?} does not exist!", args.path);
        return;
    }

    let content = std::fs::read_to_string(&args.path).expect("Failed to read the file");

    let mc = MarkovChain::from_text(content);

    let mut app = App::new();

    app.insert_resource(mc);
    app.add_plugins((DefaultPlugins, RatatuiPlugins::default()));

    app.run();
}

#[derive(Parser)]
#[command(author, version, about = "A simple file reader")]
struct Args {
    /// The path to the file to read
    #[arg(short, long, value_name = "FILE")]
    path: PathBuf,
}
