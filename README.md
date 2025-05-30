[![Netlify Status](https://api.netlify.com/api/v1/badges/a8bd44af-89f0-4afe-8765-f9cfc38191bf/deploy-status)](https://app.netlify.com/sites/andor/deploys)

Hello there! 👋 You just found the source code of my personal website. It's powered by a static site generator I wrote in Rust, without using any third-party libraries. This whole project is a playground for me to improve my "rusty" Rust skills. Originally, I wanted to write it in C, but Netlify still doesn't support deploying C projects. :( 

For storing the content, I'm using Markdown files with Front Matter, and with Liquid and Handlebars templates. I was inspired by Jekyll, which uses a similar structure for content management.

The design was inspired by Mac OS 9, you can steal the CSS from my codepen: [https://codepen.io/hngrhorace/pen/gbYKpxE](https://codepen.io/hngrhorace/pen/gbYKpxE)

## Requirements

To run the website locally, make sure you have the following installed:
- Git
- Rust

## Usage

To get the website running on your local machine, follow these steps:

### Clone the repository
```bash
git clone https://github.com/hngrhorace/lepkef.ing.git
cd lepkef.ing
```

### Generate a site
```bash
cargo run -- generate <site_name>
```

This will generate the static files for the specified site. The site content should be located in `./sites/<site_name>/`.

Example:
```bash
cargo run -- generate lepkef.ing
```

### Start development server
```bash
cargo run -- serve
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
