# Repository Guidelines

## Project Structure & Module Organization
Source code lives in `src/`, split by responsibility (`minifier/`, `parsers/`, `server/`, etc.) and coordinated through `main.rs`. Static content for each site resides in `sites/<domain>/` following the `posts/`, `pages/`, `includes/`, and `layouts/` pattern; keep shared assets DRY across sites. Build artifacts land in `out/`, while coverage reports are written to `coverage/`. Git hooks are stored under `hooks/`; install them before contributing.

## Build, Test, and Development Commands
Use `cargo run -- generate <site>` to render a site once and `cargo run -- watch <site> [--ramdisk]` for incremental rebuilds. `cargo run -- serve` hosts the generated output locally. Make targets wrap common flows: `make generate SITE=example.com`, `make watch`, `make serve`, and `make install-hooks` to copy the pre-commit hook. Keep code tidy with `make format`, lint with `make lint` or `make lint-pedantic`, and measure coverage via `make coverage` (Tarpaulin HTML report in `coverage/`).

## Coding Style & Naming Conventions
We rely on `rustfmt` defaults (4-space indentation, trailing commas where possible) and `clippy` to enforce idiomatic Rust. Modules and files follow snake_case; types use UpperCamelCase, functions and variables stay snake_case. Prefer small, focused modules under `src/` rather than mega-files. Template includes under `sites/*/includes/` should mirror their usage scope, and generated asset filenames remain hashed.

## Testing Guidelines
Regression tests live alongside code and lean on `insta` snapshots stored in `src/snapshots/`. Run `cargo test -- --test-threads=1` to avoid race conditions with filesystem fixtures. When snapshots change, use `cargo insta review` (or `cargo insta accept` once verified) and commit the updated `.snap` files. New tests should isolate temporary output—clean `out/` before and after to keep results stable. For coverage tracking in CI, run `cargo tarpaulin --out xml`.

## Commit & Pull Request Guidelines
Follow the existing Conventional Commit style: `type(scope): short imperative summary` (e.g., `feat(lepkef.ing): add explicit banner`). Squash incidental fixes before raising a PR. Each PR should describe the motivation, list key changes, link relevant issues, and note testing (`cargo test`, `make lint`, etc.). Include screenshots or output snippets when UI or generated site changes are involved.

## Environment & Tooling Tips
Install hooks via `make install-hooks` to auto-format staged Rust files. Local development assumes the stable Rust toolchain (`rustup toolchain install stable`). The watch command supports a RAM-disk mode on Linux; ensure `/dev/shm` has space if you enable it. Keep the repo clean by removing generated artifacts from commits unless explicitly required.
