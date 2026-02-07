---
title: 'TODO: Split JS Minifier Into Focused Modules'
priority: Medium
effort: 1 day
created: 2026-02-07
status: Open
completed: null
---

# TODO: Split JS Minifier Into Focused Modules

## Problem Description

`src/minifier/js.rs` mixes multiple responsibilities: parser state, comment handling, template/string parsing, regex heuristics, whitespace policy, and orchestration. This increases regression risk when tweaking rules.

## Proposed Solution

Refactor into smaller modules with one primary function per file while keeping `minify_js` as the stable entry point.

## Implementation Plan

### Step 1: Define split plan
- Separate modules for state, comment handling, string/template handling, regex detection, whitespace handling, and orchestration.

### Step 2: Extract and wire
- Move logic into `src/minifier/js/` with explicit interfaces.
- Keep existing API and outputs stable.

### Step 3: Colocate tests
- Move/add tests near extracted logic.
- Preserve end-to-end minifier tests.

## Validation Checkpoints (Optional)

- `cargo test minifier::js -- --test-threads=1`
- `cargo test minifier -- --test-threads=1`

## Success Criteria

- [ ] JS minifier split by concern
- [ ] Unit tests colocated with modules
- [ ] No behavior regressions in existing tests

## Affected Components

- `src/minifier/js.rs`
- `src/minifier/mod.rs`

## Notes

Regex literal detection is fragile; preserve current heuristics in the first refactor pass.
Expectation: group closely related functions into folder-based modules instead of keeping a single large file.
