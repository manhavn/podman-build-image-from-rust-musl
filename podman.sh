#!/bin/bash
# shellcheck disable=SC2164
cd "$(dirname "$0")"

IMAGE_NAME="docker.zii.ovh/rust-api-demo:latest"

podman manifest create $IMAGE_NAME

podman build --arch amd64 \
  --manifest $IMAGE_NAME .

podman manifest push $IMAGE_NAME $IMAGE_NAME

podman manifest rm $IMAGE_NAME

podman image prune -f

podman run --rm -p 8080:8080 $IMAGE_NAME

# curl http://localhost:8080/

podman image rm $IMAGE_NAME

#podman image prune -f
