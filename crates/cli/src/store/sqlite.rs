use crate::db::models::Project;
use crate::store::Store;
use diesel::dsl::{insert_into, update};
use diesel::result::{DatabaseErrorKind, Error as DieselError};
use diesel::{ExpressionMethods, OptionalExtension, QueryDsl, RunQueryDsl, SelectableHelper};
use uuid::Uuid;

pub struct SqliteStore {
    database_url: String,
}

impl SqliteStore {
    pub fn new(database_url: String) -> Self {
        SqliteStore { database_url }
    }

    fn db_url(&self) -> String {
        if self.database_url.is_empty() {
            std::env::var("DATABASE_URL").expect("NO DATABASE URL WAS SET")
        } else {
            self.database_url.clone()
        }
    }
}

fn construct_error(message: &str) -> super::Result<()> {
    Err(Box::new(std::io::Error::new(
        std::io::ErrorKind::AlreadyExists,
        message.to_string(),
    )))
}

impl Store for SqliteStore {
    fn create_project(&self, proj_name: &str) -> super::Result<()> {
        use crate::db::schema::projects::dsl::{created_at, id, name, projects};
        let mut conn = crate::db::establish_connection(self.db_url());

        if self.project_exists(proj_name)? {
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

    fn delete_project(&self, proj_name: &str) -> super::Result<()> {
        use crate::db::schema::projects::dsl::{name, projects};
        let mut conn = crate::db::establish_connection(self.db_url());

        if !self.project_exists(proj_name)? {
            return construct_error("Project does not exist");
        }

        diesel::delete(projects.filter(name.eq(proj_name))).execute(&mut conn)?;

        Ok(())
    }
    fn delete_secret(
        &self,
        proj_id: &str,
        secret_name: &str,
        environment: crate::Environment,
    ) -> super::Result<()> {
        use crate::db::schema::secrets::dsl::{config, id, name, project_id, secrets};
        let mut conn = crate::db::establish_connection(self.db_url());

        let existing_secret = secrets
            .filter(project_id.eq(proj_id))
            .filter(name.eq(secret_name))
            .filter(config.eq(environment))
            .select(id)
            .first::<String>(&mut conn)
            .optional()?;

        if existing_secret.is_none() {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Secret does not exist",
            )));
        }

        diesel::delete(
            secrets
                .filter(project_id.eq(proj_id))
                .filter(name.eq(secret_name))
                .filter(config.eq(environment)),
        )
        .execute(&mut conn)?;

        Ok(())
    }
    fn get_project_id(&self, proj_name: &str) -> super::Result<String> {
        use crate::db::schema::projects::dsl::{id, name, projects};
        let mut conn = crate::db::establish_connection(self.db_url());

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
    fn get_project_secrets(
        &self,
        proj_id: &str,
        environment: crate::Environment,
    ) -> super::Result<Vec<(String, Vec<u8>, Vec<u8>)>> {
        use crate::db::schema::secrets::dsl::{config, name, nonce, project_id, secret, secrets};
        let mut conn = crate::db::establish_connection(self.db_url());

        let results = secrets
            .filter(project_id.eq(proj_id))
            .filter(config.eq(environment))
            .select((name, secret, nonce))
            .load::<(String, Vec<u8>, Vec<u8>)>(&mut conn)?;

        Ok(results)
    }
    fn get_projects(&self) -> super::Result<Vec<Project>> {
        use crate::db::schema::projects::dsl::projects;
        let mut conn = crate::db::establish_connection(self.db_url());

        let results = projects
            .select(Project::as_select())
            .load::<Project>(&mut conn)?;

        Ok(results)
    }

    fn project_exists(&self, proj_name: &str) -> super::Result<bool> {
        use crate::db::schema::projects::dsl::{name, projects};
        let mut conn = crate::db::establish_connection(self.db_url());

        let existing_project = projects
            .filter(name.eq(proj_name))
            .select(Project::as_select())
            .first::<Project>(&mut conn)
            .optional()?;

        Ok(existing_project.is_some())
    }
    fn secret_exists(
        &self,
        proj_id: &str,
        secret_name: &str,
        environment: crate::Environment,
    ) -> super::Result<bool> {
        use crate::db::schema::secrets::dsl::{config, id, name, project_id, secrets};
        let mut conn = crate::db::establish_connection(self.db_url());

        let existing_secret = secrets
            .filter(project_id.eq(proj_id))
            .filter(name.eq(secret_name))
            .filter(config.eq(environment))
            .select(id)
            .first::<String>(&mut conn)
            .optional()?;

        Ok(existing_secret.is_some())
    }
    fn set_secret(
        &self,
        proj_id: &str,
        secret_name: &str,
        secret_value: Vec<u8>,
        conf: crate::Environment,
        non: crypto::Nonce,
    ) -> super::Result<()> {
        use crate::db::schema::secrets::dsl::{
            config, created_at, id, name, nonce, project_id, secret, secrets,
        };
        let mut conn = crate::db::establish_connection(self.db_url());

        if self.secret_exists(proj_id, secret_name, conf)? {
            update(
                secrets
                    .filter(project_id.eq(proj_id))
                    .filter(name.eq(secret_name))
                    .filter(config.eq(conf)),
            )
            .set((secret.eq(secret_value), nonce.eq(non.to_vec())))
            .execute(&mut conn)?;
        } else {
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
        }

        Ok(())
    }
}
