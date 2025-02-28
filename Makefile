# Makefile for Chinese Code Detector

# Configuration
CARGO := cargo
BIN_NAME := cig
TARGET_DIR := target/release
BUILD_CMD := $(CARGO) build --release

.PHONY: all build clean install uninstall

all: build

build:
	@echo "Building Chinese Code Detector..."
	@$(BUILD_CMD)
	@mv $(TARGET_DIR)/cn-in-git ./$(BIN_NAME)
	@echo "Build complete. Binary: ./$(BIN_NAME)"

clean:
	@echo "Cleaning build artifacts..."
	@$(CARGO) clean
	@rm -f ./$(BIN_NAME)
	@echo "Clean complete"

install: build
	@echo "Installing to /usr/local/bin (may require sudo)"
	@sudo cp ./$(BIN_NAME) /usr/local/bin/$(BIN_NAME)
	@echo "Installation complete. Run with '$(BIN_NAME)'"

uninstall:
	@echo "Removing from /usr/local/bin (may require sudo)"
	@sudo rm -f /usr/local/bin/$(BIN_NAME)
	@echo "Uninstall complete"
