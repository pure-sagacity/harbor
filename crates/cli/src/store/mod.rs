// This trait will replace the current implementation.
// Right now, the implementation is locked on only SQLite.
// With this trait, we can implement other savers like Postgres, MySQL, etc.
// Or even third party services like Hashicorp Vault, AWS Secrets Manager, etc.

use std::error::Error;

pub mod sqlite;

use crate::{Environment, db::models::Project};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub trait Store {
    fn project_exists(&self, proj_name: &str) -> Result<bool>;
    fn create_project(&self, proj_name: &str) -> Result<()>;
    fn delete_project(&self, proj_name: &str) -> Result<()>;
    fn get_projects(&self) -> Result<Vec<Project>>;
    fn get_project_id(&self, proj_name: &str) -> Result<String>;
    fn secret_exists(&self, proj_id: &str, secret_name: &str, environment: Environment)
        -> Result<bool>;
    fn set_secret(
        &self,
        proj_id: &str,
        secret_name: &str,
        secret_value: Vec<u8>,
        conf: Environment,
        non: crypto::Nonce,
    ) -> Result<()>;
    fn get_project_secrets(
        &self,
        proj_id: &str,
        environment: Environment,
    ) -> Result<Vec<(String, Vec<u8>, Vec<u8>)>>;
    fn delete_secret(&self, proj_id: &str, secret_name: &str, environment: Environment)
        -> Result<()>;
}
