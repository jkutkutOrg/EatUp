#!/bin/bash

source .secrets ||
{
	echo "secrets not created!"
	exit 1
}

echo "Removing DB Web Controller..." &&
docker stop $WEB_CONTROLLER_CONTAINER_NAME > /dev/null &&
docker rm $WEB_CONTROLLER_CONTAINER_NAME  > /dev/null &&
echo "Done!"