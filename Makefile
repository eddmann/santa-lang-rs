IMAGE = rust:1.68.0-bullseye
DOCKER = docker run --rm -e CARGO_HOME=/app/.cargo -v $(PWD):/app -w /app

.PHONY: shell
shell:
	@$(DOCKER) -it $(IMAGE) bash

.PHONY: can-release
can-release: lint test

.PHONY: lint
lint:
	@$(DOCKER) $(IMAGE) bash -c "\
		rustup component add rustfmt clippy && \
		cargo fmt -- --check && \
		cargo clippy -- -D warnings"

.PHONY: test
test:
	@$(DOCKER) $(IMAGE) cargo test --verbose

.PHONY: fmt
fmt:
	@$(DOCKER) -it $(IMAGE) bash -c "rustup component add rustfmt && cargo fmt"

# https://dmitryfrank.com/articles/js_closures
