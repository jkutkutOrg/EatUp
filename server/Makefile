OS	=	$(shell uname -s)
ifeq ($(OS),Linux)
	CURRENT_PATH = $(shell pwd)
endif
ifeq ($(OS),Darwin)
	CURRENT_PATH = ${PWD}
endif

DOCKER_EXEC = docker run -it --rm -v ${CURRENT_PATH}:/app -w /app --entrypoint "/root/.cargo/bin/cargo" jkutkut/docker4rust

run:
	${DOCKER_EXEC} run

build:
	${DOCKER_EXEC} build

init_docker:
	docker run -it --rm -v ${CURRENT_PATH}:/app -w /app jkutkut/docker4rust
