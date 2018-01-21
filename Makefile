all:
	REDIS_URL=redis://127.0.0.1:6379 cargo test -- --test-threads=1

.PHONY: all
