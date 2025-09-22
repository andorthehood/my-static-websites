# TODO: Rename Include Tag To Render

**Priority**: 🟡
**Estimated Effort**: 4-6 hours
**Created**: 2025-09-22
**Status**: Open
**Completed**: -

## Problem Description

Shopify Liquid has deprecated the `{% include %}` tag in favour of `{% render %}`. Our engine still parses, documents, and emits helpers for `{% include %}`, which diverges from the official syntax and makes it harder for users to reuse snippets from upstream documentation. Because we don't intend to retain backward compatibility, keeping the old tag creates needless maintenance and guarantees a breaking change later. Aligning with `render` now keeps our feature surface consistent with the ecosystem and simplifies upcoming work on snippet handling.

## Proposed Solution

Transition the engine to use `{% render %}` exclusively.
- Update parsing utilities to recognise `render` as the only supported partial invocation tag.
- Remove `include`-specific code paths, helpers, and documentation.
- Refresh fixtures and examples to use `{% render 'snippet' %}` consistently, noting the breaking change for template authors.

## Implementation Plan

### Step 1: Update parsing and processing
- Rename or replace include-specific helpers (e.g., `parse_include_tag`, `process_liquid_includes`) with render-focused equivalents.
- Ensure tag parsing enforces the quoted filename style and canonicalises lookups through the new implicit-extension logic.
- Delete `include` token handling entirely to avoid fallback behaviour.

### Step 2: Adjust call sites and fixtures
- Replace `{% include %}` usages across test fixtures, templates, and documentation with `{% render %}`.
- Update snapshots/tests that assert on tag names or helper function outputs.
- Add focused tests covering the render path (including negative tests confirming `include` now errors).

### Step 3: Communicate and document the breaking change
- Document the new canonical syntax and highlight the removal of `{% include %}` in README/docs.
- Add release notes or migration guidance so users know to update templates before upgrading.

## Success Criteria

- [ ] Templates using `{% render 'snippet' %}` resolve correctly through the include/render infrastructure.
- [ ] `{% include %}` tags are no longer parsed or executed, and tests fail if they appear.
- [ ] All fixtures, docs, and examples reference `{% render %}` exclusively.

## Affected Components

- `src/template_processors/liquid/parse_include_tag.rs` - Replace with render-specific parsing utilities.
- `src/template_processors/liquid/process_includes.rs` - Rename and update logic for render-only handling.
- `src/template_processors/processor.rs` - Ensure the main entry point wires in the render helpers.
- `sites/` fixtures and snapshots - Update sample templates to use `{% render %}` only.
- `docs/`, `README.md`, changelog - Communicate the breaking change and migration steps.

## Risks & Considerations

- **Breaking Change**: Removing `{% include %}` will break existing templates; coordinate release messaging and versioning.
- **Scope Semantics**: Liquid `render` introduces isolated scope semantics; evaluate whether matching this behaviour is necessary before removing `include`.
- **Testing Surface**: Renaming functions and removing legacy paths may ripple through many tests; ensure coverage stays comprehensive.

## Related Items

- **Depends on**: TODO 013 (implicit `.liquid` resolution) for canonical include keys.
- **Related**: Future work on directory-aware renders.

## References

- [Shopify Liquid Render Tag](https://shopify.dev/docs/api/liquid/tags/theme-tags#render)
- [Liquid Deprecation Notice](https://shopify.dev/docs/themes/migrate/snippets#include-to-render)

## Notes

- Consider introducing a version gate or release flag to warn users ahead of the breaking change.
- Monitor performance after helper renames to ensure no regressions occur.

## Archive Instructions

When this TODO is completed, move it to the `todo/archived/` folder to keep the main todo directory clean and organized.
