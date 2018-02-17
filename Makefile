WASM_TARGET=wasm32-unknown-unknown
BUILD_TYPE=release

all: redis
	@REDIS_URL=redis://127.0.0.1:6379/2 cargo test -- --test-threads=1

server: redis
	@REDIS_URL=redis://127.0.0.1:6379/1 cargo run --bin rustybin --features server

redis_cli:
	@docker-compose exec redis redis-cli -n 1

deploy:
	git push heroku master --force-with-lease

redis:
	@docker-compose up -d redis

static:
	rm -f ./static/js/application.*
	cargo-web build --bin application --no-default-features --${BUILD_TYPE} --target ${WASM_TARGET}
	cp ./target/${WASM_TARGET}/${BUILD_TYPE}/application.* ./static/js/
	sed -i -e 's/"application.wasm"/"\/static\/js\/application.wasm"/g' ./static/js/application.js

.PHONY: all server redis_cli deploy static
