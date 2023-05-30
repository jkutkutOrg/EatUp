# TODO docker pull all images

images="\
	jkutkut/eatup:db_latest \
	jkutkut/eatup:server_latest \
";

echo "Fetching Docker containers"
for i in $images; do
	echo $i;
	docker pull $i;
done
