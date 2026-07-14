use std::collections::HashMap;
use std::process;

use clap::Parser;
use clap::crate_version;
use cli::Cli;
use cli::Environment;
use cli::config::{Config, ConfigError};
use cli::gen_or_get_key;
use cli::get_input;
use cli::interactions::delete_secret;
use cli::interactions::get_project_id;
use cli::interactions::get_project_secrets;
use cli::interactions::get_projects;
use cli::interactions::secret_exists;
use cli::interactions::set_secret;
use colored::*;
use crypto::encrypt;

const GIT_VERSION: &str = env!("GIT_VERSION");

fn print_error(message: impl std::fmt::Display) {
    eprintln!("{}", message.to_string().bright_red().bold());
}

fn main() {
    use cli::{Commands, ProjectCommands};
    let cli = Cli::parse();

    if cli.version {
        let top_msg = format!("Harbor version {}", crate_version!().green())
            .bright_cyan()
            .bold();
        let bottom_msg =
            format!("An open source secrets management and distribution platform.").blue();
        let commit_msg = format!("Git commit {}", GIT_VERSION).dimmed();

        println!("{}\n{}\n{}", top_msg, bottom_msg, commit_msg);
        process::exit(0);
    }

    let root = match std::env::current_dir() {
        Ok(path) => path,
        Err(err) => {
            print_error(format!("Error resolving current directory: {}", err));
            process::exit(1);
        }
    };

    let config_path = root.join(".harbor.toml");
    let has_config = config_path.exists();

    match cli.command {
        Some(c) => match c {
            _ if requires_config(&c) && !has_config => {
                print_error("Missing .harbor.toml. Run `harbor config create` first.");
                process::exit(1);
            }
            Commands::Show { environment, key } => {
                let config = require_config(&root);
                let environment: Environment = match environment {
                    Some(env) => match env.parse::<Environment>() {
                        Ok(parsed) => parsed,
                        Err(_) => {
                            print_error(format!("Invalid environment '{}'.", env));
                            process::exit(1);
                        }
                    },
                    None => config.default_env.into(),
                };

                let project = match get_project_id(&config.name) {
                    Ok(p) => p,
                    Err(e) => {
                        print_error(format!(
                            "Unable to get project ID for '{}': {}",
                            config.name, e
                        ));
                        process::exit(1);
                    }
                };

                let secrets = match get_project_secrets(&project, environment) {
                    Ok(s) => s,
                    Err(e) => {
                        print_error(format!("Error getting secrets: {}", e));
                        process::exit(1);
                    }
                };

                let secret = secrets.into_iter().find(|(name, _, _)| name == &key);

                match secret {
                    Some((_, ciphertext, nonce)) => {
                        let key = match gen_or_get_key() {
                            Ok(k) => k,
                            Err(_) => {
                                print_error("Error generating or getting key");
                                process::exit(1);
                            }
                        };
                        let nonce = crypto::Nonce::from_slice(&nonce);
                        let decrypted = match crypto::decrypt(&key, ciphertext, nonce) {
                            Ok(d) => d,
                            Err(e) => {
                                print_error(format!("Error decrypting secret: {}", e));
                                process::exit(1);
                            }
                        };

                        let decrypted_str = match String::from_utf8(decrypted) {
                            Ok(s) => s,
                            Err(e) => {
                                print_error(format!(
                                    "Error converting decrypted secret to string: {}",
                                    e
                                ));
                                process::exit(1);
                            }
                        };

                        println!("{}", decrypted_str);
                    }
                    None => {
                        print_error(format!(
                            "Secret with key '{}' not found in project '{}'.",
                            key, config.name
                        ));
                        process::exit(1);
                    }
                }
            }
            Commands::Set { environment, vars } => {
                let config = require_config(&root);
                let environment: Environment = match environment {
                    Some(env) => match env.parse::<Environment>() {
                        Ok(parsed) => parsed,
                        Err(_) => {
                            print_error(format!("Invalid environment '{}'.", env));
                            process::exit(1);
                        }
                    },
                    None => config.default_env.into(),
                };

                let pairs = match cli::parse_secret_pairs(&vars) {
                    Ok(pairs) => pairs,
                    Err(e) => {
                        print_error(format!("Error parsing secrets: {}", e));
                        process::exit(1);
                    }
                };

                if pairs.is_empty() {
                    print_error("No secrets provided. Use KEY=VALUE pairs.");
                    process::exit(1);
                }

                let project = match get_project_id(&config.name) {
                    Ok(p) => p,
                    Err(e) => {
                        print_error(format!(
                            "Unable to get project ID for '{}': {}",
                            config.name, e
                        ));
                        process::exit(1);
                    }
                };

                for pair in pairs {
                    let existed = match secret_exists(&project, &pair.0, environment) {
                        Ok(exists) => exists,
                        Err(e) => {
                            print_error(format!(
                                "Error checking existing secret for key '{}': {}",
                                pair.0, e
                            ));
                            process::exit(1);
                        }
                    };
                    if existed {
                        let confirmation = match get_input(
                            format!(
                                "Secret '{}' already exists. Overwrite? (y/N)",
                                pair.0
                            )
                            .yellow(),
                            ':',
                            false,
                        ) {
                            Ok(input) => input,
                            Err(e) => {
                                print_error(format!("Error getting confirmation: {}", e));
                                process::exit(1);
                            }
                        };

                        if confirmation.to_lowercase() != "y" {
                            println!(
                                "{}",
                                format!(
                                    "Skipped secret '{}' in {}.",
                                    pair.0,
                                    environment.as_str()
                                )
                                .dimmed()
                            );
                            continue;
                        }
                    }

                    let key = match gen_or_get_key() {
                        Ok(k) => k,
                        Err(_) => {
                            print_error("Error generating or getting key");
                            process::exit(1);
                        }
                    };
                    let (nonce, encrypted) = match encrypt(&key, pair.1.as_bytes().to_vec()) {
                        Ok(result) => result,
                        Err(e) => {
                            print_error(format!(
                                "Error encrypting secret for key '{}': {}",
                                pair.0, e
                            ));
                            process::exit(1);
                        }
                    };
                    match set_secret(&project, &pair.0, encrypted, environment, nonce) {
                        Ok(()) => {
                            let verb = if existed { "Updated" } else { "Set" };
                            println!(
                                "{}",
                                format!(
                                    "{} secret '{}' in {}.",
                                    verb,
                                    pair.0,
                                    environment.as_str()
                                )
                                .bright_green()
                                .bold()
                            );
                        }
                        Err(e) => {
                            print_error(format!(
                                "Error setting secret for key '{}': {}",
                                pair.0, e
                            ));
                            process::exit(1);
                        }
                    }
                }
            }
            Commands::Delete { environment, keys } => {
                let config = require_config(&root);
                let environment: Environment = match environment {
                    Some(env) => match env.parse::<Environment>() {
                        Ok(parsed) => parsed,
                        Err(_) => {
                            print_error(format!("Invalid environment '{}'.", env));
                            process::exit(1);
                        }
                    },
                    None => config.default_env.into(),
                };
                let proj_id = match get_project_id(&config.name) {
                    Ok(p) => p,
                    Err(e) => {
                        print_error(format!(
                            "Unable to get project ID for '{}': {}",
                            config.name, e
                        ));
                        process::exit(1);
                    }
                };
                let mut cleaned = Vec::new();

                for key in keys {
                    let trimmed = key.trim();
                    if !trimmed.is_empty() {
                        cleaned.push(trimmed.to_string());
                    }
                }

                // Redefining as immutable
                let cleaned = cleaned;

                if cleaned.is_empty() {
                    print_error("No secret keys provided. Use one or more keys.");
                    process::exit(1);
                }

                for key in cleaned {
                    match delete_secret(&proj_id, &key, environment) {
                        Ok(()) => {
                            println!("Deleted secret for key '{}'", key);
                        }
                        Err(e) => {
                            print_error(format!("Error deleting secret for key '{}': {}", key, e));
                            process::exit(1);
                        }
                    };
                }
            }
            Commands::Inject { environment, after } => {
                let config = require_config(&root);
                let environment: Environment = match environment {
                    Some(env) => match env.parse::<Environment>() {
                        Ok(parsed) => parsed,
                        Err(_) => {
                            print_error(format!("Invalid environment '{}'.", env));
                            process::exit(1);
                        }
                    },
                    None => config.default_env.into(),
                };
                let proj_id = match get_project_id(&config.name) {
                    Ok(p) => p,
                    Err(e) => {
                        print_error(format!(
                            "Unable to get project ID for '{}': {}",
                            config.name, e
                        ));
                        process::exit(1);
                    }
                };

                // for harbor inject -- bun dev
                // after = ["bun", "dev"]

                let mut cmd = process::Command::new(&after[0]);

                if after.len() > 1 {
                    cmd.args(&after[1..]);
                }

                let secrets = match get_project_secrets(&proj_id, environment) {
                    Ok(s) => s,
                    Err(e) => {
                        print_error(format!("Error getting secrets: {}", e));
                        process::exit(1);
                    }
                };

                let mut uncrypted_envs: HashMap<String, String> = HashMap::new();

                for (name, ciphertext, nonce) in secrets {
                    let key = match gen_or_get_key() {
                        Ok(k) => k,
                        Err(_) => {
                            print_error("Error generating or getting key");
                            process::exit(1);
                        }
                    };
                    let nonce = crypto::Nonce::from_slice(&nonce);
                    let decrypted = match crypto::decrypt(&key, ciphertext, nonce) {
                        Ok(d) => d,
                        Err(e) => {
                            print_error(format!(
                                "Error decrypting secret for key '{}': {}",
                                name, e
                            ));
                            process::exit(1);
                        }
                    };

                    let decrypted_str = match String::from_utf8(decrypted) {
                        Ok(s) => s,
                        Err(e) => {
                            print_error(format!(
                                "Error converting decrypted secret to string for key '{}': {}",
                                name, e
                            ));
                            process::exit(1);
                        }
                    };

                    uncrypted_envs.insert(name, decrypted_str);
                }

                for (key, value) in uncrypted_envs {
                    cmd.env(key, value);
                }

                if cmd.status().is_err() {
                    print_error(format!("Error waiting for command to finish: {:?}", after));
                    process::exit(1);
                } else {
                    process::exit(0);
                }
            }
            Commands::List {} => {
                // Get all secrets, then use colorize and split them between environments, and print them out in a table format.
                let config = require_config(&root);
                let proj_id = match get_project_id(&config.name) {
                    Ok(p) => p,
                    Err(e) => {
                        print_error(format!(
                            "Unable to get project ID for '{}': {}",
                            config.name, e
                        ));
                        process::exit(1);
                    }
                };

                let secrets = match get_project_secrets(&proj_id, Environment::Dev) {
                    Ok(s) => s,
                    Err(e) => {
                        print_error(format!("Error getting secrets: {}", e));
                        process::exit(1);
                    }
                };

                if secrets.is_empty() {
                    println!("No secrets found for project '{}'.", config.name);
                    process::exit(0);
                }

                println!(
                    "{}",
                    format!("Secrets for project '{}':", config.name)
                        .bright_cyan()
                        .bold()
                );

                for (name, _, _) in secrets {
                    println!(" - {}", name.blue().bold());
                }
            }
            Commands::Project { command } => match command {
                ProjectCommands::List {} => {
                    let projects = match get_projects() {
                        Ok(projects) => projects,
                        Err(e) => {
                            print_error(format!("Error getting projects: {}", e));
                            process::exit(1);
                        }
                    };
                    for project in projects {
                        println!(" - {}", project.name.blue().bold());
                    }
                }
                ProjectCommands::Create {} => {
                    let project_name = match get_input("Project name", ':', false) {
                        Ok(name) => name,
                        Err(e) => {
                            print_error(format!("Error getting project name: {}", e));
                            process::exit(1);
                        }
                    };

                    match cli::interactions::create_project(&project_name) {
                        Ok(()) => {
                            println!("{}", "Project created successfully.".bright_green().bold())
                        }
                        Err(e) => {
                            print_error(format!("Error creating project: {}", e));
                            process::exit(1);
                        }
                    }
                }
                ProjectCommands::Delete { name } => {
                    // We'll check if the project exists before prompting for confirmation
                    let exists = match cli::interactions::project_exists(&name) {
                        Ok(exists) => exists,
                        Err(e) => {
                            print_error(format!("Error checking if project exists: {}", e));
                            process::exit(1);
                        }
                    };

                    if !exists {
                        print_error(format!("Project '{}' does not exist.", name));
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
                            print_error(format!("Error getting confirmation: {}", e));
                            process::exit(1);
                        }
                    };

                    if confirmation.to_lowercase() == "y" {
                        match cli::interactions::delete_project(&name) {
                            Ok(()) => {
                                println!(
                                    "{}",
                                    "Project deleted successfully.".bright_green().bold()
                                )
                            }
                            Err(e) => {
                                print_error(format!("Error deleting project: {}", e));
                                process::exit(1);
                            }
                        }
                    } else {
                        println!("Project deletion canceled.");
                    }
                }
            },
            Commands::Shell {
                environment,
                shell,
                command,
            } => {
                let config = require_config(&root);
                let environment: Environment = match environment {
                    Some(env) => match env.parse::<Environment>() {
                        Ok(parsed) => parsed,
                        Err(_) => {
                            print_error(format!("Invalid environment '{}'.", env));
                            process::exit(1);
                        }
                    },
                    None => config.default_env.into(),
                };
                let proj_id = match get_project_id(&config.name) {
                    Ok(p) => p,
                    Err(e) => {
                        print_error(format!(
                            "Unable to get project ID for '{}': {}",
                            config.name, e
                        ));
                        process::exit(1);
                    }
                };

                // for harbor inject -- bun dev
                // after = ["bun", "dev"]

                let shell = shell
                    .or_else(|| std::env::var("SHELL").ok())
                    .or_else(|| Some("/bin/sh".into()))
                    .unwrap();

                let mut cmd = process::Command::new(&shell);

                if let Some(command) = command {
                    cmd.arg("-c");
                    cmd.arg(command);
                }

                let secrets = match get_project_secrets(&proj_id, environment) {
                    Ok(s) => s,
                    Err(e) => {
                        print_error(format!("Error getting secrets: {}", e));
                        process::exit(1);
                    }
                };

                let mut uncrypted_envs: HashMap<String, String> = HashMap::new();

                for (name, ciphertext, nonce) in secrets {
                    let key = match gen_or_get_key() {
                        Ok(k) => k,
                        Err(_) => {
                            print_error("Error generating or getting key");
                            process::exit(1);
                        }
                    };
                    let nonce = crypto::Nonce::from_slice(&nonce);
                    let decrypted = match crypto::decrypt(&key, ciphertext, nonce) {
                        Ok(d) => d,
                        Err(e) => {
                            print_error(format!(
                                "Error decrypting secret for key '{}': {}",
                                name, e
                            ));
                            process::exit(1);
                        }
                    };

                    let decrypted_str = match String::from_utf8(decrypted) {
                        Ok(s) => s,
                        Err(e) => {
                            print_error(format!(
                                "Error converting decrypted secret to string for key '{}': {}",
                                name, e
                            ));
                            process::exit(1);
                        }
                    };

                    uncrypted_envs.insert(name, decrypted_str);
                }

                for (key, value) in uncrypted_envs {
                    cmd.env(key, value);
                }

                let mut child = match cmd.spawn() {
                    Ok(c) => c,
                    Err(_) => {
                        print_error(format!("Error executing command: {:?}", &shell));
                        process::exit(1);
                    }
                };

                if child.wait().is_err() {
                    print_error(format!("Error waiting for command to finish: {:?}", &shell));
                    process::exit(1);
                } else {
                    process::exit(0);
                }
            }
            Commands::Setup {} => {
                use std::process;
                // Setup will get all projects and prompt the user to select one,
                // then create a .harbor.toml file in the current directory with the selected project name.
                let projects = match get_projects() {
                    Ok(projects) => projects,
                    Err(e) => {
                        print_error(format!("Error getting projects: {}", e));
                        process::exit(1);
                    }
                };

                if projects.is_empty() {
                    print_error("No projects found. Please create a project first.");
                    process::exit(1);
                }

                // We'll just use fzf, and pipe the project names to it, and get the selected project name back.
                let project_names: Vec<String> = projects.iter().map(|p| p.name.clone()).collect();

                let mut fzf = match process::Command::new("fzf")
                    .stdin(process::Stdio::piped())
                    .stdout(process::Stdio::piped())
                    .spawn()
                {
                    Ok(c) => c,
                    Err(e) => {
                        print_error(format!("Error spawning fzf: {}", e));
                        process::exit(1);
                    }
                };

                {
                    let mut stdin = fzf.stdin.take().expect("Failed to open stdin");
                    for name in &project_names {
                        use std::io::Write;
                        writeln!(stdin, "{}", name).expect("Failed to write to stdin");
                    }
                    drop(stdin);
                }

                let output = match fzf.wait_with_output() {
                    Ok(o) => o,
                    Err(e) => {
                        print_error(format!("Error waiting for fzf: {}", e));
                        process::exit(1);
                    }
                };

                if !output.status.success() {
                    print_error("fzf exited with non-zero status");
                    process::exit(1);
                }

                let project = String::from_utf8_lossy(&output.stdout).trim().to_string();

                if project.is_empty() {
                    print_error("No project selected.");
                    process::exit(1);
                }

                let new_config = format!(
                    r#"version = "1"
name = "{}"
config = "dev""#,
                    project
                );

                let config_path = root.join(".harbor.toml");
                match std::fs::write(&config_path, new_config) {
                    Ok(_) => {
                        println!(
                            "{}",
                            format!("Created .harbor.toml for project '{}'.", project)
                                .bright_green()
                                .bold()
                        );
                    }
                    Err(e) => {
                        print_error(format!(
                            "Error creating .harbor.toml for project '{}': {}",
                            project, e
                        ));
                        process::exit(1);
                    }
                }
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

fn require_config(root: &std::path::Path) -> Config {
    match Config::from_repo_root(root) {
        Ok(config) => config,
        Err(ConfigError::Io(err)) if err.kind() == std::io::ErrorKind::NotFound => {
            print_error("Missing .harbor.toml. Run `harbor config create` first.");
            process::exit(1);
        }
        Err(err) => {
            print_error(format!("Error reading config: {}", err));
            process::exit(1);
        }
    }
}

fn requires_config(command: &cli::Commands) -> bool {
    matches!(
        command,
        cli::Commands::Inject { .. }
            | cli::Commands::Show { .. }
            | cli::Commands::Set { .. }
            | cli::Commands::Delete { .. }
            | cli::Commands::Shell { .. }
            | cli::Commands::List { .. }
    )
}
