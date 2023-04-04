#!/bin/bash

source .env

os=$(uname -s);
if [ "$os" = "Darwin" ]; then
    ip=$(ipconfig getifaddr en1)
else # Linux
    ip=$(ip -4 addr | grep -oP '(?<=inet\s)\d+(\.\d+){3}' | head -n 1)
fi


echo "Starting DB";
docker start $DB_NAME &&
echo "DB: $ip:$DB_PORT" &&
echo "  Usr: $DB_USR" &&
echo "  Passwd: $DB_USR_PASSWD" ||
echo "Failed to start DB";

echo
echo "Starting DB Web Controller";
docker start $WEB_CONTROLLER_NAME &&
echo "Controller: $ip:$WEB_CONTROLLER_PORT" &&
echo "  Email: $WEB_CONTROLLER_EMAIL" &&
echo "  Passwd: $WEB_CONTROLLER_PASSWD" ||
echo "Failed to start DB Web Controller";