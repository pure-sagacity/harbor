use clap::{Args, Parser, Subcommand};
use keyring::Entry;
use std::error::Error;
mod db;

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

pub fn gen_or_get_key() -> Result<crypto::Key, Box<dyn Error>> {
    let entry = Entry::new("harbor", "encryption-key")?;

    match entry.get_password() {
        Ok(hex_string) => {
            let key_bytes = hex::decode(hex_string)?;

            if key_bytes.len() != 32 {
                return Err("Retrieved key length is invalid (must be 32 bytes)".into());
            }

            Ok(key_bytes
                .as_slice()
                .try_into()
                .map_err(|_| "Invalid key length")?)
        }

        Err(err) => {
            if is_key_not_found(&err) {
                let key = crypto::helper::gen_key();

                let hex_string = hex::encode(key.as_slice());
                entry.set_password(&hex_string)?;

                Ok(key)
            } else {
                Err(Box::new(err))
            }
        }
    }
}

fn is_key_not_found(err: &keyring::Error) -> bool {
    matches!(err, keyring::Error::NoEntry)
}
