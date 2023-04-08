SECRETS = .secrets
SCRIPT_DIR = .scripts

${SECRETS}:
	@bash ${SCRIPT_DIR}/set_secrets.sh

create_db: ${SECRETS}
	@bash ${SCRIPT_DIR}/create_db.sh

create_db_web_controller: ${SECRETS}
	@bash ${SCRIPT_DIR}/create_web_controller_db.sh

start_all: ${SECRETS}
	@bash ${SCRIPT_DIR}/start_all_db.sh

stop_all: ${SECRETS}
	@bash ${SCRIPT_DIR}/stop_all_db.sh