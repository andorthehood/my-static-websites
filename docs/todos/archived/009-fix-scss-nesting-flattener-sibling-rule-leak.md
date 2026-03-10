---
title: 'TODO: Fix SCSS Nesting Flattener Sibling Rule Leak'
priority: High
effort: 2 hours
created: 2026-03-10
status: Completed
completed: 2026-03-10
---

# TODO: Fix SCSS Nesting Flattener Sibling Rule Leak

## Problem Description

The SCSS flattening pass under `src/converters/scss/nesting.rs` produces invalid CSS for nested blocks that contain an element selector followed by a sibling nested selector.

Current regression from `sites/polgarand.org/assets/wikipedia.scss`:
- Source SCSS keeps `.userbox-image` and `.userbox-content` as siblings under `.userbox`
- The generated CSS incorrectly places `.userbox-content` inside `.userbox-image`
- The broken output currently looks like `.userbox .userbox-image{img{display:block;}.userbox-content{padding:4px;}}`

This indicates the flattener is misclassifying `img { ... }` as declaration-like content instead of a nested rule, which then corrupts block boundaries for the following sibling selector.

## Proposed Solution

Make nested rule detection in `src/converters/scss/nesting.rs` structural instead of relying on a limited set of selector-leading characters.

High-level approach:
- Detect whether the next block item is a rule by scanning for the next top-level `{`, `;`, or `}` while respecting strings, comments, and bracket/parenthesis depth
- Parse nested element selectors like `img { ... }` as rules, not declarations
- Add a regression test that matches the `wikipedia.scss` failure shape

Expected fixed output for the affected fragment:
- `.userbox .userbox-image img{display:block;}.userbox .userbox-content{padding:4px;}`

## Anti-Patterns (Optional)

- Do not patch only `wikipedia.scss`; the bug is in the generic SCSS flattening pass
- Do not special-case `img` or element selectors by name
- Do not rely on the CSS minifier to repair malformed nested output

## Implementation Plan

### Step 1: Reproduce the regression in a unit test
- Add a focused test in `src/converters/scss/nesting.rs`
- Use the `.userbox`, `.userbox-image`, `img`, and `.userbox-content` structure from the reported failure
- Assert that the sibling `.userbox-content` rule remains a sibling after flattening

### Step 2: Fix nested rule detection
- Update block item parsing in `src/converters/scss/nesting.rs`
- Distinguish declarations from nested rules using top-level token boundaries instead of only the first character
- Preserve current behavior for declarations, at-rules, comments, and strings

### Step 3: Verify end-to-end output
- Run the relevant SCSS converter tests
- Rebuild the affected stylesheet if needed and confirm the flattened CSS no longer nests `.userbox-content` under `.userbox-image`

## Validation Checkpoints (Optional)

- `cargo test scss -- --test-threads=1`
- `cargo test nesting -- --test-threads=1`
- `cargo run -- generate polgarand.org`
- Inspect the generated CSS for `.userbox .userbox-image img{display:block;}` and `.userbox .userbox-content{padding:4px;}`

## Success Criteria

- [ ] Nested element selectors like `img { ... }` are flattened as rules
- [ ] Sibling selectors following a nested element selector stay at the correct block depth
- [ ] `sites/polgarand.org/assets/wikipedia.scss` generates correct flat CSS for the userbox rules
- [ ] Regression coverage exists in `src/converters/scss/nesting.rs`

## Affected Components

- `src/converters/scss/nesting.rs`
- `src/converters/scss/mod.rs`
- `sites/polgarand.org/assets/wikipedia.scss`
- Generated CSS under `out/polgarand.org/` after site generation

## Risks & Considerations

- **Parser scope**: This converter is intentionally minimal, so the fix should stay narrow and structural rather than trying to become a full SCSS parser
- **False positives**: Rule detection must avoid treating declarations with complex values as nested selectors
- **Regression surface**: Existing flattening behavior for at-rules and plain declarations should remain unchanged

## Related Items

- **Blocks**: None
- **Depends on**: None
- **Related**: `src/file_copier.rs`

## Notes

- The failure was observed while processing `sites/polgarand.org/assets/wikipedia.scss`
- The bug appears before CSS minification; the malformed structure originates in the SCSS nesting flattening pass
