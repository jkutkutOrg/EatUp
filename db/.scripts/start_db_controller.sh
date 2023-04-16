#!/bin/bash

source .secrets

os=$(uname -s);
if [ "$os" = "Darwin" ]; then
    ip=$(ipconfig getifaddr en1)
else # Linux
    ip=$(ip -4 addr | grep -oP '(?<=inet\s)\d+(\.\d+){3}' | head -n 1)
fi

echo
echo "Starting DB Web Controller";
docker start $WEB_CONTROLLER_CONTAINER_NAME &&
echo "Controller: http://$ip:$WEB_CONTROLLER_PORT" &&
echo "  Email: $WEB_CONTROLLER_EMAIL" &&
echo "  Passwd: $WEB_CONTROLLER_PASSWD" ||
echo "Failed to start DB Web Controller";