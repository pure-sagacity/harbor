use std::process;

use clap::Parser;
use clap::crate_version;
use cli::Cli;
use colored::*;

const GIT_VERSION: &str = env!("GIT_VERSION");

fn main() {
    let cli = Cli::parse();

    if cli.version {
        let top_msg = format!("Harbor version {}", crate_version!().green()).bright_blue();
        let bottom_msg =
            format!("An open source secrets management and distribution platform.").blue();
        let commit_msg = format!("Git commit {}", GIT_VERSION).dimmed();

        println!("{}\n{}\n{}", top_msg, bottom_msg, commit_msg);
        process::exit(0);
    }

    if cli.command.is_none() {
        println!(
            "{}\n{}",
            "No command was entered.".red(),
            "Exiting...".dimmed()
        );
        process::exit(0);
    }
}
