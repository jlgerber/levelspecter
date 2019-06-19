
build:
	cargo build --release --features case-insensitive

install:
	cp target/release/levelspecter ~/bin/.

.PHONY: all
all: build install

.PHONY: test-case-insensitive
test-case-insensitive:
	cargo test --release --features case-insensitive

.PHONY: test-case-sensitive
test-case-sensitive:
	cargo test --release

.PHONY: test
test: test-case-insensitive  test-case-sensitive
