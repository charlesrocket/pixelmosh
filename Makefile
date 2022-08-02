PREFIX=/usr/local
INSTALL_DIR=$(PREFIX)/bin
APP=$(INSTALL_DIR)/pixelmosh
RELEASE=target/release/pixelmosh

build:
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
	@rm -f $(APP)
	cp $(RELEASE) $(APP)

uninstall:
	rm -f $(APP)

clean:
	@rm -f moshed.png test.png
	@cargo clean && cargo clean -r

.PHONY: build test format-check lint install uninstall clean
