.PHONY: all
all: check test clippy doc build

.PHONY: check
check:
	cargo check --all-targets

.PHONY: test
test:
	cargo test --no-default-features

.PHONY: test-watch
test-watch:
	cargo watch -s "make test"

# TODO flaky as it doesn't always get the right state within 3 frames
.PHONY: test-compute-pool
test-compute-pool:
	cargo test --features=compute-pool

.PHONY: clippy
clippy:
	cargo clippy

.PHONY: doc
doc:
	cargo doc --workspace --no-deps

.PHONY: doc-watch
doc-watch:
	cargo watch -s "make doc"

.PHONY: build
build: check test clippy doc
	cargo build

.PHONY: release
release: check test clippy doc
	cargo build --release

.PHONY: test-coverage
test-coverage:
	cargo tarpaulin --no-default-features -o html

.PHONY: wasm-examples
wasm-examples:
	./build-wasm-examples.sh
