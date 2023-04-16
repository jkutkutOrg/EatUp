#!/bin/bash

source .secrets

echo "Starting DB";
docker start $DB_CONTAINER_NAME ||
echo "Failed to start DB";

db_ip=$(docker inspect -f '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}' $DB_CONTAINER_NAME)
cp .secrets .env
echo "DB_IP='$db_ip'" >> .env