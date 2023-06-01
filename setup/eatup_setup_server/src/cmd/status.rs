use super::*;

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
    match copy_dir_all("/code", "/installation") {
        Ok(_) => Ok(()),
        Err(e) => {
            eprintln!("{}", e);
            Err("Failed to copy code".to_string())
        }
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
        DB_CONTAINER_NAME=eatup_db\n\
        DB_NAME=postgres\n\
        DB_PORT=5432\n\
        DB_USER={}\n\
        DB_USER_PASSWD={}\n\
        # DB Web controller\n\
        WEB_CONTROLLER_CONTAINER_NAME=db_controller\n\
        WEB_CONTROLLER_PORT=1250\n\
        WEB_CONTROLLER_EMAIL={}@admin.com\n\
        WEB_CONTROLLER_PASSWD={}\n\
        # Server\n\
        SERVER_PORT={}\n",
        &db_usr,
        &db_usr_passwd,
        &db_usr,
        &db_usr_passwd,
        server_port
    );
    std::fs::write(ENV, content).unwrap();
}

pub fn create_db() -> Result<(), String> {
    let env = dotenv::from_filename(ENV).unwrap();
    let mut cmd = Command::new("docker");
    let args = format!("\
        run -d --name {} \
        -e POSTGRES_PASSWORD={} \
        -e POSTGRES_USER={} \
        -e POSTGRES_DB={} \
        -v eatup_installation:/docker-entrypoint-initdb.d/ \
        jkutkut/eatup:db_latest",
        env.var("DB_CONTAINER_NAME").unwrap(),
        env.var("DB_USER_PASSWD").unwrap(),
        env.var("DB_USER").unwrap(),
        env.var("DB_NAME").unwrap()
    );
    println!("docker {}", &args);
    cmd.args(args.split(" "));
    let output = cmd.output().expect("Failed to create db container");
    match output.status.success() {
        true => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            Ok(())
        }
        false => {
            let e = format!("Failed to create db container:\n{}", String::from_utf8_lossy(&output.stderr));
            eprintln!("{}", &e);
            Err(e)
        }
    }
}