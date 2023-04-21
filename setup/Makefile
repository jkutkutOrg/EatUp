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

PORT = 5173

DOCKER_CMD = docker run --rm -it
DOCKER_APP_V = -v ${PWD}/eatup_setup:/app -w /app
DOCKER_IMG = node:current-alpine3.16

run:
	$(DOCKER_CMD) -p ${PORT}:5173 ${DOCKER_APP_V} --entrypoint=npm ${DOCKER_IMG} run dev

terminal:
	$(DOCKER_CMD) ${DOCKER_APP_V} --entrypoint=/bin/sh ${DOCKER_IMG}

# ---------------------------------------------------------------------

create_vscode:
	docker env create -o 

vscode:
	docker env start -o 