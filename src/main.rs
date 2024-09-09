use std::io::{self, Write};
use clap::{Parser, Subcommand};

mod commands;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand, Debug)]
enum Commands {
    New { project: String },
    Build,
    Run,
    Test,
}

fn main() {
    let args = Args::parse();

    match args.command {
        Some(Commands::New { project }) => commands::create_project(project.as_str()),
        Some(Commands::Build) => commands::build_project(),
        Some(Commands::Run) => commands::run_project(),
        Some(Commands::Test) => commands::test_project(),

        None => eprintln!("Unknown command entered"),
    };
}
