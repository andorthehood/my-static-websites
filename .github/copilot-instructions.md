# GitHub Copilot Instructions

This repository contains **lepkefing**, a zero-dependency static site generator written in Rust. This document provides guidance for GitHub Copilot when working with this codebase.

## Project Overview

lepkefing is a fast, zero-dependency static site generator that powers real websites like lepkef.ing and polgarhivatal.nl. It regenerates everything on each build (no caching) but remains performant due to Rust's efficiency.

### Key Characteristics
- **Zero external dependencies** - Only use Rust standard library
- **Fast regeneration** - No caching, but optimized for speed
- **Template pipeline** - Liquid conditionals → Liquid includes → Markdown → Handlebars
- **Asset processing** - TypeScript stripping, SCSS compilation, minification
- **Development features** - File watching, RAM disk support, live server

## Critical Requirements

### 🚨 Testing Constraints
**ALWAYS run tests with single threading due to file I/O conflicts:**
```bash
cargo test -- --test-threads=1
```
Tests write to a shared `out/` directory causing race conditions in parallel execution.

### 🚨 Dependency Policy
**NEVER add external dependencies.** Only use Rust standard library. This is a core design principle.

### 🚨 Build Warnings
**Never ignore build or test warnings.** Address all warnings before completing changes.

## Essential Commands

### Development Workflow
```bash
# Build project
cargo build

# Generate a site
cargo run -- generate <site_name>
cargo run -- generate lepkef.ing

# Development server with auto-reload
cargo run -- serve

# File watching with auto-regeneration
cargo run -- watch <site_name>
cargo run -- watch <site_name> --ramdisk  # Linux only
```

### Testing & Quality
```bash
# Run tests (REQUIRED: single-threaded)
cargo test -- --test-threads=1
make test

# Code formatting
cargo fmt
make format

# Linting
cargo clippy
make lint-pedantic

# Test coverage
make coverage
```

## Architecture & Code Organization

### Core Data Structures
- **ContentItem**: `HashMap<String, String>` - Post/page with YAML front matter metadata
- **ContentCollection**: `Vec<ContentItem>` - Collections of content
- **Site config**: Loaded from `sites/<site>/config.md` with YAML front matter

### Key Modules
- `src/generate.rs` - Main generation orchestrator
- `src/template_processors/` - Liquid, Handlebars, Markdown processing
- `src/file_readers.rs` - Content loading and front matter parsing
- `src/file_copier.rs` - Asset copying with versioning and minification
- `src/server/` - Development server
- `src/watch.rs` - File watching with optional RAM disk
- `src/minifier/` - HTML, CSS, JS minification

### Template Processing Pipeline
Process templates in this exact order:
1. Liquid conditionals (`{% if %}`, `{% unless %}`)
2. Liquid includes (`{% include %}`)
3. Markdown to HTML (if `.md` file)
4. Handlebars variables (`{{variable}}`)

## Supported Features

### Liquid Template Features
- **Conditionals**: `{% if %}`, `{% unless %}` with boolean evaluation
- **Loops**: `{% for %}` with optional `limit:N` parameter
- **Assignment**: `{% assign variable = value %}` with `where` filter
- **Includes**: `{% include template.liquid %}` with parameters
- **Variables**: `forloop.index`, `forloop.length`, etc.

### Asset Processing
- **TypeScript**: Strip types and minify to JavaScript
- **SCSS**: Inline local `@use`/`@import`, flatten nesting (no variables/mixins)
- **Minification**: HTML, CSS, JavaScript optimization
- **Versioning**: Content-based hash suffixes for cache busting

### Site Structure
```
sites/<site_name>/
├── posts/          # Blog posts (.md or .liquid)
├── pages/          # Static pages (.md or .liquid)
├── layouts/        # HTML templates
├── includes/       # Liquid template partials
├── assets/         # Static files (CSS, JS, images)
├── data/           # JSON data files (exposed as data.* variables)
└── config.md       # Site configuration
```

## Code Style & Conventions

### Rust Style
- Use modern Rust formatting (inline format args: `format!("text {var}")`)
- Prefer `?` operator for error handling
- Use `clippy::pedantic` lints
- Follow existing patterns in the codebase

### Error Handling
- Return `Result<T, Box<dyn std::error::Error>>` for fallible functions
- Use descriptive error messages
- Don't ignore warnings or errors

### Testing
- Use `insta` for snapshot testing - accept with `cargo insta accept`
- Test files should be focused and clear
- Always test error conditions
- Remember: **single-threaded execution only**

## Special Considerations

### File I/O
- All generated output goes to `./out/` directory
- Use temporary directories for intermediate processing
- Be aware of shared file access in tests

### Platform Support
- RAM disk feature (`--ramdisk`) is Linux-only, falls back gracefully
- Use cross-platform file paths with `std::path`

### Performance
- No caching by design - optimize for fast regeneration
- Minimize file I/O operations
- Use efficient string processing for templates

## Development Best Practices

### Before Making Changes
1. Build with `cargo build` to check current state
2. Run tests with `cargo test -- --test-threads=1`
3. Check for existing similar implementations

### When Adding Features
1. Follow existing patterns in the codebase
2. Add comprehensive tests (remember single-threading)
3. Update documentation if interface changes
4. Test with real site generation: `cargo run -- generate lepkef.ing`

### When Fixing Bugs
1. Write a failing test first
2. Make minimal changes to fix the issue
3. Ensure all tests pass
4. Check that fix doesn't break existing functionality

## Common Patterns

### Content Processing
```rust
// Reading content with front matter
let content = read_content_with_front_matter(&path)?;
let title = content.get("title").unwrap_or("Untitled");

// Template processing pipeline
let processed = process_liquid_conditionals(&input, &data)?;
let processed = process_liquid_includes(&processed, &includes)?;
let processed = markdown_to_html(&processed)?;
let processed = process_handlebars(&processed, &data)?;
```

### File Operations
```rust
// Safe file copying with error handling
fn copy_file_with_versioning(source: &str, dest_dir: &str) -> Result<String, Box<dyn std::error::Error>> {
    // Implementation follows existing pattern
}
```

### Testing Patterns
```rust
#[test]
fn test_feature() {
    let temp_dir = TempDir::new().unwrap();
    // Setup test data
    // Execute function
    // Assert results
    // Clean up handled by TempDir drop
}
```

This guidance ensures Copilot suggestions align with the project's architecture, constraints, and conventions.