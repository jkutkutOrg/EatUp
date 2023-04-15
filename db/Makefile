SECRETS = .secrets
SCRIPT_DIR = .scripts

# ******** Create ********

${SECRETS}:
	@bash ${SCRIPT_DIR}/set_secrets.sh

create_db: ${SECRETS}
	@bash ${SCRIPT_DIR}/create_db.sh

create_db_web_controller: ${SECRETS}
	@bash ${SCRIPT_DIR}/create_web_controller_db.sh

# ******** Run ********

start_db: ${SECRETS}
	@bash ${SCRIPT_DIR}/start_db.sh

start_db_controller: ${SECRETS}
	@bash ${SCRIPT_DIR}/start_db_controller.sh

start_all: start_db start_db_controller

# ******** Stop ********

stop_db: ${SECRETS}
	@bash ${SCRIPT_DIR}/stop_db.sh

stop_db_controller: ${SECRETS}
	@bash ${SCRIPT_DIR}/stop_db_controller.sh

stop_all: stop_db stop_db_controller

# ******** Destroy ********

uninstall_db: ${SECRETS}
	@bash ${SCRIPT_DIR}/uninstall_db.sh

uninstall_db_controller: ${SECRETS}
	@bash ${SCRIPT_DIR}/uninstall_db_controller.sh

uninstall_all: uninstall_db uninstall_db_controller

# ******** tools ********

install: create_db start_db

start: start_db

stop: stop_db

reset_db: uninstall_db create_db start_db

uninstall: uninstall_db