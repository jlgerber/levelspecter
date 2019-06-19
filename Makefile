
build:
	cargo build --release --features case-insensitive

install:
	cp target/release/levelspecter ~/bin/.

.PHONY: all
all: build install

.PHONY: test
test:
	cargo test --release
	cargo test --release --features case-insensitive
