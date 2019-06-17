build:
	cargo build --release

install:
	cp target/release/levelspecter ~/bin/.

.PHONY: all
all: build install

.PHONY: test
test:
	cargo test --release
