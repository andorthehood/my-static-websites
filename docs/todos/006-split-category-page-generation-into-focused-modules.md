---
title: 'TODO: Split Category Page Generation Into Focused Modules'
priority: Medium
effort: 1 day
created: 2026-02-07
status: Open
completed: null
---

# TODO: Split Category Page Generation Into Focused Modules

## Problem Description

`src/generate_category_pages.rs` blends slug normalization, grouping posts, pagination context construction, URL generation, layout selection, and rendering orchestration.

## Proposed Solution

Split into focused modules with one primary function per file: category grouping, pagination context builder, URL builder, and render orchestration. Preserve current output behavior.

## Implementation Plan

### Step 1: Isolate pure logic
- Extract pure helpers for slugging, grouping, URL construction, and page variable generation.

### Step 2: Separate orchestration
- Keep IO/render calls in a thin orchestrator.
- Move logic into `src/generate_category_pages/`.

### Step 3: Strengthen tests
- Add unit tests for pure helpers and keep integration coverage for generated output expectations.

## Validation Checkpoints (Optional)

- `cargo test generate_category_pages -- --test-threads=1`
- `cargo test generate -- --test-threads=1`

## Success Criteria

- [ ] File split into focused modules
- [ ] Pure logic independently unit-tested
- [ ] Generated category pagination output unchanged

## Affected Components

- `src/generate_category_pages.rs`
- `src/generate.rs`

## Notes

Avoid changing URL formats or pagination variable names during this refactor.
Expectation: group closely related functions into folder-based modules instead of keeping a single large file.
