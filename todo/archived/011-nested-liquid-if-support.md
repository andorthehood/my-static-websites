# TODO: Nested Liquid IF Support

**Priority**: 🟡
**Estimated Effort**: 6-8 hours
**Created**: 2025-09-24
**Status**: Open
**Completed**: 2025-09-24

## Problem Description

The Liquid conditional processor only matches `{% if %}` blocks at a single depth because it pairs opening and closing tags by simple substring search. Nested conditionals are stripped from the template or cause spurious "Missing {% endif %} tag" errors, and we currently have no regression tests covering nested branching. This prevents authoring templates that rely on Liquid's documented ability to nest control-flow tags and makes the engine diverge from Shopify Liquid semantics.

## Proposed Solution

- Capture the current failure mode with focused unit tests that render nested `{% if %}` blocks (truthy, falsy, and mixed branches) and assert the expected output and error handling.
- Extend the conditional tag processor to walk tag blocks with depth awareness, reusing existing helpers like `read_nested_block` where possible so nested `{% if %}`/`{% endif %}` pairs are resolved correctly.
- Keep the existing truthiness evaluation semantics and error messaging consistent, updating shared utilities only if required to support recursion.

## Implementation Plan

### Step 1: Add regression tests for nested conditionals
- Introduce new unit tests in `process_liquid_conditional_tags` covering nested truthy/falsy branches and unterminated inner blocks.
- Validate that the current implementation fails, establishing baseline coverage for the bug.
- Dependencies: test harness already available; no new data fixtures required.

### Step 2: Update parser to support nested `{% if %}`
- Refactor the conditional processor to iterate through the template while tracking depth, leveraging or adapting `read_nested_block` to capture inner content safely.
- Ensure replacements are applied from the innermost block outward so nested evaluations propagate correctly.
- Dependencies: Step 1 to confirm the failing behaviour and guide refactor.

### Step 3: Verify integrations and error handling
- Re-run the Liquid processor integration tests (or add new ones) to confirm nested support plays nicely with `unless`, `for`, and variable replacement passes.
- Confirm missing `{% endif %}` errors still surface and add documentation/tests if behaviour changes.
- Dependencies: Step 2 implementation complete.

## Success Criteria

- [ ] Nested `{% if %}` blocks render correctly for truthy and falsy combinations.
- [ ] Unterminated nested blocks raise the expected `Error::Liquid` message.
- [ ] `cargo test -- --test-threads=1` passes for Liquid modules without regressions.

## Affected Components

- `src/template_processors/liquid/_if.rs` - Extend parsing logic and add regression tests.
- `src/template_processors/liquid/utils/tag_parsing.rs` - Potentially reuse or enhance nested block utilities.
- `src/template_processors/liquid/processor.rs` - Ensure orchestration of Liquid passes still functions with nested support.

## Risks & Considerations

- **Risk**: Refactoring the parser could regress performance on large templates; mitigate with targeted benchmarking if slowdowns appear.
- **Risk**: Changes might disrupt `unless` or other tags that depend on similar helpers; mitigate by running the full Liquid processor test suite.
- **Dependencies**: None beyond existing Liquid utilities; coordination with TODO 010 (testing improvements) could simplify broader coverage.
- **Breaking Changes**: None expected; the goal is to align behaviour with standard Liquid semantics.

## Related Items

- **Related**: TODO 010 (Testing with Mocks) for expanding automated coverage once nested logic lands.

## References

- [Shopify Liquid Control Flow](https://shopify.dev/docs/api/liquid/tags/control-flow#if)
- [Liquid Conditional Documentation](https://shopify.github.io/liquid/tags/control-flow/)

## Notes

- Plan captured from investigation into missing nested support; evaluate whether shared utilities (`read_nested_block`) should be generalized for reuse across other Liquid tags.
- Track parser complexity to ensure future tags can leverage the same approach without duplication.

## Archive Instructions

When this TODO is completed, move it to the `todo/archived/` folder to keep the main todo directory clean and organized.
