#!/bin/sh

id=$(docker run --rm --detach redis:alpine)
ip=$(docker inspect ${id} --format '{{range .NetworkSettings.Networks}}{{.IPAddress}}{{end}}')
export REDIS_URL=redis://${ip}:6379
