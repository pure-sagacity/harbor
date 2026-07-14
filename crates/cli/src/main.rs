use std::process;

use clap::Parser;
use clap::crate_version;
use cli::Cli;
use cli::get_input;
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

    match cli.command {
        Some(c) => match c {
            cli::Commands::Add {} => {
                println!("Add command executed");
            }
            cli::Commands::Config { command } => match command {
                cli::ConfigCommands::List {} => {
                    println!("Config list command executed");
                }
                cli::ConfigCommands::Create { project } => {
                    println!("Config create command executed for project: {}", project);
                }
                cli::ConfigCommands::Delete { project, name } => {
                    println!(
                        "Config delete command executed for project: {}, name: {}",
                        project, name
                    );
                }
            },
            cli::Commands::Inject { verbose, after } => {
                println!(
                    "Inject command executed with verbose: {}, after: {:?}",
                    verbose, after
                );
                // for harbor inject -- bun dev
                // after = ["bun", "dev"]
            }
            cli::Commands::List {} => {
                println!("List command executed");
            }
            cli::Commands::Project { command } => match command {
                cli::ProjectCommands::List {} => {
                    println!("Project list command executed");
                }
                cli::ProjectCommands::Create {} => {
                    let project_name = match get_input("Project name", ':', false) {
                        Ok(name) => name,
                        Err(e) => {
                            eprintln!("Error getting project name: {}", e);
                            process::exit(1);
                        }
                    };

                    match cli::interactions::create_project(&project_name) {
                        Ok(()) => println!("{}", "Project created successfully".blue()),
                        Err(e) => {
                            eprintln!("Error creating project: {}", e);
                            process::exit(1);
                        }
                    }
                }
                cli::ProjectCommands::Delete { name } => {
                    println!("Project delete command executed for name: {}", name);
                }
            },
            cli::Commands::Shell {} => {
                println!("Shell command executed");
            }
            cli::Commands::Setup {} => {
                println!("Setup command executed");
            }
        },
        None => {
            println!(
                "{}\n{}",
                "No command was entered.".red(),
                "Use --help for more information.".dimmed()
            );
            process::exit(0);
        }
    }
}
