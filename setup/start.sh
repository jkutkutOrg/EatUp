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
echo "Starting setup service"
port=9000

docker run -it --rm --name eatup_setup -p $port:$port -v eatup_installation:/installation:rw -v /var/run/docker.sock:/var/run/docker.sock:rw jkutkut/eatup:setup_latest $port