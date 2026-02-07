---
title: 'TODO: Split Liquid For-Loop Processor Into Focused Modules'
priority: Medium
effort: 1 day
created: 2026-02-07
status: Open
completed: null
---

# TODO: Split Liquid For-Loop Processor Into Focused Modules

## Problem Description

`src/template_processors/liquid/for_loop.rs` currently contains loop tag scanning, loop expression parsing, parameter handling (e.g. limits), loop expansion, nested-loop handling, and tests in one file.

## Proposed Solution

Split into focused modules with one primary function per file and clear interfaces. Keep `process_liquid_for_loops` behavior unchanged.

## Implementation Plan

### Step 1: Define module boundaries
- Scanner, expression parser, parameter parser, expander, and nested-loop handling.

### Step 2: Extract incrementally
- Move code into `src/template_processors/liquid/for_loop/`.
- Keep public API stable.

### Step 3: Test colocation
- Place unit tests with each extracted function/module.
- Keep high-level expansion tests for behavior parity.

## Validation Checkpoints (Optional)

- `cargo test template_processors::liquid::for_loop -- --test-threads=1`
- `cargo test template_processors::liquid -- --test-threads=1`

## Success Criteria

- [ ] For-loop processor split by concern
- [ ] Nested-loop behavior preserved
- [ ] Existing tests pass with colocated unit tests

## Affected Components

- `src/template_processors/liquid/for_loop.rs`
- `src/template_processors/liquid/mod.rs`

## Notes

Nested behavior should be treated as high-risk; keep regression tests broad.
Expectation: group closely related functions into folder-based modules instead of keeping a single large file.
