PREFIX=/usr/local
INSTALL_DIR=$(PREFIX)/bin
DEST=$(INSTALL_DIR)/pixelmosh
BIN=target/release/pixelmosh
SOURCE_FILES = $(shell test -e src/ && find src -type f)

all: build

build: $(BIN)

$(BIN): $(SOURCE_FILES)
	@cargo build --release

test:
	@cargo test

format-check:
	@-rustup component add rustfmt 2> /dev/null
	@cargo fmt --all -- --check

lint:
	@-rustup component add clippy 2> /dev/null
	@cargo clippy --all-features --all -- -D clippy::all -D warnings

install:
	@rm -f $(DEST)
	cp $(BIN) $(DEST)

uninstall:
	rm -f $(DEST)

clean:
	@rm -f moshed.png test.png
	@cargo clean && cargo clean -r

.PHONY: test format-check lint install uninstall clean
