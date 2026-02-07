---
title: 'TODO: Split JSON Parser Into Focused Modules'
priority: Medium
effort: 1 day
created: 2026-02-07
status: Open
completed: null
---

# TODO: Split JSON Parser Into Focused Modules

## Problem Description

`src/parsers/json.rs` is large and includes value dispatch, token-level parsing, parser cursor helpers, object/array parsing, and error handling in one file.

## Proposed Solution

Split parser responsibilities into focused modules with one primary function per file, while keeping `parse_json`/parser external behavior stable.

## Implementation Plan

### Step 1: Define parser layers
- Cursor/state utilities, value dispatcher, string parser, number parser, boolean parser, array/object parsers.

### Step 2: Extract modules
- Move code under `src/parsers/json/` with a clear `mod.rs` boundary.
- Keep public interfaces unchanged.

### Step 3: Colocate tests
- Move/add unit tests for each parser function/module.
- Keep end-to-end parser tests unchanged.

## Validation Checkpoints (Optional)

- `cargo test parsers::json -- --test-threads=1`
- `cargo test parsers -- --test-threads=1`

## Success Criteria

- [ ] JSON parser split into focused modules
- [ ] Parser behavior and error semantics preserved
- [ ] Existing and colocated tests pass

## Affected Components

- `src/parsers/json.rs`
- `src/parsers/mod.rs`

## Notes

Do not change supported JSON subset as part of structural refactor.
Expectation: group closely related functions into folder-based modules instead of keeping a single large file.
