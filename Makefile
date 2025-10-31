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

# Benchmark configuration
BENCH_IMAGE = santa-lang-benchmark
BENCH_DOCKER = docker run --rm \
	-v $(PWD):/workspace \
	-v $(PWD)/benchmarks/results:/results \
	-w /workspace \
	-e CARGO_HOME=/tmp/cargo \
	$(BENCH_IMAGE)
BENCH_TIMESTAMP = $(shell date +%Y%m%d_%H%M%S)

.PHONY: bench/build
bench/build:
	@echo "Building benchmark Docker image..."
	@docker build -t $(BENCH_IMAGE) -f benchmarks/Dockerfile benchmarks

.PHONY: bench/shell
bench/shell:
	@docker run --rm -it \
		-v $(PWD):/workspace \
		-w /workspace \
		-e CARGO_HOME=/tmp/cargo \
		$(BENCH_IMAGE) bash

.PHONY: bench/criterion
bench/criterion:
	@echo "Running Criterion microbenchmarks in Docker..."
	@mkdir -p benchmarks/results
	@$(BENCH_DOCKER) bash -c "cargo bench --package santa-lang-benchmarks"

.PHONY: bench/run
bench/run:
	@echo "Running hyperfine benchmarks in Docker..."
	@mkdir -p benchmarks/results
	@$(BENCH_DOCKER) bash -c ' \
		echo "Building santa-cli..." && \
		cargo build --release --bin santa-cli --quiet && \
		echo "" && \
		echo "Running benchmarks..." && \
		hyperfine \
			--warmup 3 \
			--runs 10 \
			--export-json /results/benchmark_$(BENCH_TIMESTAMP).json \
			--export-markdown /results/benchmark_$(BENCH_TIMESTAMP).md \
			--command-name "empty" "./target/release/santa-cli benchmarks/fixtures/empty.santa" \
			--command-name "fibonacci" "./target/release/santa-cli benchmarks/fixtures/fibonacci.santa" \
			--command-name "list_processing" "./target/release/santa-cli benchmarks/fixtures/list_processing.santa" \
			--command-name "pattern_matching" "./target/release/santa-cli benchmarks/fixtures/pattern_matching.santa" \
	'
	@echo ""
	@echo "Results saved to: benchmarks/results/benchmark_$(BENCH_TIMESTAMP).*"

.PHONY: bench/compare
bench/compare:
	@if [ -z "$(V1)" ] || [ -z "$(V2)" ]; then \
		echo "Usage: make bench/compare V1=main V2=HEAD"; \
		echo "Example: make bench/compare V1=v1.0.0 V2=feature-branch"; \
		exit 1; \
	fi
	@echo "Comparing $(V1) vs $(V2) in Docker..."
	@mkdir -p benchmarks/results/compare_$(BENCH_TIMESTAMP)
	@echo "Preserving benchmark fixtures..."
	@if [ -d "benchmarks/fixtures" ]; then \
		cp -r benchmarks/fixtures /tmp/bench_fixtures_$(BENCH_TIMESTAMP); \
	else \
		echo "Error: benchmarks/fixtures directory not found"; \
		exit 1; \
	fi
	@echo "Benchmarking $(V1)..."
	@git checkout $(V1) 2>/dev/null || (echo "Failed to checkout $(V1)" && exit 1)
	@mkdir -p benchmarks/fixtures
	@cp -r /tmp/bench_fixtures_$(BENCH_TIMESTAMP)/* benchmarks/fixtures/
	@$(BENCH_DOCKER) bash -c ' \
		cargo build --release --bin santa-cli --quiet && \
		for fixture in benchmarks/fixtures/*.santa; do \
			name=$$(basename $$fixture .santa); \
			echo "  $$name..."; \
			hyperfine --warmup 3 --runs 10 \
				--export-json /results/compare_$(BENCH_TIMESTAMP)/$${name}_v1.json \
				"./target/release/santa-cli $$fixture"; \
		done \
	'
	@echo "Benchmarking $(V2)..."
	@git checkout $(V2) 2>/dev/null || (echo "Failed to checkout $(V2)" && exit 1)
	@mkdir -p benchmarks/fixtures
	@cp -r /tmp/bench_fixtures_$(BENCH_TIMESTAMP)/* benchmarks/fixtures/
	@$(BENCH_DOCKER) bash -c ' \
		cargo build --release --bin santa-cli --quiet && \
		for fixture in benchmarks/fixtures/*.santa; do \
			name=$$(basename $$fixture .santa); \
			echo "  $$name..."; \
			hyperfine --warmup 3 --runs 10 \
				--export-json /results/compare_$(BENCH_TIMESTAMP)/$${name}_v2.json \
				"./target/release/santa-cli $$fixture"; \
		done \
	'
	@rm -rf /tmp/bench_fixtures_$(BENCH_TIMESTAMP)
	@git checkout - >/dev/null 2>&1
	@echo ""
	@echo "Generating comparison report..."
	@docker run --rm \
		-v $(PWD)/benchmarks:/benchmarks \
		$(BENCH_IMAGE) python3 /benchmarks/scripts/compare_results.py \
		/benchmarks/results/compare_$(BENCH_TIMESTAMP) \
		$(V1) $(V2) > benchmarks/results/compare_$(BENCH_TIMESTAMP)/comparison.txt
	@cat benchmarks/results/compare_$(BENCH_TIMESTAMP)/comparison.txt
	@echo ""
	@echo "Generating comparison chart..."
	@mkdir -p benchmarks/results/compare_$(BENCH_TIMESTAMP)/charts
	@docker run --rm \
		-v $(PWD)/benchmarks:/benchmarks \
		$(BENCH_IMAGE) python3 /benchmarks/scripts/visualize_results.py \
		/benchmarks/results/compare_$(BENCH_TIMESTAMP)/*_v1.json \
		/benchmarks/results/compare_$(BENCH_TIMESTAMP)/*_v2.json \
		--output /benchmarks/results/compare_$(BENCH_TIMESTAMP)/charts \
		--labels "$(V1)" "$(V2)" 2>/dev/null || echo "Chart generation skipped (matplotlib may not be available)"
	@echo ""
	@echo "Results saved to: benchmarks/results/compare_$(BENCH_TIMESTAMP)/"

.PHONY: bench/visualize
bench/visualize:
	@if [ -z "$(RESULTS)" ]; then \
		echo "Usage: make bench/visualize RESULTS=benchmarks/results/benchmark_*.json"; \
		echo "   or: make bench/visualize RESULTS='benchmarks/results/base.json benchmarks/results/pr.json'"; \
		exit 1; \
	fi
	@echo "Generating visualization..."
	@mkdir -p benchmarks/results/charts
	@docker run --rm \
		-v $(PWD)/benchmarks:/benchmarks \
		$(BENCH_IMAGE) python3 /benchmarks/scripts/visualize_results.py \
		$(RESULTS) --output /benchmarks/results/charts
	@echo "Charts saved to: benchmarks/results/charts/"

.PHONY: bench/clean
bench/clean:
	@echo "Cleaning benchmark results..."
	@rm -rf benchmarks/results/*
	@echo "Results cleaned."

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
