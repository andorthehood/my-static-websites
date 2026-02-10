# Default site name (can be overridden)
SITE ?= polgarand.org
SESSION ?= polgarand-dev

.PHONY: install-hooks
install-hooks:
	cp hooks/pre-commit .git/hooks/pre-commit
	chmod +x .git/hooks/pre-commit

.PHONY: generate
generate:
	cargo run -- generate $(SITE)

.PHONY: netlify
netlify:
	rustup toolchain install stable
	cargo run -- generate $(SITE)

.PHONY: serve
serve:
	cargo run -- serve $(SITE)

.PHONY: watch
watch:
	cargo run -- watch $(SITE) --ramdisk

.PHONY: polgarand-dev
polgarand-dev:
	tmux new-session -d -s polgarand-dev \; \
	split-window -h \; \
	send-keys -t 0 'cargo run -- watch polgarand.org --ramdisk' Enter \; \
	send-keys -t 1 'cargo run -- serve polgarand.org' Enter \; \
	attach-session -t polgarand-dev

.PHONY: polgarhivatal-dev
polgarhivatal-dev:
	tmux new-session -d -s polgarhivatal-dev \; \
	split-window -h \; \
	send-keys -t 0 'cargo run -- watch polgarhivatal.nl --ramdisk' Enter \; \
	send-keys -t 1 'cargo run -- serve polgarhivatal.nl' Enter \; \
	attach-session -t polgarhivatal-dev

.PHONY: tmux-recover
tmux-recover:
	@session_count=$$(tmux ls -F '#S' 2>/dev/null | wc -l | tr -d ' '); \
	if [ "$$session_count" -eq 0 ]; then \
		echo "No tmux sessions found."; \
		exit 1; \
	fi; \
	if [ -n "$(SESSION)" ]; then \
		tmux attach -t "$(SESSION)"; \
		exit 0; \
	fi; \
	if [ "$$session_count" -eq 1 ]; then \
		tmux attach -t "$$(tmux ls -F '#S')"; \
		exit 0; \
	fi; \
	echo "Multiple tmux sessions found. Re-run with SESSION=<name>:"; \
	tmux ls -F '#S'

.PHONY: tmux-kill-all
tmux-kill-all:
	@if tmux ls >/dev/null 2>&1; then \
		tmux kill-server; \
		echo "Killed all tmux sessions."; \
	else \
		echo "No tmux sessions found."; \
	fi

.PHONY: format
format:
	cargo fmt

.PHONY: lint
lint:
	cargo clippy

.PHONY: lint-pedantic
lint-pedantic:
	cargo clippy -- -W clippy::pedantic

# Coverage targets
.PHONY: coverage
coverage:
	cargo tarpaulin --out html --output-dir coverage/ -- --test-threads=1

.PHONY: coverage-ci
coverage-ci:
	cargo tarpaulin --out xml -- --test-threads=1

.PHONY: test
test:
	cargo test -- --test-threads=1

.PHONY: bookmarks-favicons
bookmarks-favicons:
	python3 sites/polgarand.org/scripts/download_favicons.py

# Help target to show usage
.PHONY: help
help:
	@echo "Available targets:"
	@echo "  install-hooks     - Install git hooks for code formatting"
	@echo "  generate          - Generate the site"
	@echo "  netlify           - Build the site for Netlify deployment"
	@echo "  serve             - Start the development server"
	@echo "  lepkefing-dev     - Start watch and serve in tmux split view"
	@echo "  polgarhivatal-dev - Start watch and serve in tmux split view"
	@echo "  bookmarks-favicons - Download favicons for bookmarks.json (polgarand.org)"
	@echo "  format            - Format the code"
	@echo "  lint              - Lint the code"
	@echo "  lint-pedantic     - Lint the code with pedantic checks"
	@echo "  coverage          - Generate HTML coverage report"
	@echo "  coverage-ci       - Generate XML coverage report for CI"
	@echo "  tmux-recover      - Attach to an existing tmux session"
	@echo "  tmux-kill-all     - Kill all tmux sessions"
	@echo "  help              - Show this help message"
	@echo ""
	@echo "Usage:"
	@echo "  make generate SITE=polgarand.org   # Generate polgarand.org site (default)"
	@echo "  make netlify SITE=polgarand.org    # Build polgarand.org site (default)"
	@echo "  make netlify SITE=mysite.com    # Build mysite.com site"
	@echo "  make serve                      # Start the development server"
	@echo "  make format                     # Format the code"
	@echo "  make lint                       # Lint the code"
	@echo "  make lint-pedantic              # Lint the code with pedantic checks"
	@echo "  make coverage                   # Generate coverage report"
	@echo "  make tmux-recover SESSION=polgarand-dev  # Attach to a tmux session"
	@echo "  make tmux-kill-all              # Kill all tmux sessions"
