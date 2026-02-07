---
title: 'TODO: Split Liquid Assign Processor Into Focused Modules'
priority: Medium
effort: 1 day
created: 2026-02-07
status: Open
completed: null
---

# TODO: Split Liquid Assign Processor Into Focused Modules

## Problem Description

`src/template_processors/liquid/assign.rs` combines tag scanning, assign parsing, filter parsing/execution, data projection into variables, and tests in one file.

## Proposed Solution

Split into focused modules (scanner, parser, filter dispatcher, `where` filter, variable writer) with one primary function per file. Keep external behavior unchanged.

## Implementation Plan

### Step 1: Carve responsibilities
- Define internal interfaces between scanning, parsing, filtering, and variable writes.

### Step 2: Extract modules
- Move implementation under `src/template_processors/liquid/assign/`.
- Keep `process_liquid_assign_tags` as public entry point.

### Step 3: Move tests
- Colocate unit tests with extracted functions/modules.
- Keep integration tests for end-to-end tag behavior.

## Validation Checkpoints (Optional)

- `cargo test template_processors::liquid::assign -- --test-threads=1`
- `cargo test template_processors::liquid -- --test-threads=1`

## Success Criteria

- [ ] Assign processor split into focused modules
- [ ] Filter behavior (especially `where`) unchanged
- [ ] Tests colocated and passing

## Affected Components

- `src/template_processors/liquid/assign.rs`
- `src/template_processors/liquid/mod.rs`

## Notes

Treat filter behavior as externally observable contract; avoid semantic changes during split.
Expectation: group closely related functions into folder-based modules instead of keeping a single large file.
