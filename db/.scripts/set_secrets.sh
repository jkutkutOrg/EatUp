#!/bin/bash

askPassword() {
	concept="$1"
	while true; do
		read -s -p "Password for $concept: " PASSWD;
		echo;
		read -s -p "Confirm password: " PASSWD_CONFIRM;
		echo;
		if [ "$PASSWD" = "$PASSWD_CONFIRM" ]; then
			break;
		fi
		if [ "$PASSWD" = "" ]; then
			echo "Password cannot be empty";
		else
			echo "Passwords do not match";
		fi
		echo;
	done
}

SECRET_FILE=".secrets"

askData() {
	question="$1"
	default="$2"

	if [ ! "$default" = "" ]; then
		question="$question [$default]: ";
	fi

	read -p "$question" data

	if [ "$data" = "" ]; then
		data="$default"
	fi
}


echo "DB:";
askData "DB_CONTAINER_NAME" "eatup_db"; DB_CONTAINER_NAME="$data";
askData "DB_USER" "admin"; DB_USER="$data";
askPassword "$DB_CONTAINER_NAME"; DB_USER_PASSWD="$PASSWD";

echo;
echo "DB Web controller:";
askData "WEB_CONTROLLER_CONTAINER_NAME" "db_controller"; WEB_CONTROLLER_CONTAINER_NAME="$data";
askData "WEB_CONTROLLER_PORT" "1250"; WEB_CONTROLLER_PORT="$data";
askData "WEB_CONTROLLER_EMAIL" "admin@admin.com"; WEB_CONTROLLER_EMAIL="$data";
askPassword "$WEB_CONTROLLER_EMAIL"; WEB_CONTROLLER_PASSWD="$PASSWD";

echo "#!/bin/bash
# DB
DB_CONTAINER_NAME='$DB_CONTAINER_NAME'
DB_NAME='postgres'
DB_PORT='5432'
DB_USER='$DB_USER'
DB_USER_PASSWD='$DB_USER_PASSWD'
# DB Web controller
WEB_CONTROLLER_CONTAINER_NAME='$WEB_CONTROLLER_CONTAINER_NAME'
WEB_CONTROLLER_PORT='$WEB_CONTROLLER_PORT'
WEB_CONTROLLER_EMAIL='$WEB_CONTROLLER_EMAIL'
WEB_CONTROLLER_PASSWD='$WEB_CONTROLLER_PASSWD'
" > $SECRET_FILE