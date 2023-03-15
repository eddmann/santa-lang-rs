IMAGE = rust:1.68.0-bullseye
DOCKER = docker run --rm -e CARGO_HOME=/app/.cargo -v $(PWD):/app -w /app

.PHONY: shell
shell:
	@$(DOCKER) -it $(IMAGE) bash

.PHONY: lint
lint:
	@$(DOCKER) -it $(IMAGE) bash -c "\
		rustup component add rustfmt clippy && \
		cargo fmt -- --check && \
		cargo clippy -- -D warnings"

.PHONY: test
test:
	@$(DOCKER) $(IMAGE) cargo test --verbose

.PHONY: fmt
fmt:
	@$(DOCKER) -it $(IMAGE) bash -c "rustup component add rustfmt && cargo fmt"
