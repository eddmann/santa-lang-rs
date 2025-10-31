IMAGE = rust:1.85.0-bullseye
DOCKER = docker run --rm -e CARGO_HOME=/app/.cargo -v $(PWD):/app -w /app

.DEFAULT_GOAL := help

.PHONY: help
help: ## Display this help message
	@awk 'BEGIN {FS = ":.*##"; printf "\nUsage:\n  make \033[36m<target>\033[0m\n"} /^[a-zA-Z\/_%-]+:.*?##/ { printf "  \033[36m%-20s\033[0m %s\n", $$1, $$2 } /^##@/ { printf "\n\033[1m%s\033[0m\n", substr($$0, 5) } ' $(MAKEFILE_LIST)

##@ Development

.PHONY: shell
shell: ## Interactive shell in Docker build environment
	@$(DOCKER) -it $(IMAGE) bash

.PHONY: fmt
fmt: ## Format all code with rustfmt
	@$(DOCKER) -it $(IMAGE) bash -c "rustup component add rustfmt && cargo fmt"

##@ Testing/Linting

.PHONY: can-release
can-release: lint test ## Run all CI checks (lint + test)

.PHONY: lint
lint: ## Run rustfmt and clippy checks
	@$(DOCKER) $(IMAGE) bash -c "\
		rustup component add rustfmt clippy && \
		cargo fmt -- --check && \
		cargo clippy -- -D warnings"

.PHONY: test
test: test/lang test/cli test/wasm ## Run all tests (lang, CLI, WASM)

.PHONY: test/lang
test/lang: ## Test core language only
	@$(DOCKER) $(IMAGE) cargo test --package santa-lang --verbose

.PHONY: test/cli
test/cli: ## Test CLI only
	@$(DOCKER) $(IMAGE) cargo build --bin santa-cli --verbose
	@$(DOCKER) $(IMAGE) cargo test --bin santa-cli --verbose

.PHONY: test/wasm
test/wasm: ## Test WebAssembly (runs on host machine)
	@echo "Note: test/wasm runs on the host"
	@wasm-pack test --node runtime/wasm

##@ Lambda Runtime

.PHONY: lambda/build
lambda/build: ## Build Lambda runtime
	@docker build -t local/santa-lambda-build -f runtime/lambda/build.Dockerfile .
	@docker run --rm -v $(PWD)/target:/output local/santa-lambda-build cp -r /app/target/lambda /output/

.PHONY: lambda/serve
lambda/serve: ## Serve Lambda runtime locally
	@docker run --rm -it \
		-e DOCKER_LAMBDA_STAY_OPEN=1 \
		-e _HANDLER=fibonacci.handler \
		-p 9001:9001 \
		-v $(PWD)/target/lambda/release/santa-lambda:/opt/bootstrap \
		-v $(PWD)/runtime/lambda/fixtures:/var/task \
		lambci/lambda:provided.al2

.PHONY: lambda/invoke
lambda/invoke: ## Test Lambda endpoint
	@curl -d '{"number": 100}' http://localhost:9001/2015-03-31/functions/myfunction/invocations

##@ PHP Extension

.PHONY: php-ext/build
php-ext/build: ## Build PHP extension
	@docker build -t local/santa-php-ext-build - < runtime/php-ext/build.Dockerfile
	@$(DOCKER) local/santa-php-ext-build bash -c "cargo build --package santa-php-ext --release"

.PHONY: php-ext/test
php-ext/test: ## Test PHP extension
	@docker build -t local/santa-php-ext-build - < runtime/php-ext/build.Dockerfile
	@$(DOCKER) local/santa-php-ext-build bash -c "php -dextension=./target/release/libsanta_lang_php.so runtime/php-ext/fixtures/test.php"

##@ Jupyter Kernel

.PHONY: jupyter/build
jupyter/build: ## Build Jupyter kernel Docker image
	docker build \
		-f runtime/jupyter/build.Dockerfile \
		--label "org.opencontainers.image.source=https://github.com/eddmann/santa-lang-rs" \
		-t ghcr.io/eddmann/santa-lang-jupyter:latest .

.PHONY: jupyter/run
jupyter/run: ## Run Jupyter notebook server
	docker run --rm -it -p 8888:8888 ghcr.io/eddmann/santa-lang-jupyter:latest

##@ CLI

cli/build/%: ## Build CLI for specific target (e.g., cli/build/x86_64-unknown-linux-gnu)
	@$(DOCKER) joseluisq/rust-linux-darwin-builder:1.86.0 \
		sh -c "cargo build --release --bin santa-cli --target $*"
