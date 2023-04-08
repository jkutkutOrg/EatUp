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
	-v ./load_db.sql:/docker-entrypoint-initdb.d/load_db.sql \
	postgres:latest &&
echo "Done!"