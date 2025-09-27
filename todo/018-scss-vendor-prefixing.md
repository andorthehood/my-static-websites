# TODO: Add vendor prefix support to SCSS converter

**Priority**: 🟡
**Estimated Effort**: 2-3 days
**Created**: 2025-09-27
**Status**: Open
**Completed**: N/A

## Problem Description

The in-house SCSS converter currently flattens nesting and inlines partials but emits modern CSS without vendor-prefixed fallbacks.
- Older browsers (Safari, legacy Edge) still rely on prefixed declarations for flexbox, user-selection, appearance, and gradient syntax.
- Without prefixes, generated sites lose layout fidelity (e.g., flex layouts breaking, buttons losing styling) on those browsers.
- This gap undermines our zero-dependency toolchain goal because users must still post-process CSS externally to ensure compatibility.

## Proposed Solution

Implement a lightweight vendor-prefixing stage directly in the SCSS pipeline after nesting flattening.
- Introduce a prefixing helper that scans declaration blocks, injects only the missing prefixed forms, and avoids duplicates.
- Maintain a curated mapping of properties/values requiring prefixes (flexbox, user-select, appearance, gradients, backdrop-filter, etc.).
- Expose the mapping via Rust configuration so we can opt in/out of specific prefixed rules without touching templates.
- Keep scope intentionally small and easily extensible so future additions are straightforward.

## Implementation Plan

### Step 1: Define prefix rules and parser helpers
- Create `src/converters/scss/prefixes.rs` with rule tables, configuration structs, and helpers to parse declarations safely.
- Expected outcome: a function returning prefixed declaration lists, using configuration to decide which rules to emit, without altering unrelated CSS.
- Dependencies or prerequisites: reuse/extend existing parser patterns from `nesting.rs`.

### Step 2: Integrate prefixer into pipeline
- Call the new helper from `scss_to_css_with_inline_imports` right after nesting flattening, wiring in configuration defaults at the module boundary.
- Expected outcome: generated CSS includes required prefixed declarations while keeping original rules intact and allowing overrides when needed.
- Dependencies or prerequisites: ensure `file_copier` continues minifying the enriched CSS without regressions.

### Step 3: Validate with automated coverage
- Add unit tests for the prefix helper, including cases toggling configuration flags, and an integration test covering SCSS → CSS → minified output.
- Expected outcome: deterministic tests confirming prefixes are emitted, deduplicated, and configurable.
- Dependencies or prerequisites: none.

## Success Criteria

- [ ] Flexbox declarations render with `-webkit-` and `-ms-` prefixes when missing.
- [ ] `user-select` and `appearance` gain prefixed variants without duplicate lines.
- [ ] Configuration toggles allow excluding a rule from prefixing and are covered by tests.
- [ ] New tests cover the prefixing logic and pass via `cargo test -- --test-threads=1`.

## Affected Components

- `src/converters/scss/mod.rs` - Wire in prefixing stage to existing pipeline.
- `src/converters/scss` - New prefixing module, configuration types, and possible parser utilities.
- `src/file_copier.rs` - Indirectly affected through SCSS conversion output used for asset hashing and minification.

## Risks & Considerations

- **Risk 1**: Manual prefix list gets outdated; mitigate by documenting scope and adding follow-up checklist for new CSS features.
- **Risk 2**: Prefix injection might increase bundle size; mitigate by deduplicating declarations and keeping mapping focused.
- **Dependencies**: None.
- **Breaking Changes**: Minimal; additional declarations should be backwards-compatible, but watch for hash changes in asset filenames.

## Related Items

- **Blocks**: None.
- **Depends on**: None.
- **Related**: TODO-010 (testing improvements) for adding targeted coverage patterns.

## References

- [MDN compatibility data for CSS features](https://developer.mozilla.org/)
- [Autoprefixer documentation](https://github.com/postcss/autoprefixer)
- [Can I use](https://caniuse.com/)

## Notes

- Keep prefix rules data-driven to simplify future updates.
- Consider exposing the prefix map via tests so contributors can update confidently.
- Hash-based asset filenames will change once prefixes are added—plan to invalidate caches accordingly.
- Update this entry when the prefixing module lands to track maintenance expectations.

## Archive Instructions

When this TODO is completed, move it to the `todo/archived/` folder to keep the main todo directory clean and organized.
