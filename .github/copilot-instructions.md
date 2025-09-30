# lepkefing - Static Site Generator

**ALWAYS follow these instructions first. Only fall back to search or bash commands when you encounter unexpected information that does not match these instructions.**

lepkefing is a zero-dependency static site generator written in Rust. It regenerates everything on each build (no caching) but remains fast due to Rust's performance. It powers real websites like polgarand.org, polgarhivatal.nl, and 8f4e.com deployed on Netlify.

## Working Effectively

### Initial Setup and Build
Run these commands in order to bootstrap the repository:

```bash
# Install git hooks for automatic code formatting
make install-hooks

# Build the project - takes ~30 seconds first time
cargo build
```

**NEVER CANCEL: First build takes ~30 seconds. Set timeout to 120+ seconds.**

### Testing
```bash
# Run all tests - CRITICAL: Must use --test-threads=1 due to file I/O conflicts
cargo test -- --test-threads=1
```

**NEVER CANCEL: Tests take ~10 seconds. Set timeout to 60+ seconds.**

**CRITICAL TEST REQUIREMENT**: Always use `--test-threads=1` when running tests. Parallel tests cause race conditions in the shared `out/` directory.

### Release Build
```bash
# Build optimized release binary - takes ~3 seconds
cargo build --release
```

**NEVER CANCEL: Release build takes ~5 seconds. Set timeout to 60+ seconds.**

## Development Workflow

### Generate Static Sites
```bash
# Generate a specific site (very fast - ~5ms for test site, ~1.7s for polgarand.org)
cargo run -- generate <site_name>

# Examples:
cargo run -- generate test
cargo run -- generate polgarand.org
cargo run -- generate polgarhivatal.nl
```

### Development Server
```bash
# Start development server on http://localhost:2030
cargo run -- serve <site_name>

# Example:
cargo run -- serve test
```

The server runs on `localhost:2030` and serves the generated site files.

### Auto-Regeneration (Watch Mode)
```bash
# Basic watch mode - monitors ./sites/<site_name>/ for changes
cargo run -- watch <site_name>

# Watch with RAM disk (Linux only) - prevents SSD wear during development
cargo run -- watch <site_name> --ramdisk
```

**RAM Disk Feature**: The `--ramdisk` flag uses `/dev/shm` on Linux to store generated files in RAM instead of on disk, preventing SSD wear during active development.

### Advanced Development with tmux
```bash
# Start split-screen development for polgarand.org (watch + serve)
make lepkefing-dev

# Start split-screen development for polgarhivatal.nl
make polgarhivatal-dev
```

These commands create tmux sessions with watch in one pane and serve in another.

## Code Quality and Validation

### Formatting and Linting
```bash
# Format code (fast ~1 second)
make format
# OR: cargo fmt

# Basic linting (~3 seconds)
make lint
# OR: cargo clippy

# Pedantic linting with extra warnings (~1 second)
make lint-pedantic
# OR: cargo clippy -- -W clippy::pedantic
```

**ALWAYS run formatting and linting before committing changes or CI will fail.**

### Coverage Reports
```bash
# Generate HTML coverage report - MUST install cargo-tarpaulin first
cargo install cargo-tarpaulin  # Takes ~4 minutes first time
make coverage
# OR: cargo tarpaulin --out html --output-dir coverage/ -- --test-threads=1
```

**NEVER CANCEL: cargo-tarpaulin installation takes ~4 minutes. Coverage generation takes ~10 seconds. Set timeout to 300+ seconds for installation.**

Coverage reports are generated in `./coverage/tarpaulin-report.html`.

## Validation Scenarios

**ALWAYS run these validation scenarios after making changes:**

### 1. Build and Test Validation
```bash
# CRITICAL: Use proper timeout and threading
cargo build                                    # ~30s first time, ~1s incremental
cargo test -- --test-threads=1               # ~10s - NEVER use parallel tests
```

### 2. Site Generation Validation
```bash
# Test site generation with different sites
cargo run -- generate test                   # Fast test site (~5ms)
cargo run -- generate polgarand.org            # Real site (~1.7s)

# Verify output structure
ls -la out/test/                             # Should contain .html, .json, assets/, posts/
```

### 3. Development Server Validation
```bash
# Start server and test HTTP response
cargo run -- serve test &
curl -s http://localhost:2030 | head -5     # Should return HTML content
# Stop the server with Ctrl+C or kill
```

### 4. Watch Mode Validation
```bash
# Test basic watch mode
cargo run -- watch test &
# Touch a file in sites/test/ and verify regeneration message
# Stop with Ctrl+C

# Test RAM disk mode (Linux only)
cargo run -- watch test --ramdisk &
ls -la /dev/shm/ | grep lepkefing           # Should show lepkefing_out directory
# Stop with Ctrl+C
```

## Repository Structure

### Key Directories
- `src/` - Rust source code
  - `src/main.rs` - CLI entry point and command handling
  - `src/generate.rs` - Main site generation orchestrator
  - `src/template_processors/` - Liquid, Handlebars, Markdown processing
  - `src/server/` - Development server implementation
  - `src/watch.rs` - File watching with RAM disk support
  - `src/minifier/` - HTML, CSS, JS minification
- `sites/` - Site content directories
  - `sites/test/` - Test site for development and testing
  - `sites/polgarand.org/` - Production site example
- `out/` - Generated static files (created during generation)
- `hooks/` - Git hooks for code formatting
- `.github/workflows/` - CI/CD pipeline

### Site Structure (under `sites/<site_name>/`)
```
sites/<site_name>/
├── config.md       # Site configuration with YAML front matter
├── posts/          # Blog posts (.md or .liquid files)
├── pages/          # Static pages (.md or .liquid files)
├── layouts/        # HTML templates (main.html, etc.)
├── includes/       # Liquid template partials
├── assets/         # Static files (CSS, images, JS, SCSS, TypeScript)
└── data/          # JSON data files exposed as data.* variables
```

## Common Issues and Solutions

### Test Failures
- **Always use `--test-threads=1`** - Parallel tests fail due to shared file I/O
- If tests fail unexpectedly, clean the `out/` directory: `rm -rf out/`

### Build Issues
- If incremental build fails, clean and rebuild: `cargo clean && cargo build`
- For release issues, try: `cargo clean && cargo build --release`

### Site Generation Issues
- Verify site directory exists: `ls -la sites/<site_name>/`
- Check for required files: `config.md` in site root
- Warning messages about missing layouts or CSS files are non-fatal

### Development Server Issues
- Default port is 2030: `http://localhost:2030`
- Generate the site first before serving: `cargo run -- generate <site_name>`
- Server process must be stopped (Ctrl+C) before starting a new one

## Timing Expectations

All timings are measured on typical development hardware:

- **First build**: ~30 seconds (downloading dependencies)
- **Incremental build**: ~1 second
- **Release build**: ~3 seconds  
- **Test suite**: ~10 seconds (with `--test-threads=1`)
- **Test site generation**: ~5 milliseconds
- **polgarand.org generation**: ~1.7 seconds
- **Development server startup**: ~1 second
- **Linting**: ~3 seconds
- **Coverage generation**: ~10 seconds (after tarpaulin installed)
- **cargo-tarpaulin installation**: ~4 minutes (one-time)

## Docker Support

The repository includes Docker support for containerized builds:

```bash
# Build Docker image (runs tests and generates sites)
docker build -t lepkefing .

# Run with docker-compose
docker-compose up
```

**NEVER CANCEL: Docker build takes several minutes. Set timeout to 600+ seconds.**

## Features and Capabilities

- **Zero runtime dependencies** - Pure Rust implementation
- **Partial TypeScript support** - Strips types and minifies to JS
- **Partial SCSS support** - Inlines imports and flattens nesting
- **Liquid template support** - `{% if %}`, `{% for %}`, `{% assign %}`, `{% render %}`
- **Markdown processing** - Converts `.md` files to HTML
- **Asset minification** - HTML, CSS, JS optimization
- **Asset versioning** - Content hashes for cache busting
- **JSON API** - Generates `.json` files alongside `.html` for client-side routing
- **Sitemap generation** - Create `sitemap.xml.liquid` in pages directory
- **Data loading** - JSON files in `data/` exposed as template variables

## CLI Commands Reference

```bash
# Show available commands
cargo run

# Generate static site
cargo run -- generate <site_name>

# Start development server  
cargo run -- serve <site_name>

# Watch for changes and auto-regenerate
cargo run -- watch <site_name>

# Watch with RAM disk (Linux only)
cargo run -- watch <site_name> --ramdisk
```

**Command validation**: Always test CLI changes by running `cargo run` without arguments to see help output.