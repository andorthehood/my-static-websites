---
title: 'TODO: Split HTML Minifier Into Focused Modules'
priority: Medium
effort: 1 day
created: 2026-02-07
status: Open
completed: null
---

# TODO: Split HTML Minifier Into Focused Modules

## Problem Description

`src/minifier/html.rs` currently combines parse state, comment stripping, tag parsing, string handling, whitespace rules, and top-level minification flow in one file. The file is harder to reason about and harder to extend safely.

## Proposed Solution

Refactor `src/minifier/html.rs` into focused modules with one primary function per file. Keep `minify_html` as the public entry point and preserve current behavior.

## Implementation Plan

### Step 1: Define boundaries
- Split into modules for parse state, comment handling, tag handling, string handling, whitespace policy, and orchestration.

### Step 2: Extract incrementally
- Move one concern at a time into `src/minifier/html/`.
- Add `mod.rs` wiring and keep public API unchanged.

### Step 3: Move/extend tests
- Colocate focused unit tests with each extracted function/module.
- Keep high-level minifier behavior tests intact.

## Validation Checkpoints (Optional)

- `cargo test minifier::html -- --test-threads=1`
- `cargo test minifier -- --test-threads=1`

## Success Criteria

- [ ] HTML minifier split into focused modules
- [ ] Relevant tests colocated with extracted logic
- [ ] Existing minifier behavior preserved

## Affected Components

- `src/minifier/html.rs`
- `src/minifier/mod.rs`

## Notes

Prefer behavioral parity first; avoid changing whitespace semantics during the structural split.
Expectation: group closely related functions into folder-based modules instead of keeping a single large file.
