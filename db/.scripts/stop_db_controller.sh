#!/bin/bash

source .secrets

echo "Stopping controller"
docker stop $WEB_CONTROLLER_CONTAINER_NAME &&
echo "Done!" ||
echo "Failed to stop controller"