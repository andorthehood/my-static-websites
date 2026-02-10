---
title: 'TODO: Split Type Annotation Stripper Into Focused Modules'
priority: Medium
effort: 1 day
created: 2026-02-07
status: Completed
completed: 2026-02-10
---

# TODO: Split Type Annotation Stripper Into Focused Modules

## Problem Description

The current TypeScript type-annotation stripper implementation in `src/converters/typescript/type_annotations.rs` contains multiple responsibilities in one file.
This makes it harder to reason about edge cases, validate behavior in isolation, and evolve parsing rules without introducing regressions.
As new scenarios are added (for example optional parameter annotations and Promise return types), maintenance cost increases.

## Proposed Solution

Refactor the existing `type_annotations` implementation into smaller modules with one primary function per file.
Colocate the relevant unit tests with each function/module to keep behavior and verification together.
Keep the public behavior and API unchanged (`remove_type_annotations` remains the entry point exposed by the TypeScript converter).

## Anti-Patterns (Optional)

- Do not change parsing behavior while doing the initial structural split.
- Do not move all tests into one shared mega-test module after splitting.
- Do not introduce cross-module circular dependencies for helper functions.

## Implementation Plan

### Step 1: Define split boundaries
- Identify logical functions/groups (for example: string/comment state handling, object-property detection, colon handling, type-span skipping).
- Create one file per function or tightly scoped unit.
- Keep internal interfaces minimal and explicit.

### Step 2: Move implementation incrementally
- Extract each function into its own file under `src/converters/typescript/type_annotations/`.
- Add a `mod.rs` for wiring and keep `remove_type_annotations` as the external entry point.
- Ensure the module compiles after each extraction.

### Step 3: Move and align tests
- Move existing unit tests so each extracted function has relevant tests in the same module/file.
- Add missing focused tests where behavior was previously only covered indirectly.
- Preserve current converter-level integration tests.

## Validation Checkpoints (Optional)

- `cargo test converters::typescript::type_annotations -- --test-threads=1`
- `cargo test converters::typescript -- --test-threads=1`
- `rg -n "mod tests|#\[test\]" src/converters/typescript/type_annotations* -S`

## Success Criteria

- [ ] `type_annotations` logic is split into focused files with one primary function per file
- [ ] Relevant unit tests are colocated with each function/module
- [ ] Public behavior remains unchanged (all existing TypeScript converter tests pass)

## Affected Components

- `src/converters/typescript/type_annotations.rs` - Current monolithic implementation to split
- `src/converters/typescript/mod.rs` - Module wiring may need updates
- `src/converters/typescript/tests.rs` - Integration-level assertions should stay intact

## Risks & Considerations

- **Risk 1**: Accidental behavioral changes during extraction. Mitigation: move code in small steps and run targeted tests after each step.
- **Risk 2**: Over-fragmentation making navigation harder. Mitigation: keep only meaningful boundaries and avoid tiny trivial modules.
- **Dependencies**: No external dependency; can be done independently.
- **Breaking Changes**: None expected if public API remains unchanged.

## Related Items

- **Blocks**: None
- **Depends on**: None
- **Related**: Recent regressions around optional parameters and Promise return type stripping in the TypeScript converter

## References

- `src/converters/typescript/type_annotations.rs`
- `src/converters/typescript/tests.rs`
- `docs/todos/_template.md`

## Notes

The request references `type_annotations.ts`, but this repository implementation is Rust-based and currently lives in `src/converters/typescript/type_annotations.rs`.
This TODO targets the Rust implementation.
Expectation: group closely related functions into folder-based modules instead of keeping a single large file.

## Archive Instructions

When this TODO is completed:
1. Update the front matter to set `status: Completed` and provide the `completed` date
2. Move it to the `todo/archived/` folder to keep the main todo directory clean and organized
3. Update the `todo/_index.md` file to:
   - Move the TODO from the "Active TODOs" section to the "Completed TODOs" section
   - Add the completion date to the TODO entry (use `date +%Y-%m-%d` command if current date is not provided in the context)
