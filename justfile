default: clean build

cargo_opts := env_var_or_default("CARGO_OPTS", "")

build:
	cargo build {{cargo_opts}} --workspace

clean:
	cargo clean

release: clean
	cargo build {{cargo_opts}} --release --workspace

test:
	cargo test {{cargo_opts}} --workspace

run:
	cargo run {{cargo_opts}}

fmt:
	cargo fmt {{cargo_opts}}

fix:
	cargo clippy --fix --allow-dirty --allow-staged

udep:
	cargo install cargo-udeps --locked
	cargo +nightly udeps --workspace

publish:
	cargo publish {{cargo_opts}} --manifest-path=./crates/app/Cargo.toml
