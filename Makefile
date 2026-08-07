BINARY_NAME=vault
PREFIX=$(HOME)/.local/bin

.PHONY: all build install clean uninstall help

all: build

help:
	@echo "Available targets: build, install, uninstall, clean"

build:
	cargo build --release

install: build
	@test -f target/release/$(BINARY_NAME) || (echo "Error: Binary not found"; exit 1)
	@echo "Installing $(BINARY_NAME) to $(PREFIX)..."
	@install -Dm755 target/release/$(BINARY_NAME) $(PREFIX)/$(BINARY_NAME)
	@echo "Successfully installed $(BINARY_NAME)!"

uninstall:
	@echo "Removing $(BINARY_NAME) from $(PREFIX)..."
	@rm -f $(PREFIX)/$(BINARY_NAME)
	@echo "Successfully uninstalled $(BINARY_NAME)!"

clean:
	cargo clean
