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
use std::path::PathBuf;
pub mod config;
mod db;
pub mod store;

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
        #[arg(short = 'e', long = "environment")]
        environment: Option<String>,

        #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
        keys: Vec<String>,
    },
    Show {
        #[arg(short = 'e', long = "environment")]
        environment: Option<String>,

        key: String,
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

#[derive(Subcommand)]
pub enum ProjectCommands {
    List {},
    Create {},
    Delete { name: String },
}

pub fn expand_tilde(path: &str) -> String {
    if let Some(stripped) = path.strip_prefix("~/") {
        if let Some(home) = dirs::home_dir() {
            return home.join(stripped).to_string_lossy().to_string();
        }
    }

    path.to_string()
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
