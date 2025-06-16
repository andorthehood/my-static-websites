# Default site name (can be overridden)
SITE ?= lepkef.ing

install-hooks:
	cp hooks/pre-commit .git/hooks/pre-commit
	chmod +x .git/hooks/pre-commit

netlify:
	rustup toolchain install stable
	cargo run -- generate $(SITE)

# Coverage targets
coverage:
	cargo tarpaulin --out html --output-dir coverage/

coverage-ci:
	cargo tarpaulin --out xml

# Help target to show usage
help:
	@echo "Available targets:"
	@echo "  install-hooks - Install git hooks for code formatting"
	@echo "  netlify      - Build the site for Netlify deployment"
	@echo "  coverage     - Generate HTML coverage report"
	@echo "  coverage-ci  - Generate XML coverage report for CI"
	@echo "  help         - Show this help message"
	@echo ""
	@echo "Usage:"
	@echo "  make netlify SITE=lepkef.ing    # Build lepkef.ing site (default)"
	@echo "  make netlify SITE=mysite.com    # Build mysite.com site"
	@echo "  make coverage                   # Generate coverage report"

.PHONY: install-hooks netlify coverage coverage-ci help
