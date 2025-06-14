# Lepkefing - Static Site Generator

A fast, simple static site generator built in Rust.

## Features

- Markdown to HTML conversion
- Liquid template processing
- Handlebars variable replacement
- Automatic pagination
- Static file copying with versioning
- Development server with live reload

## Setup

1. Clone the repository:
   ```bash
   git clone <repository-url>
   cd andor.cool
   ```

2. Install Git hooks (recommended):
   ```bash
   ./scripts/setup-hooks.sh
   ```
   This installs a pre-commit hook that automatically formats your Rust code.

3. Build the project:
   ```bash
   cargo build
   ```

4. Run tests:
   ```bash
   cargo test
   ```

## Usage

Generate a static site:
```bash
cargo run -- generate <site-name>
```

Start development server:
```bash
cargo run -- serve <site-name>
```

Watch for changes:
```bash
cargo run -- watch <site-name>
```

## Development

This project uses `rustfmt` for code formatting. If you set up the Git hooks, your code will be automatically formatted before each commit.

To manually format code:
```bash
cargo fmt
```

To check formatting:
```bash
cargo fmt --check
```

Zero-dep static site generator written in Rust because I was bored and wanted to learn Rust the hard way. 
It has built-in ramdisk support so I don't grind my SSD into dust every time I hit rebuild. 
Regenerates *everything* every time. No caching. Still stupid fast because Rust.


It actually powers my real websites:
- lepkef.ing: [![Netlify Status](https://api.netlify.com/api/v1/badges/a8bd44af-89f0-4afe-8765-f9cfc38191bf/deploy-status)](https://app.netlify.com/sites/andor/deploys)
- polgarhivatal.nl: [![Netlify Status](https://api.netlify.com/api/v1/badges/ea7ae987-302e-4cb0-816f-0aec9b7b5c18/deploy-status)](https://app.netlify.com/projects/polgarhivatal/deploys)

## Requirements

To run the website locally, make sure you have the following installed:
- Git
- Rust

## Site Structure

Each site should be organized in the following structure under `./sites/<site_name>/`:

```