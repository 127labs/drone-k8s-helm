default: build docker.build docker.push
build:
	@cargo clean
	@docker run --rm -it -v $(shell pwd):/home/rust/src ekidd/rust-musl-builder cargo build --release
docker.build:
	@docker build -t 127labs/drone-k8s-helm:latest .
docker.push:
	@docker push 127labs/drone-k8s-helm:latest
