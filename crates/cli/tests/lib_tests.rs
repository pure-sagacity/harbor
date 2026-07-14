use cli::Environment;
use cli::config::Config;
use cli::interactions::{
    create_project,
    delete_project,
    delete_secret,
    get_project_id,
    get_project_secrets,
    get_projects,
    project_exists,
    secret_exists,
    set_secret,
};
use cli::{format_doppler_set_command, parse_secret_pairs};
use diesel::connection::SimpleConnection;
use diesel::{Connection, SqliteConnection};
use std::path::PathBuf;
use std::sync::{Mutex, OnceLock};
use uuid::Uuid;

fn test_lock() -> &'static Mutex<()> {
    static LOCK: OnceLock<Mutex<()>> = OnceLock::new();
    LOCK.get_or_init(|| Mutex::new(()))
}

fn setup_test_db() -> PathBuf {
    let db_path = std::env::temp_dir().join(format!("harbor_test_{}.db", Uuid::new_v4()));
    std::fs::File::create(&db_path).expect("create sqlite file");

    unsafe {
        std::env::set_var("DATABASE_URL", &db_path);
    }

    let database_url = db_path.to_str().expect("db path");
    let mut conn = SqliteConnection::establish(database_url).expect("connect sqlite");
    conn.batch_execute(
        "PRAGMA foreign_keys = ON;\
        CREATE TABLE projects (\
            id TEXT PRIMARY KEY NOT NULL,\
            name TEXT NOT NULL UNIQUE,\
            created_at TIMESTAMP NOT NULL\
        );\
        CREATE TABLE secrets (\
            id TEXT PRIMARY KEY NOT NULL DEFAULT (lower(hex(randomblob(16)))),\
            name TEXT NOT NULL,\
            project_id TEXT NOT NULL,\
            config TEXT NOT NULL,\
            secret BLOB NOT NULL,\
            nonce BLOB NOT NULL,\
            created_at TIMESTAMP NOT NULL,\
            FOREIGN KEY(project_id) REFERENCES projects(id)\
        );",
    )
    .expect("create schema");

    db_path
}

fn cleanup_test_db(path: PathBuf) {
    let _ = std::fs::remove_file(path);
}

#[test]
fn environment_parsing_and_display() {
    let dev: Environment = "DEV".parse().expect("parse dev");
    let staging: Environment = " staging ".parse().expect("parse staging");

    assert_eq!(dev, Environment::Dev);
    assert_eq!(staging, Environment::Staging);
    assert_eq!(format!("{}", Environment::Prod), "prod");

    let err = "unknown".parse::<Environment>().unwrap_err();
    assert_eq!(err.to_string(), "Environment must be one of: dev, prod, staging");
}

#[test]
fn parse_secret_pairs_handles_valid_and_invalid_input() {
    let input = vec![
        " FOO=bar ".to_string(),
        "".to_string(),
        "BAZ=qux".to_string(),
        "A=B=C".to_string(),
    ];

    let parsed = parse_secret_pairs(&input).expect("parse secret pairs");
    assert_eq!(parsed.len(), 3);
    assert_eq!(parsed[0], ("FOO".to_string(), "bar".to_string()));
    assert_eq!(parsed[1], ("BAZ".to_string(), "qux".to_string()));
    assert_eq!(parsed[2], ("A".to_string(), "B=C".to_string()));

    let err = parse_secret_pairs(&["NOPE".to_string()]).unwrap_err();
    assert_eq!(err.to_string(), "Invalid secret format: NOPE");

    let err = parse_secret_pairs(&["=value".to_string()]).unwrap_err();
    assert_eq!(err.to_string(), "Secret key cannot be empty");
}

#[test]
fn format_doppler_set_command_outputs_expected_format() {
    let empty = format_doppler_set_command(&[]);
    assert_eq!(empty, "doppler secrets set");

    let one = format_doppler_set_command(&[("API_KEY".to_string(), "abc".to_string())]);
    assert_eq!(one, "doppler secrets set \\\n  API_KEY=\"abc\"");

    let many = format_doppler_set_command(&[
        ("A".to_string(), "1".to_string()),
        ("B".to_string(), "2".to_string()),
    ]);

    assert_eq!(
        many,
        "doppler secrets set \\\n  A=\"1\" \\\n  B=\"2\""
    );
}

#[test]
fn config_reads_valid_file_and_reports_errors() {
    let temp_root = std::env::temp_dir().join(format!("harbor_config_{}", Uuid::new_v4()));
    std::fs::create_dir_all(&temp_root).expect("create temp dir");

    let config_path = temp_root.join(".harbor.toml");
    std::fs::write(
        &config_path,
        "version = \"1\"\nname = \"proj\"\nconfig = \"dev\"\n",
    )
    .expect("write config");

    let config = Config::from_repo_root(&temp_root).expect("read config");
    assert_eq!(config.name, "proj");
    assert_eq!(config.version, "1");
    assert_eq!(config.default_env, Environment::Dev);

    let invalid_path = temp_root.join("bad.toml");
    std::fs::write(&invalid_path, "not = [valid").expect("write invalid");
    let err = Config::read_main_file(&invalid_path).unwrap_err();
    assert!(err.to_string().contains("TOML error"));

    let missing_path = temp_root.join("missing.toml");
    let err = Config::read_main_file(&missing_path).unwrap_err();
    assert!(err.to_string().contains("IO error"));

    let _ = std::fs::remove_dir_all(&temp_root);
}

#[test]
fn project_and_secret_lifecycle() {
    let _guard = test_lock().lock().expect("lock tests");
    let db_path = setup_test_db();

    assert!(!project_exists("alpha").expect("project exists"));

    create_project("alpha").expect("create project");
    assert!(project_exists("alpha").expect("project exists"));

    let project_id = get_project_id("alpha").expect("get project id");
    let projects = get_projects().expect("get projects");
    assert_eq!(projects.len(), 1);
    assert_eq!(projects[0].name, "alpha");

    assert!(!secret_exists().expect("secret exists"));

    let nonce = crypto::helper::gen_nonce();
    set_secret(
        &project_id,
        "API_KEY",
        b"first".to_vec(),
        Environment::Dev,
        nonce,
    )
    .expect("set secret");

    assert!(secret_exists().expect("secret exists"));

    let secrets = get_project_secrets(&project_id, Environment::Dev).expect("get secrets");
    assert_eq!(secrets.len(), 1);
    assert_eq!(secrets[0].0, "API_KEY");
    assert_eq!(secrets[0].1, b"first".to_vec());

    let nonce = crypto::helper::gen_nonce();
    set_secret(
        &project_id,
        "API_KEY",
        b"second".to_vec(),
        Environment::Dev,
        nonce,
    )
    .expect("update secret");

    let updated = get_project_secrets(&project_id, Environment::Dev).expect("get secrets");
    assert_eq!(updated.len(), 1);
    assert_eq!(updated[0].1, b"second".to_vec());

    delete_secret("API_KEY").expect("delete secret");
    assert!(!secret_exists().expect("secret exists"));

    delete_project("alpha").expect("delete project");
    assert!(!project_exists("alpha").expect("project exists"));

    cleanup_test_db(db_path);
}

#[test]
fn errors_for_missing_project_or_secret() {
    let _guard = test_lock().lock().expect("lock tests");
    let db_path = setup_test_db();

    let err = delete_project("missing").unwrap_err();
    assert_eq!(err.to_string(), "Project does not exist");

    let err = get_project_id("missing").unwrap_err();
    assert_eq!(err.to_string(), "Project not found");

    let err = delete_secret("missing").unwrap_err();
    assert_eq!(err.to_string(), "Secret does not exist");

    cleanup_test_db(db_path);
}
