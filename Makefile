# libthai-idcard Makefile

.DEFAULT_GOAL := help

##@ Development

.PHONY: build
build: ## Build all targets (library + examples)
	cargo build --examples

.PHONY: build-release
build-release: ## Build in release mode
	cargo build --release

.PHONY: example
example: ## Run the Rust usage example (waits for card)
	cargo run --example rust_usage

.PHONY: check
check: ## Check code compiles without building (fast)
	cargo check

.PHONY: fmt
fmt: ## Format all Rust code
	cargo fmt

.PHONY: vet
vet: ## Run clippy lints
	cargo clippy -- -D warnings

.PHONY: test
test: ## Run all tests
	cargo test

##@ C FFI

.PHONY: shared
shared: ## Build the native shared library (.dylib/.so)
	cargo build --lib

.PHONY: headers
headers: ## Generate C header file (requires cbindgen)
	cbindgen --config cbindgen.toml --crate thaiidcard --output thaiidcard.h

.PHONY: install
install: build-release ## Install the shared library system-wide (Linux/macOS)
	sudo cp target/release/libthaiidcard.* /usr/local/lib/
	@-sudo cp thaiidcard.h /usr/local/include/ 2>/dev/null; true

##@ Cross-compilation

.PHONY: build-linux
build-linux: ## Build Linux .so via Docker
	docker build --build-arg TARGET=x86_64-unknown-linux-gnu \
		-f Dockerfile.build -t thaiidcard-builder .
	docker run --rm -v $(PWD):/build thaiidcard-builder
	@ls -lh target/x86_64-unknown-linux-gnu/release/libthaiidcard.so 2>/dev/null || true

.PHONY: build-mac
build-mac: ## Build macOS .dylib via Docker (Apple Silicon)
	docker build --build-arg TARGET=aarch64-apple-darwin \
		-f Dockerfile.build -t thaiidcard-builder .
	docker run --rm -v $(PWD):/build thaiidcard-builder
	@ls -lh target/aarch64-apple-darwin/release/libthaiidcard.dylib 2>/dev/null || true

.PHONY: build-mac-x64
build-mac-x64: ## Build macOS .dylib via Docker (Intel)
	docker build --build-arg TARGET=x86_64-apple-darwin \
		-f Dockerfile.build -t thaiidcard-builder .
	docker run --rm -v $(PWD):/build thaiidcard-builder
	@ls -lh target/x86_64-apple-darwin/release/libthaiidcard.dylib 2>/dev/null || true

.PHONY: build-win
build-win: ## Build Windows DLL via Docker
	docker build --build-arg TARGET=x86_64-pc-windows-gnu \
		-f Dockerfile.build -t thaiidcard-builder .
	docker run --rm -v $(PWD):/build thaiidcard-builder
	@ls -lh target/x86_64-pc-windows-gnu/release/thaiidcard.dll 2>/dev/null || true

.PHONY: build-win-native
build-win-native: ## Build Windows DLL natively (requires mingw-w64)
	cargo build --target x86_64-pc-windows-gnu --lib
	@echo "---"
	@ls -lh target/x86_64-pc-windows-gnu/debug/thaiidcard.dll

.PHONY: build-win-native-release
build-win-native-release: ## Build Windows DLL natively in release mode
	cargo build --target x86_64-pc-windows-gnu --lib --release
	@echo "---"
	@ls -lh target/x86_64-pc-windows-gnu/release/thaiidcard.dll

##@ C Example

.PHONY: c-example
c-example: shared ## Build and run the C usage example
	cc -o /tmp/thaiid-c-example examples/c_usage.c -ldl
	/tmp/thaiid-c-example

.PHONY: c-example-link
c-example-link: shared ## Build C example with direct linking
	cc -o /tmp/thaiid-c-link examples/c_usage.c \
		-Ltarget/debug -lthaiidcard \
		-framework PCSC \
		-Wl,-rpath,target/debug
	/tmp/thaiid-c-link

##@ Go Example

.PHONY: go-example
go-example: shared ## Run the Go usage example
	go run examples/go_usage.go

##@ Python Example

.PHONY: python-example
python-example: shared ## Run the Python usage example
	python3 examples/python_usage.py

##@ Ruby Example

.PHONY: ruby-example
ruby-example: shared ## Run the Ruby usage example
	ruby examples/ruby_usage.rb

##@ Clean

.PHONY: clean
clean: ## Remove build artifacts
	cargo clean
	rm -f thaiidcard.h

##@ Help

.PHONY: help
help: ## Display this help screen
	@grep -E '^[a-zA-Z_-]+:.*?## .*$$' $(MAKEFILE_LIST) | sort | \
		awk 'BEGIN {FS = ":.*?## "}; {printf "\033[36m%-20s\033[0m %s\n", $$1, $$2}'
