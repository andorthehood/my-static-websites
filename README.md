[![Netlify Status](https://api.netlify.com/api/v1/badges/a8bd44af-89f0-4afe-8765-f9cfc38191bf/deploy-status)](https://app.netlify.com/sites/andor/deploys)

Hello! 👋 You've found the source code for the static site generator I built for my websites. I wrote it in Rust as a practice project, which is why it has zero dependencies.

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

```
sites/
└── your-site-name/
    ├── posts/          # Blog posts (Markdown files)
    ├── pages/          # Static pages (Markdown files)
    ├── includes/       # Template includes (Liquid files)
    ├── layouts/        # Page layouts (HTML templates)
    └── style.css       # Site stylesheet
```

The generated output will be placed in the `./out/` directory.
