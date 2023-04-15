install:
	@make -C db install

run:
	@make -C db start
	@make -C server run_release

stop:
	@make -C server stop
	@make -C db stop

uninstall:
	@make -C db uninstall

reset_db:
	@make -C db reset_db