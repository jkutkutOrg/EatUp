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


images="\
	jkutkut/eatup:db_latest \
	jkutkut/eatup:server_latest \
	jkutkut/eatup:setup_latest
";

echo "Fetching Docker containers"
for i in $images; do
	echo "  $i";
	docker pull $i > /dev/null || {
		echo "Error fetching $i";
		exit 1;
	}
done

echo
askData "Port to use in the setup service" "9000"; port=$data
echo
echo "Starting setup service"

docker run -it --rm --name eatup_setup -p $port:$port -v eatup_installation:/installation:rw -v /var/run/docker.sock:/var/run/docker.sock:rw jkutkut/eatup:setup_latest $port