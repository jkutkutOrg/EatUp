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

uninstall_all: ${SECRETS}
	@bash ${SCRIPT_DIR}/uninstall_all_db.sh

destroy_db:
	docker stop eatup_db
	docker rm eatup_db

reset_db: destroy_db create_db start_all

