Zero-dep static site generator written in Rust. I originally built it to learn Rust, now it's my playground for testing AI coding tools. It regenerates everything on each build (no caching), but still stupid fast thanks to Rust. Includes built-in ramdisk support so I don't wear out my SSD.

It powers my real websites:
- [lepkef.ing](https://lepkef.ing): [![Netlify Status](https://api.netlify.com/api/v1/badges/a8bd44af-89f0-4afe-8765-f9cfc38191bf/deploy-status)](https://app.netlify.com/sites/andor/deploys)
- [polgarhivatal.nl](https://polgarhivatal.nl): [![Netlify Status](https://api.netlify.com/api/v1/badges/ea7ae987-302e-4cb0-816f-0aec9b7b5c18/deploy-status)](https://app.netlify.com/projects/polgarhivatal/deploys)

## Features

- Partial TypeScript support: `.ts` assets are stripped of types and minified to `.js` (interfaces, simple generics, casts, and type annotations removed).
- Partial SCSS support: local `@use`/`@import` are inlined and simple nesting flattened; variables and mixins are not supported.
- Generates `.json` files alongside `.html` pages (content/title/css) to enable client-side routing.
- Minifies HTML, JS, and CSS.
- Parses `.json` files under `sites/<site>/data/` and exposes them to templates via `data.*` variables (e.g., `data.navigation.0.name`).
- Partial Liquid support: `include`, `assign` (with `where` filter), `for` (with `limit`, `forloop` vars like `first`, `last`, `index`, `index0`, `length`), `if`, `unless`, and basic `{{ variable }}` replacement.


## Requirements

To run the website locally, make sure you have the following installed:
- Git
- Rust

## Usage

To get the website running on your local machine, follow these steps:

### Clone the repository
```bash
git clone https://github.com/hngrhorace/my-static-websites.git
cd my-static-websites
```

### Quick Setup (Recommended)
```bash
make setup
```
This will install git hooks, build the project, and set up everything you need for development.

**Alternative manual setup:**
```bash
./scripts/setup-hooks.sh  # Install git hooks
cargo build              # Build the project
```

### Generate a site
```bash
cargo run generate <site_name>
```

This will generate the static files for the specified site. The site content should be located in `./sites/<site_name>/`.

Example:
```bash
cargo run generate lepkef.ing
```

### Development with auto-regeneration
```bash
# Basic watch mode
cargo run watch <site_name>

# Watch mode with RAM-based output (Linux only)
cargo run watch <site_name> --ramdisk
```

This starts watching your site's directory for changes and automatically regenerates the site when files are modified. 

The `--ramdisk` flag enables storing generated files in RAM instead of on disk, which can help prevent SSD wear during development. This feature is only available on Linux systems and will automatically fall back to regular disk storage on other operating systems.

### Start development server
```bash
cargo run serve
```

This starts a local development server to preview your generated site.

## Site Structure

Each site should be organized in the following structure under `./sites/<site_name>/`:

```sites/
└── your-site-name/
    ├── posts/          # Blog posts (Markdown files)
    ├── pages/          # Static pages (Markdown files)
    ├── includes/       # Template includes (Liquid files)
    ├── layouts/        # Page layouts (HTML templates)
    └── style.css       # Site stylesheet
```

The generated output will be placed in the `./out/` directory.

