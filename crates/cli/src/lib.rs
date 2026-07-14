use base64::{Engine, engine::general_purpose::STANDARD};
use clap::{Args, Parser, Subcommand};
use crypto::helper::gen_key;
use keyring::Entry;
use std::error::Error;
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
    Inject(InjectArgs),
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

#[derive(Args, Debug)]
pub struct InjectArgs {
    #[arg(short, long)]
    pub verbose: bool,

    #[arg(trailing_var_arg = true, allow_hyphen_values = true)]
    pub after: Vec<String>,
}

#[derive(Subcommand)]
pub enum ProjectCommands {
    List {},
    Create { name: String },
    Delete { name: String },
}

#[derive(Subcommand)]
pub enum ConfigCommands {
    List {},
    Create { project: String, name: String },
    Delete { project: String, name: String },
}

fn gen_or_get_key() -> Result<crypto::Key, Box<dyn Error>> {
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
