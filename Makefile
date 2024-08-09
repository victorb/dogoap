.PHONY: all
all: check test clippy doc build

.PHONY: check
check:
	cargo check --all-targets

.PHONY: test
test:
	cargo test

.PHONY: clippy
clippy:
	cargo clippy

.PHONY: doc
doc:
	cargo doc --workspace --no-deps

.PHONY: build
build: check test clippy doc
	cargo build

.PHONY: release
release: check test clippy doc
	cargo build --release

.PHONY: test-coverage
test-coverage:
	cargo tarpaulin -o html

.PHONY: wasm-examples
wasm-examples:
	./build-wasm-examples.sh
