#!/bin/bash

source .env ||
{
	echo "secrets not created!"
	exit 1
}

echo "Creating web controller for DataBases:"
docker run -d \
	--name $WEB_CONTROLLER_NAME \
	-p $WEB_CONTROLLER_PORT:80 \
	-e PGADMIN_DEFAULT_EMAIL="$WEB_CONTROLLER_EMAIL" \
	-e PGADMIN_DEFAULT_PASSWORD="$WEB_CONTROLLER_PASSWD" \
	dpage/pgadmin4 &&
echo "Done!"