use std::path::Path;
use std::process::Command;
use std::{env, fs, io};

use crate::model::*;
use crate::{MICROSERVICES, ENV};
use crate::api::error::InvalidAPI;

pub fn get_all_microservices() -> Vec<Microservice> {
    let mut microservices = vec![];
    for m in MICROSERVICES.iter() {
        microservices.push(Microservice::by_name(m.to_string()));
    }
    microservices
}

pub fn microservice_action(
    action: MicroserviceAction,
    name: String
) -> Option<InvalidAPI> {
    match MICROSERVICES.iter().find(|&m| m == &name) {
        None => return Some(InvalidAPI::new(
            "This container does not exist or belong to this project.".to_string()
        )),
        _ => ()
    }
    match Microservice::by_name(name.to_string()).do_action(action) {
        Some(e) => Some(InvalidAPI::new(e)),
        None => None
    }
}

pub fn get_status() -> ProjectState {
    let public_dir_exists = Path::new("/installation/public").exists();
    if !public_dir_exists {
        return ProjectState::NotCreated;
    }
    let db_container = Microservice::by_name("eatup_db".to_string());
    if db_container.get_state() == MicroserviceState::NotFound {
        return ProjectState::Created;
    }
    ProjectState::Installed
}

pub fn project_create() -> Result<(), String> {
    let status = get_status();
    if status != ProjectState::NotCreated {
        return Err("Project already created".to_string());
    }
    if std::fs::create_dir_all("/installation/public").is_err() {
        println!("Failed to create public directory");
        return Err("Failed to create public directory".to_string());
    }
    // TODO Copy start script
    if std::fs::copy("/code/db/load_db.sql", "/installation/load_db.sql").is_err() {
        println!("Failed to copy load_db.sql");
        return Err("Failed to copy load_db.sql".to_string());
    }
    Ok(())
}

pub fn create_db() -> Result<(), String> {
    dotenv::from_filename(ENV).unwrap();
    let mut cmd = Command::new("docker");
    let args = format!("\
        create --name {} \
        -e POSTGRES_PASSWORD={} \
        -e POSTGRES_USER={} \
        -e POSTGRES_DB={} \
        -v /installation/load_db.sql:/docker-entrypoint-initdb.d/load_db.sql \
        postgres:latest",
        env::var("DB_CONTAINER_NAME").unwrap(),
        env::var("DB_USER_PASSWD").unwrap(),
        env::var("DB_USER").unwrap(),
        env::var("DB_NAME").unwrap()
    );
    cmd.args(args.split(" "));
    match cmd.output() {
        Ok(_) => Ok(()),
        Err(e) => Err(format!("Failed to create db container: {}", e))
    }
}

pub fn create_env_file(
    db_usr: String,
    db_usr_passwd: String,
    server_port: u16
) {
    let content = format!("\
        # EatUp Secrets\n\
        # DB\n\
        DB_CONTAINER_NAME='eatup_db'\n\
        DB_NAME='postgres'\n\
        DB_PORT='5432'\n\
        DB_USER='{}'\n\
        DB_USER_PASSWD='{}'\n\
        # DB Web controller\n\
        WEB_CONTROLLER_CONTAINER_NAME='db_controller'\n\
        WEB_CONTROLLER_PORT='1250'\n\
        WEB_CONTROLLER_EMAIL='{}@admin.com'\n\
        WEB_CONTROLLER_PASSWD='{}'\n\
        # Server\n\
        SERVER_PORT='{}'\n",
        &db_usr,
        &db_usr_passwd,
        &db_usr,
        &db_usr_passwd,
        server_port
    );
    std::fs::write(ENV, content).unwrap();
}

pub fn remove_dir_contents<P: AsRef<Path>>(path: P) -> io::Result<()> {
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();

        if entry.file_type()?.is_dir() {
            remove_dir_contents(&path)?;
            fs::remove_dir(path)?;
        } else {
            fs::remove_file(path)?;
        }
    }
    Ok(())
}