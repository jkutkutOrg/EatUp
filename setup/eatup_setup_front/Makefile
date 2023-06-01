OS	=	$(shell uname -s)
ifeq ($(OS),Linux)
	CURRENT_PATH = $(shell pwd)
endif
ifeq ($(OS),Darwin)
	CURRENT_PATH = ${PWD}
endif

FRONT_NAME = eatup_setup_front
PORT_FRONT = 5173

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

DOCKER_CMD = docker run --rm -it --name ${FRONT_NAME}
DOCKER_APP_V = -v ${CURRENT_PATH}:/app -w /app
DOCKER_IMG_FRONT = node:current-alpine3.16

run_front:
	$(DOCKER_CMD) -p ${PORT_FRONT}:${PORT_FRONT} ${DOCKER_APP_V} --entrypoint=npm -e PORT=${PORT_FRONT} ${DOCKER_IMG_FRONT} run dev

terminal_front:
	$(DOCKER_CMD) ${DOCKER_APP_V} --entrypoint=/bin/sh ${DOCKER_IMG_FRONT}

# ---------------------------------------------------------------------