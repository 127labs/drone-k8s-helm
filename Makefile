default: build docker.build
build:
	@cargo clean
	@docker run --rm -it -v $(shell pwd):/home/rust/src ekidd/rust-musl-builder cargo build --release
docker.build:
	@docker rmi 127labs/yakp:latest
	@docker build -t 127labs/yakp:latest .
docker.push:
	@docker push 127labs/yakp:latest
