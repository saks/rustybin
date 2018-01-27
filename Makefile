all: redis
	@REDIS_URL=redis://127.0.0.1:6379/2 cargo test -- --test-threads=1

server: redis
	@REDIS_URL=redis://127.0.0.1:6379/1 cargo run

redis_cli:
	@docker-compose exec redis redis-cli -n 1

deploy:
	git push heroku master --force-with-lease

redis:
	@docker-compose up -d redis

.PHONY: all server redis_cli deploy
