#!/bin/bash

source .env

echo "Stopping controller"
docker stop $WEB_CONTROLLER_NAME &&
echo "Done!" ||
echo "Failed to stop controller"

echo;
echo "Stopping DB"
docker stop $DB_NAME &&
echo "Done!" ||
echo "Failed to stop DB"