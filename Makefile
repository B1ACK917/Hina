NAME=hina
EXEC=hina
PREFIX=/usr/local

default: build

ifeq ($(shell command -v cargo 2> /dev/null),)
    $(error "cargo is not installed.")
endif

clean:
	@echo "Cleaning build dir"
	@rm -rf target/*
	@echo "Cleaning using cargo"
	@cargo clean
check:
	@echo "Checking $(NAME)"
	@cargo check
build:
	@echo "Building release"
	@cargo build --release
run:
	@echo "Running debug"
	@cargo run
install: build
	@echo "Installing release: $(VERSION)"
	@cp target/release/$(EXEC) $(PREFIX)/bin