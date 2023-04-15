#!/bin/bash

source .secrets ||
{
	echo "secrets not created!"
	exit 1
}

echo "Removing DB..." &&
docker stop $DB_CONTAINER_NAME > /dev/null &&
docker rm $DB_CONTAINER_NAME > /dev/null &&
echo "Done!"

rm -f .env #.secrets