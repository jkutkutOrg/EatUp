OS	=	$(shell uname -s)
ifeq ($(OS),Linux)
	CURRENT_PATH = $(shell pwd)
endif
ifeq ($(OS),Darwin)
	CURRENT_PATH = ${PWD}
endif

FRONT_NAME = eatup_setup_front
PORT_FRONT = 5173

BACK_NAME = eatup_setup_server
PORT_BACK = 9000

# ---------------------------------------------------------------------

# Create app directory:
# Enter a docker container:
#   docker run --rm -it -v $(pwd):/app --entrypoint="/bin/sh" -w="/app" node:current-alpine3.16
# once in:
#  npm create vite@latest # Or any: 4.1.0...
# Ok to install the package
# Enter project name
# Select framework: React
# Select variant: TypeScript
# Done! Follow the steps and that's it!

DOCKER_CMD = docker run --rm -it
DOCKER_APP_V = -v ${PWD}/${FRONT_NAME}:/app -w /app
DOCKER_IMG_FRONT = node:current-alpine3.16

run_front:
	$(DOCKER_CMD) -p ${PORT_FRONT}:5173 ${DOCKER_APP_V} --entrypoint=npm ${DOCKER_IMG_FRONT} run dev

terminal_front:
	$(DOCKER_CMD) ${DOCKER_APP_V} --entrypoint=/bin/sh ${DOCKER_IMG_FRONT}

# ---------------------------------------------------------------------

# DOCKER_IMG_SERVER = jkutkut/docker4rust
DOCKER_IMG_SERVER = jkutkut/docker4rust_eatup_setup
CARGO_REGISTRY = eatup_cargo_registy

BASIC_ATTRS = -p ${PORT_BACK}:9000

ATTRS = ${BASIC_ATTRS} \
	-v ${CURRENT_PATH}/${BACK_NAME}:/app -w /app \
	-v ${CARGO_REGISTRY}:/root/.cargo/registry \
	-v /var/run/docker.sock:/var/run/docker.sock:rw

DOCKER_EXEC = docker run -it --rm \
	${ATTRS} \
	--entrypoint "/root/.cargo/bin/cargo" \
	--name ${BACK_NAME} \
	${DOCKER_IMG_SERVER}

run_server:
	${DOCKER_EXEC} run

stop_server:
	docker stop ${BACK_NAME}

build_server:
	${DOCKER_EXEC} build

terminal_server:
	docker run -it --rm ${ATTRS} ${DOCKER_IMG_SERVER}

uwsc:
	uwsc ws://127.0.0.1:${PORT_BACK}/ws

build_server_base:
	docker build -t ${DOCKER_IMG_SERVER} -f ${BACK_NAME}/Dockerfile .


# ---------------------------------------------------------------------