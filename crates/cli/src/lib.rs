use base64::{Engine, engine::general_purpose::STANDARD};
use clap::{Parser, Subcommand};
use crypto::helper::gen_key;
use diesel::backend::Backend;
use diesel::deserialize::{self, FromSql};
use diesel::serialize::{self, IsNull, Output, ToSql};
use diesel::sql_types::Text;
use diesel::sqlite::Sqlite;
use keyring::Entry;
use serde::Deserialize;
use std::error::Error;
use std::io::{self, Write};
pub mod config;
mod db;
mod store;

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, Deserialize, diesel::AsExpression, diesel::FromSqlRow,
)]
#[diesel(sql_type = Text)]
#[serde(rename_all = "kebab-case")]
pub enum Environment {
    Dev,
    Prod,
    Staging,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct EnvironmentParseError(String);

impl Environment {
    pub const fn as_str(self) -> &'static str {
        match self {
            Environment::Dev => "dev",
            Environment::Prod => "prod",
            Environment::Staging => "staging",
        }
    }
}

impl std::fmt::Display for Environment {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.as_str())
    }
}

impl std::str::FromStr for Environment {
    type Err = EnvironmentParseError;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value.trim().to_ascii_lowercase().as_str() {
            "dev" => Ok(Environment::Dev),
            "prod" => Ok(Environment::Prod),
            "staging" => Ok(Environment::Staging),
            _ => Err(EnvironmentParseError(
                "Environment must be one of: dev, prod, staging".to_string(),
            )),
        }
    }
}

impl std::fmt::Display for EnvironmentParseError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

impl std::error::Error for EnvironmentParseError {}

impl ToSql<Text, Sqlite> for Environment {
    fn to_sql<'b>(&'b self, out: &mut Output<'b, '_, Sqlite>) -> serialize::Result {
        out.set_value(self.as_str());
        Ok(IsNull::No)
    }
}

impl FromSql<Text, Sqlite> for Environment {
    fn from_sql(bytes: <Sqlite as Backend>::RawValue<'_>) -> deserialize::Result<Self> {
        let raw = <String as FromSql<Text, Sqlite>>::from_sql(bytes)?;
        raw.parse::<Environment>()
            .map_err(|err| Box::new(err) as Box<dyn std::error::Error + Send + Sync>)
    }
}

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
        #[arg(short = 'e', long = "environment")]
        environment: Option<String>,

        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        after: Vec<String>,
    },
    Shell {
        #[arg(short = 'e', long = "environment")]
        environment: Option<String>,

        #[arg(short = 's', long = "shell", default_value = "sh")]
        shell: Option<String>,

        #[arg(short = 'c', long = "command")]
        command: Option<String>,
    },
    #[command(alias = "add")]
    Set {
        #[arg(short = 'e', long = "environment")]
        environment: Option<String>,

        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        vars: Vec<String>,
    },
    Delete {
        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        keys: Vec<String>,
    },
    Setup {},
    List {},
    Project {
        #[command(subcommand)]
        command: ProjectCommands,
    },
}

pub fn parse_secret_pairs(raw: &[String]) -> Result<Vec<(String, String)>, Box<dyn Error>> {
    let mut pairs = Vec::new();

    for item in raw {
        let trimmed = item.trim();

        if trimmed.is_empty() {
            continue;
        }

        let (key, value) = trimmed.split_once('=').ok_or_else(|| {
            Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                format!("Invalid secret format: {}", item),
            )) as Box<dyn Error>
        })?;

        if key.trim().is_empty() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::InvalidInput,
                "Secret key cannot be empty",
            )));
        }

        pairs.push((key.to_string(), value.to_string()));
    }

    Ok(pairs)
}

pub fn format_doppler_set_command(secrets: &[(String, String)]) -> String {
    let mut command = String::from("doppler secrets set");

    if secrets.is_empty() {
        return command;
    }

    command.push_str(" \\");

    for (index, (key, value)) in secrets.iter().enumerate() {
        command.push_str("\n  ");
        command.push_str(key);
        command.push_str("=\"");
        command.push_str(value);
        command.push('"');

        if index + 1 != secrets.len() {
            command.push_str(" \\");
        }
    }

    command
}

#[derive(Subcommand)]
pub enum ProjectCommands {
    List {},
    Create {},
    Delete { name: String },
}

const DB_URL: &str = "/Users/Maaz/Documents/Git/harbor/crates/cli/harbor.db";

pub mod interactions {
    use super::DB_URL;
    use super::db::{establish_connection, models::Project};
    use crate::Environment;
    use crate::db::schema::projects::id;
    use diesel::dsl::{insert_into, update};
    use diesel::result::{DatabaseErrorKind, Error as DieselError};
    use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
    use std::error::Error;
    use uuid::Uuid;

    type Result<T> = std::result::Result<T, Box<dyn Error>>;

    fn construct_error(message: &str) -> Result<()> {
        Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::AlreadyExists,
            message.to_string(),
        )))
    }

    pub fn project_exists(proj_name: &str) -> Result<bool> {
        use super::db::schema::projects::dsl::{name, projects};
        let mut conn = establish_connection(DB_URL.to_string());

        let existing_project = projects
            .filter(name.eq(proj_name))
            .select(Project::as_select())
            .first::<Project>(&mut conn)
            .optional()?;

        Ok(existing_project.is_some())
    }

    pub fn create_project(proj_name: &str) -> Result<()> {
        use super::db::schema::projects::dsl::{created_at, id, name, projects};
        let mut conn = establish_connection(DB_URL.to_string());

        if project_exists(proj_name)? {
            return construct_error("Project already exists");
        }

        let proj_id = Uuid::new_v4().to_string();

        match insert_into(projects)
            .values((
                id.eq(proj_id),
                name.eq(proj_name),
                created_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .execute(&mut conn)
        {
            Ok(_) => Ok(()),
            Err(DieselError::DatabaseError(DatabaseErrorKind::UniqueViolation, _)) => {
                construct_error("Project already exists")
            }
            Err(err) => Err(Box::new(err)),
        }
    }

    pub fn delete_project(proj_name: &str) -> Result<()> {
        use super::db::schema::projects::dsl::{name, projects};
        let mut conn = establish_connection(DB_URL.to_string());

        if !project_exists(proj_name)? {
            return construct_error("Project does not exist");
        }

        diesel::delete(projects.filter(name.eq(proj_name))).execute(&mut conn)?;

        Ok(())
    }

    pub fn get_projects() -> Result<Vec<Project>> {
        use super::db::schema::projects::dsl::projects;
        let mut conn = establish_connection(DB_URL.to_string());

        let results = projects
            .select(Project::as_select())
            .load::<Project>(&mut conn)?;

        Ok(results)
    }

    pub fn secret_exists(
        proj_id: &str,
        secret_name: &str,
        environment: Environment,
    ) -> Result<bool> {
        use super::db::schema::secrets::dsl::{config, id, name, project_id, secrets};
        let mut conn = establish_connection(DB_URL.to_string());

        let existing_secret = secrets
            .filter(project_id.eq(proj_id))
            .filter(name.eq(secret_name))
            .filter(config.eq(environment))
            .select(id)
            .first::<String>(&mut conn)
            .optional()?;

        Ok(existing_secret.is_some())
    }

    pub fn get_project_id(proj_name: &str) -> Result<String> {
        use super::db::schema::projects::dsl::{id, name, projects};
        let mut conn = establish_connection(DB_URL.to_string());

        let project_id = projects
            .filter(name.eq(proj_name))
            .select(id)
            .first::<String>(&mut conn)
            .optional()?;

        match project_id {
            Some(pid) => Ok(pid),
            None => Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Project not found",
            ))),
        }
    }

    pub fn set_secret(
        proj_id: &str,
        secret_name: &str,
        secret_value: Vec<u8>,
        conf: Environment,
        non: crypto::Nonce,
    ) -> Result<()> {
        use super::db::schema::secrets::dsl::{
            config, created_at, id, name, nonce, project_id, secret, secrets,
        };
        let mut conn = establish_connection(DB_URL.to_string());

        if secret_exists(proj_id, secret_name, conf)? {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::AlreadyExists,
                "Secret already exists",
            )));
        }

        insert_into(secrets)
            .values((
                id.eq(Uuid::new_v4().to_string()),
                name.eq(secret_name),
                secret.eq(secret_value),
                project_id.eq(proj_id),
                config.eq(conf),
                nonce.eq(non.to_vec()),
                created_at.eq(chrono::Utc::now().naive_utc()),
            ))
            .execute(&mut conn)?;

        Ok(())
    }

    pub fn get_project_secrets(
        proj_id: &str,
        environment: Environment,
    ) -> Result<Vec<(String, Vec<u8>, Vec<u8>)>> {
        use super::db::schema::secrets::dsl::{config, name, nonce, project_id, secret, secrets};
        let mut conn = establish_connection(DB_URL.to_string());

        let results = secrets
            .filter(project_id.eq(proj_id))
            .filter(config.eq(environment))
            .select((name, secret, nonce))
            .load::<(String, Vec<u8>, Vec<u8>)>(&mut conn)?;

        Ok(results)
    }

    pub fn delete_secret(secret_name: &str) -> Result<()> {
        use super::db::schema::secrets::dsl::{id, name, secrets};
        let mut conn = establish_connection(DB_URL.to_string());

        let existing_secret = secrets
            .filter(name.eq(secret_name))
            .select(id)
            .first::<String>(&mut conn)
            .optional()?;

        if existing_secret.is_none() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Secret does not exist",
            )));
        }

        diesel::delete(secrets.filter(name.eq(secret_name))).execute(&mut conn)?;

        Ok(())
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

pub fn get_input(
    message: impl std::fmt::Display,
    prompt: char,
    new_line: bool,
) -> Result<String, Box<dyn Error>> {
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
