use base64::{Engine, engine::general_purpose::STANDARD};
use clap::{Parser, Subcommand};
use crypto::helper::gen_key;
use keyring::Entry;
use std::error::Error;
use std::io::{self, Write};
mod db;
mod store;

#[derive(Parser)]
#[clap(
    name = "Harbor",
    version,
    about = "An open source secrets management and distribution platform"
)]
#[command(disable_version_flag = true)]
pub struct Cli {
    #[arg(short = 'v', long, help = "Print version")]
    pub version: bool,

    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Inject {
        #[arg(short, long)]
        verbose: bool,

        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        after: Vec<String>,
    },
    Shell {},
    Add {},
    Setup {},
    List {},
    Project {
        #[command(subcommand)]
        command: ProjectCommands,
    },
    Config {
        #[command(subcommand)]
        command: ConfigCommands,
    },
}

#[derive(Subcommand)]
pub enum ProjectCommands {
    List {},
    Create {},
    Delete { name: String },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    List {},
    Create { project: String },
    Delete { project: String, name: String },
}

pub mod interactions {
    use super::db::models::{NewProject, Project};
    use super::db::{
        establish_connection,
        schema::projects::dsl::{name as project_name, projects},
    };
    use diesel::result::{DatabaseErrorKind, Error as DieselError};
    use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
    use std::error::Error;
    use uuid::Uuid;

    fn construct_error(message: &str) -> Result<(), Box<dyn Error>> {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            message.to_string(),
        )))
    }

    pub fn create_project(proj_name: &str) -> Result<(), Box<dyn Error>> {
        let mut conn = establish_connection();

        // 1. Check if the project already exists
        let existing_project = projects
            .filter(project_name.eq(proj_name))
            .select(Project::as_select())
            .first::<Project>(&mut conn)
            .optional()?;

        if let Some(_) = existing_project {
            return construct_error("Project already exists");
        }

        let project_id = Uuid::new_v4().to_string();
        let new_project = NewProject {
            id: &project_id,
            name: proj_name,
        };

        match diesel::insert_into(projects)
            .values(&new_project)
            .execute(&mut conn)
        {
            Ok(_) => Ok(()),
            Err(DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
                construct_error("Project already exists")
            }
            Err(err) => Err(Box::new(err)),
        }
    }
}

pub fn gen_or_get_key() -> Result<crypto::Key, Box<dyn Error>> {
    let entry = Entry::new("harbor", "encryption-key")?;

    match entry.get_password() {
        Ok(encoded) => {
            let bytes = STANDARD.decode(encoded)?;

            let key = crypto::helper::key_from(bytes)?;

            Ok(key)
        }

        Err(keyring::Error::NoEntry) => {
            let key = gen_key();

            let encoded = STANDARD.encode(key);
            entry.set_password(&encoded)?;

            Ok(key)
        }

        Err(err) => Err(Box::new(err)),
    }
}

pub fn get_input(message: &str, prompt: char, new_line: bool) -> Result<String, Box<dyn Error>> {
    let mut input = String::new();

    if new_line {
        print!("{}\n{} ", message, prompt);
    } else {
        print!("{}{} ", message, prompt);
    }

    io::stdout().flush()?;
    io::stdin().read_line(&mut input)?;
    Ok(input.trim().to_string())
}
