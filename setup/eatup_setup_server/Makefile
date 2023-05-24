# ***** Constants *****
OS	=	$(shell uname -s)
ifeq ($(OS),Linux)
	CURRENT_PATH = $(shell pwd)
endif
ifeq ($(OS),Darwin)
	CURRENT_PATH = ${PWD}
endif

# ****** Variables ******
CONTAINER_NAME = eatup_setup_server
PORT = 42
IMAGE_NAME = jkutkut/eatup:setup_server
IMAGE_NAME_DEV = ${IMAGE_NAME}_dev
VERSION = "v$(shell grep -m 1 version Cargo.toml | cut -d '"' -f 2)"

# ****** Docker config ******
BASIC_ATTRS = -p ${PORT}:${PORT} --name ${CONTAINER_NAME}

ATTRS = ${BASIC_ATTRS} \
	-v ${CURRENT_PATH}:/app -w /app \
	-v rust_cargo_registy:/root/.cargo/registry \
	-v /var/run/docker.sock:/var/run/docker.sock:rw \
	-v eatup_installation:/installation \
	-v ${CURRENT_PATH}/../../:/code

DOCKER_EXEC = docker run -it --rm \
	${ATTRS} \
	--entrypoint "/root/.cargo/bin/cargo" \
	${IMAGE_NAME_DEV}

# ****** DEV ******
create_dev_image:
	docker build -t ${IMAGE_NAME_DEV} --target dev .

run:
	${DOCKER_EXEC} run ${PORT}

build:
	${DOCKER_EXEC} build

init_docker:
	docker run -it --rm ${ATTRS} ${IMAGE_NAME_DEV}

connect_docker:
	docker exec -it ${CONTAINER_NAME} sh

# ****** Release ******

build_release:
	${DOCKER_EXEC} build --release
	docker build -t ${IMAGE_NAME}_$(VERSION) --target release .

push_release:
	docker tag ${IMAGE_NAME}_$(VERSION) ${IMAGE_NAME}_latest
	docker push ${IMAGE_NAME}_$(VERSION)
	docker push ${IMAGE_NAME}_latest

# ****** Public tools ******

run_release:
	docker run -it --name ${CONTAINER_NAME} --rm ${BASIC_ATTRS} ${IMAGE_NAME}_$(VERSION)

stop:
	docker stop ${CONTAINER_NAME}