
build-v8:
	cargo build --example v8 --release

build-jsc:
	cargo build --example jsc --release

build-boa:
	cargo build --example boa --release

build-all: build-v8 build-jsc build-boa

test-all:
	cargo test --release --examples

test-v8:
	cargo test --release --example v8

test-jsc:
	cargo test --release --example jsc

test-boa:
	cargo test --release --example boa