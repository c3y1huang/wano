default: clean build

build:
	cargo build

clean:
	cargo clean

release: clean
	cargo release

test:
	cargo test

run:
	cargo run

fmt:
	cargo fmt

lint:
	cargo clippy --fix --allow-dirty --allow-staged