NAME = hina
PREFIX = /usr/local
MAN_DIR = man
MAN_FILES = $(wildcard $(MAN_DIR)/*.man)
GZ_FILES = $(patsubst $(MAN_DIR)/%.man,$(MAN_DIR)/%.1,$(MAN_FILES))

default: build man

ifeq ($(shell command -v cargo 2> /dev/null),)
    $(error "cargo is not installed.")
endif

clean:
	@echo "Cleaning build dir"
	@rm -rf target/*
	@echo "Cleaning using cargo"
	@cargo clean
	@echo "Cleaning man dir"
	@rm man/*.1

check:
	@echo "Checking $(NAME)"
	@cargo check

run:
	@echo "Running debug"
	@cargo run

build:
	@echo "Building release"
	@cargo build --release

man: $(GZ_FILES)
	@echo "Manual generated"

$(MAN_DIR)/%.1: $(MAN_DIR)/%.man
	gzip -c $< > $@
	@echo "Compressed: $@"

install: build man
	@echo "Installing executable target"
	@cp target/release/$(NAME) $(PREFIX)/bin
	@echo "Executable file installed to $(PREFIX)/bin/$(NAME)"
	@echo "Installing manual"
	@cp man/*.1 $(PREFIX)/share/man/man1
	@echo "Manual installed to $(PREFIX)/share/man/man1"

uninstall:
	@echo "Removing executable target"
	@rm $(PREFIX)/bin/$(NAME)
	@echo "Removing manual"
	@rm $(PREFIX)/share/man/man1/hina*
	@echo "Successfully uninstalled Hina."
