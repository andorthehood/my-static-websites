# Default site name (can be overridden)
SITE ?= lepkef.ing

install-hooks:
	cp hooks/pre-commit .git/hooks/pre-commit
	chmod +x .git/hooks/pre-commit

generate:
	cargo run -- generate $(SITE)

netlify:
	rustup toolchain install stable
	cargo run -- generate $(SITE)

serve:
	cargo run -- serve

watch:
	cargo run -- watch $(SITE) --ramdisk

dev:
	tmux new-session -d -s lepkef-dev \; \
	split-window -h \; \
	send-keys -t 0 'cargo run -- watch lepkef.ing --ramdisk' Enter \; \
	send-keys -t 1 'cargo run -- serve' Enter \; \
	attach-session -t lepkef-dev

format:
	cargo fmt

lint:
	cargo clippy

lint-pedantic:
	cargo clippy -- -W clippy::pedantic

# Coverage targets
coverage:
	cargo tarpaulin --out html --output-dir coverage/ -- --test-threads=1

coverage-ci:
	cargo tarpaulin --out xml -- --test-threads=1

test:
	cargo test -- --test-threads=1

# Help target to show usage
help:
	@echo "Available targets:"
	@echo "  install-hooks - Install git hooks for code formatting"
	@echo "  generate      - Generate the site"
	@echo "  netlify       - Build the site for Netlify deployment"
	@echo "  serve         - Start the development server"
	@echo "  dev           - Start watch and serve in tmux split view"
	@echo "  format        - Format the code"
	@echo "  lint          - Lint the code"
	@echo "  lint-pedantic - Lint the code with pedantic checks"
	@echo "  coverage      - Generate HTML coverage report"
	@echo "  coverage-ci   - Generate XML coverage report for CI"
	@echo "  help          - Show this help message"
	@echo ""
	@echo "Usage:"
	@echo "  make generate SITE=lepkef.ing   # Generate lepkef.ing site (default)"
	@echo "  make netlify SITE=lepkef.ing    # Build lepkef.ing site (default)"
	@echo "  make netlify SITE=mysite.com    # Build mysite.com site"
	@echo "  make serve                      # Start the development server"
	@echo "  make format                     # Format the code"
	@echo "  make lint                       # Lint the code"
	@echo "  make lint-pedantic              # Lint the code with pedantic checks"
	@echo "  make coverage                   # Generate coverage report"

.PHONY: install-hooks generate netlify serve dev format lint lint-pedantic coverage coverage-ci help
