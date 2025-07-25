# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

**lepkefing** is a zero-dependency static site generator written in Rust. It regenerates everything on each build (no caching) but remains fast due to Rust's performance. Currently powers lepkef.ing and polgarhivatal.nl with Netlify deployment.

## Essential Commands

### Development
```bash
# Build the project
cargo build

# Generate a specific site
cargo run -- generate <site_name>
cargo run -- generate lepkef.ing

# Development server
cargo run -- serve

# Auto-regeneration with file watching
cargo run -- watch <site_name>
cargo run -- watch <site_name> --ramdisk  # Linux only - uses RAM to prevent SSD wear
```

### Testing
```bash
# Run tests (IMPORTANT: Must use single thread due to file I/O conflicts)
cargo test -- --test-threads=1
make test

# Run single test
cargo test test_name -- --test-threads=1

# Test coverage
make coverage
```

### Code Quality
```bash
# Format code
cargo fmt
make format

# Lint
cargo clippy
make lint-pedantic
```

## Architecture Overview

### Core Data Flow
1. **Input**: Site directories in `./sites/<site_name>/` containing posts, pages, layouts, includes, and assets
2. **Processing**: Unified template pipeline processes Liquid conditionals → Liquid includes → Markdown → Handlebars variables
3. **Output**: Generated site in `./out/` directory

### Key Modules
- **`src/generate.rs`**: Main generation orchestrator
- **`src/template_processors/`**: Modular template processing (Liquid, Handlebars, Markdown)
- **`src/file_readers.rs`**: Content loading and front matter parsing
- **`src/server/`**: Development server
- **`src/watch.rs`**: File watching with optional RAM disk support

### Data Structures
- **ContentItem**: `HashMap<String, String>` - Single post/page with metadata from YAML front matter
- **ContentCollection**: `Vec<ContentItem>` - Collections of posts/pages
- Site configuration loaded from `sites/<site>/config.md` with YAML front matter

### Site Structure
```
sites/<site_name>/
├── posts/          # Blog posts (.md or .hbs files)
├── pages/          # Static pages (.md or .hbs files) 
├── layouts/        # HTML templates
├── includes/       # Liquid template partials
├── assets/         # Static files (CSS, images, etc.)
└── config.md       # Site configuration with YAML front matter
```

## Development Notes

### Testing Requirements
- **CRITICAL**: All tests must run with `--test-threads=1` due to file I/O conflicts between parallel tests
- Uses `insta` for snapshot testing - accept new snapshots with `cargo insta accept`
- Test files write to shared `out/` directory causing race conditions in parallel execution

### Template Processing Pipeline
Templates are processed in this exact order:
1. Liquid conditionals (`{% if %}` tags)
2. Liquid includes (`{% include %}` tags) 
3. Markdown to HTML conversion (if `.md` file)
4. Handlebars variable substitution (`{{variable}}`)

### Special Features
- **RAM disk support**: Linux-only `--ramdisk` flag uses `/dev/shm` for output during development
- **Asset versioning**: Adds content hashes to asset filenames for cache busting
- **Front matter parsing**: YAML metadata in all content files drives rendering
- **Zero runtime dependencies**: Pure Rust implementation

### Code Quality Standards
- Uses Clippy with pedantic lints for code quality
- Pre-commit hooks enforce formatting
- Comprehensive test coverage with `cargo-tarpaulin`
- Modern Rust formatting (inline format args, not `format!("text {}", var)`)

## Deployment

- **Netlify**: Use `make netlify SITE=<site_name>` for production builds
- **Local preview**: Use `cargo run -- serve` after generation
- Single binary deployment with no runtime dependencies