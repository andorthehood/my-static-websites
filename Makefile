# Default site name (can be overridden)
SITE ?= lepkef.ing

netlify:
	rustup toolchain install stable
	cargo run -- generate $(SITE)

# Help target to show usage
help:
	@echo "Available targets:"
	@echo "  netlify      - Build the site for Netlify deployment"
	@echo "  help         - Show this help message"
	@echo ""
	@echo "Usage:"
	@echo "  make netlify SITE=lepkef.ing    # Build lepkef.ing site (default)"
	@echo "  make netlify SITE=mysite.com    # Build mysite.com site"

.PHONY: netlify help
