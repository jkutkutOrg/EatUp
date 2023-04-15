OS	=	$(shell uname -s)
ifeq ($(OS),Linux)
	CURRENT_PATH = $(shell pwd)
endif
ifeq ($(OS),Darwin)
	CURRENT_PATH = ${PWD}
endif

PORT = 8000

BASIC_ATTRS = -p ${PORT}:8000 \
	-v ${CURRENT_PATH}/../db:/db

ATTRS = ${BASIC_ATTRS} \
	-v ${CURRENT_PATH}:/app -w /app

DOCKER_EXEC = docker run -it --rm \
	${ATTRS} \
	--entrypoint "/root/.cargo/bin/cargo" \
	-v eatup_cargo_registy:/root/.cargo/registry \
	jkutkut/docker4rust

run:
	${DOCKER_EXEC} run

build:
	${DOCKER_EXEC} build

init_docker:
	docker run -it --rm ${ATTRS} -w /app -v eatup_cargo_registy:/root/.cargo/registry jkutkut/docker4rust

build_release:
	${DOCKER_EXEC} build --release
	docker build -t jkutkut/eatup:v0.2.0 .

# ******** Public tools ********

run_release:
	docker run -it --rm ${BASIC_ATTRS} --name eatup jkutkut/eatup:latest

stop:
	docker stop eatup