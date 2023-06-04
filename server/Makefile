OS	=	$(shell uname -s)
ifeq ($(OS),Linux)
	CURRENT_PATH = $(shell pwd)
endif
ifeq ($(OS),Darwin)
	CURRENT_PATH = ${PWD}
endif

VERSION = "v$(shell grep -m 1 version Cargo.toml | cut -d '"' -f 2)"
PORT = 80

BASIC_ATTRS = -p ${PORT}:${PORT} \
	-v eatup_installation:/installation

ATTRS = ${BASIC_ATTRS} \
	-v ${CURRENT_PATH}:/app -w /app

DOCKER_EXEC = docker run -it --rm --name eatup_server \
	${ATTRS} \
	--entrypoint "/root/.cargo/bin/cargo" \
	-v eatup_cargo_registy:/root/.cargo/registry \
	jkutkut/docker4rust

run:
	${DOCKER_EXEC} run 80 172.17.0.4 5432

build:
	${DOCKER_EXEC} build

init_docker:
	docker run -it --rm ${ATTRS} -w /app -v eatup_cargo_registy:/root/.cargo/registry jkutkut/docker4rust

build_release:
	${DOCKER_EXEC} build --release
	docker build -t jkutkut/eatup:server_$(VERSION) .

push_release:
	docker tag jkutkut/eatup:server_$(VERSION) jkutkut/eatup:server_latest
	docker push jkutkut/eatup:server_$(VERSION)
	docker push jkutkut/eatup:server_latest
