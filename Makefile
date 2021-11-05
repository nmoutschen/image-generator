STACK_NAME ?= rust-imgen
ARCH := aarch64-unknown-linux-gnu

.PHONY: build deploy

all: build deploy

build:
	cross build --release --target $(ARCH)
	rm -rf ./build
	mkdir -p ./build
	cp -v ./target/$(ARCH)/release/imgen ./build/bootstrap

deploy:
	if [ -f samconfig.toml ]; \
		then sam deploy --stack-name $(STACK_NAME); \
		else sam deploy -g --stack-name $(STACK_NAME); \
	fi