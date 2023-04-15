#!/bin/bash

source .secrets

echo;
echo "Stopping DB"
docker stop $DB_CONTAINER_NAME &&
echo "Done!" ||
echo "Failed to stop DB"

rm -f .env