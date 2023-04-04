#!/bin/bash

source .env ||
{
	echo "secrets not created!"
	exit 1
}

echo "Creating DB..."
docker run -d \
	--name $DB_NAME \
	-e POSTGRES_PASSWORD=$DB_USR_PASSWD \
	-e POSTGRES_USER=$DB_USR \
	-e POSTGRES_DB=postgres \
	postgres:latest &&
echo "Done!"