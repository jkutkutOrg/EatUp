create_db_controller:
	@echo "Creating database controller..."
	docker create --name db_controller -p 8080:80 -e PGADMIN_DEFAULT_EMAIL="admin@admin.com" -e PGADMIN_DEFAULT_PASSWORD="admin" dpage/pgadmin4

start_db_controller:
	@echo "Starting database controller..."
	docker start db_controller

stop_db_controller:
	@echo "Stopping database controller..."
	docker stop db_controller