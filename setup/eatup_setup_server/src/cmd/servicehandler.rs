use super::*;

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

pub fn run_server() -> Result<(), String> {
    let db = Microservice::by_name("eatup_db".to_string());
    if db.get_state() != MicroserviceState::Running {
        return Err("DB container is not running".to_string());
    }
    let env = dotenv::from_filename(ENV).unwrap();
    let port = env.var("SERVER_PORT").unwrap();
    let args = format!("run -d --rm --name {} \
        -p {}:{} \
        -v eatup_installation:/installation:rw \
        jkutkut/eatup:server_latest \
        {} {} {}",
        "eatup_server",
        &port, &port,
        &port,
        db.get_ip().unwrap(),
        "5432"
    );
    #[cfg(debug_assertions)]
    println!("docker {}", &args);
    let args = args.split(" ");
    let mut cmd = Command::new("docker");
    cmd.args(args);
    let output = cmd.output().expect("Failed to run server");
    match output.status.success() {
        true => {
            println!("{}", String::from_utf8_lossy(&output.stdout));
            Ok(())
        },
        false => {
            let e = format!("Failed to run server:\n{}", String::from_utf8_lossy(&output.stderr));
            eprintln!("{}", &e);
            Err(e)
        }
    }
}