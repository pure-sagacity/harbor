use std::process;

use clap::Parser;
use clap::crate_version;
use cli::Cli;
use cli::get_input;
use colored::*;

const GIT_VERSION: &str = env!("GIT_VERSION");

fn main() {
    use cli::{Commands, ConfigCommands, ProjectCommands};
    let cli = Cli::parse();

    if cli.version {
        let top_msg = format!("Harbor version {}", crate_version!().green()).bright_cyan();
        let bottom_msg =
            format!("An open source secrets management and distribution platform.").blue();
        let commit_msg = format!("Git commit {}", GIT_VERSION).dimmed();

        println!("{}\n{}\n{}", top_msg, bottom_msg, commit_msg);
        process::exit(0);
    }

    match cli.command {
        Some(c) => match c {
            Commands::Add {} => {
                println!("Add command executed");
            }
            Commands::Config { command } => match command {
                ConfigCommands::List {} => {
                    println!("Config list command executed");
                }
                ConfigCommands::Create { project } => {
                    println!("Config create command executed for project: {}", project);
                }
                ConfigCommands::Delete { project, name } => {
                    println!(
                        "Config delete command executed for project: {}, name: {}",
                        project, name
                    );
                }
            },
            Commands::Inject { verbose, after } => {
                println!(
                    "Inject command executed with verbose: {}, after: {:?}",
                    verbose, after
                );
                // for harbor inject -- bun dev
                // after = ["bun", "dev"]
            }
            Commands::List {} => {
                println!("List command executed");
            }
            Commands::Project { command } => match command {
                ProjectCommands::List {} => {
                    println!("Project list command executed");
                }
                ProjectCommands::Create {} => {
                    let project_name = match get_input("Project name", ':', false) {
                        Ok(name) => name,
                        Err(e) => {
                            eprintln!("Error getting project name: {}", e);
                            process::exit(1);
                        }
                    };

                    match cli::interactions::create_project(&project_name) {
                        Ok(()) => println!("{}", "Project created successfully.".cyan()),
                        Err(e) => {
                            eprintln!("Error creating project: {}", e);
                            process::exit(1);
                        }
                    }
                }
                ProjectCommands::Delete { name } => {
                    // We'll check if the project exists before prompting for confirmation
                    let exists = match cli::interactions::project_exists(&name) {
                        Ok(exists) => exists,
                        Err(e) => {
                            eprintln!("Error checking if project exists: {}", e);
                            process::exit(1);
                        }
                    };

                    if !exists {
                        eprintln!("Project '{}' does not exist.", name);
                        process::exit(1);
                    }

                    // Prompt for confirmation
                    let confirmation = match get_input(
                        format!(
                            "Are you sure you want to delete the project '{}'? (y/N)",
                            name
                        )
                        .red(),
                        ':',
                        false,
                    ) {
                        Ok(input) => input,
                        Err(e) => {
                            eprintln!("Error getting confirmation: {}", e);
                            process::exit(1);
                        }
                    };

                    if confirmation.to_lowercase() == "y" {
                        match cli::interactions::delete_project(&name) {
                            Ok(()) => {
                                println!("{}", "Project deleted successfully.".cyan())
                            }
                            Err(e) => {
                                eprintln!("Error deleting project: {}", e);
                                process::exit(1);
                            }
                        }
                    } else {
                        println!("Project deletion canceled.");
                    }
                }
            },
            Commands::Shell {} => {
                println!("Shell command executed");
            }
            Commands::Setup {} => {
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
