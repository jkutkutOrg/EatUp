#!/bin/bash

source .secrets ||
{
	echo "secrets not created!"
	exit 1
}

echo "Creating DB..."
docker create \
	--name $DB_CONTAINER_NAME \
	-e POSTGRES_PASSWORD=$DB_USER_PASSWD \
	-e POSTGRES_USER=$DB_USER \
	-e POSTGRES_DB=$DB_NAME \
	postgres:latest &&
echo "Done!"

# TODO load data from file