CARGO    := cargo
BIN      := target/release/sysmon
INTERVAL ?= 5

.PHONY: all run live build debug nix-run nix-build clean

all: build run

run: build
	./$(BIN)

live: build
	./$(BIN) -T $(INTERVAL)

build:
	$(CARGO) build --release

debug:
	$(CARGO) run

nix-run:
	nix run

nix-build:
	nix build

clean:
	$(CARGO) clean
