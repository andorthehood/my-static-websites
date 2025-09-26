# TODO: Pagination Layout Support

**Priority**: 🟡
**Estimated Effort**: 1-2 days
**Created**: 2024-09-24
**Status**: Open

## Problem Description

Pagination and category archive pages are currently assembled with hardcoded HTML strings in `src/generate_pagination_pages.rs` and `src/generate_category_pages.rs`. The generator bypasses the template pipeline, so layouts, includes, and shared components cannot be reused, and the markup cannot be customized per site. This creates inconsistent styling, duplicates copy, and makes it difficult to localize or adjust pagination UI. The lack of configuration also blocks sites from adopting bespoke archive designs without forking the generator.

## Proposed Solution

Introduce a configurable pagination layout that flows through the existing rendering pipeline:
- Allow sites to opt into a `pagination_layout` (and optional `category_pagination_layout`) via their `config.md`.
- Reuse the layout loading utilities so pagination pages render Liquid templates with access to global variables and per-page context.
- Preserve the current hardcoded HTML as a default fallback when no layout is configured or the file is missing.
- Document the new configuration and provide a sample layout in at least one site to guide adopters.

## Implementation Plan

### Step 1: Extend configuration
- Surface `pagination_layout` keys from `config.md` through `setup_global_variables` so generators can read them.
- Define defaults to keep existing behavior when the keys are absent.
- Update documentation to describe the new settings.

### Step 2: Share layout resolution helpers
- Extract a reusable helper for building layout paths (currently in `render_page::build_layout_path`).
- Ensure pagination generators can resolve and load layouts without duplicating logic.
- Add warning logs when configured layouts cannot be found.

### Step 3: Refactor pagination generators
- Replace string concatenation in `generate_pagination_pages` and `generate_category_pages` with template rendering.
- Build structured context (page numbers, navigation URLs, sliced posts) and feed it into the chosen layout.
- Maintain the existing HTML as a fallback body when no layout is configured.

### Step 4: Provide fixtures and tests
- Add sample pagination layout files and wire them into at least one site configuration.
- Update unit/integration tests to cover both configured and fallback behavior.
- Capture snapshots or assertions that confirm the templated output renders as expected.

## Success Criteria

- [ ] Sites can define `pagination_layout` in `config.md` and see it applied.
- [ ] Pagination generators fall back to existing markup when no layout is configured.
- [ ] Automated tests cover templated and fallback pagination output.
- [ ] Documentation and sample layouts explain how to adopt the feature.

## Affected Components

- `src/generate_pagination_pages.rs` - Refactor to use template-driven rendering.
- `src/generate_category_pages.rs` - Mirror the templated approach for category pagination.
- `src/render_page.rs` / `src/layout.rs` - Expose shared layout path helpers.
- `src/generate.rs` - Pass new configuration values into pagination generators.
- `sites/*/layouts` and `sites/*/config.md` - Add sample layout and configuration entries.
- `tests` / `src/snapshots` - Update or add coverage for the new behavior.

## Risks & Considerations

- **Regression risk**: Template-based rendering might change whitespace or markup; mitigate with snapshots.
- **Configuration errors**: Missing layout files should degrade gracefully with clear warnings.
- **Performance**: Additional template processing must not slow generation noticeably.
- **Dependencies**: Requires prior pagination and category generation work to remain stable.
- **Breaking Changes**: None expected if fallbacks remain intact.

## Related Items

- **Blocks**: Future pagination UX improvements that rely on richer templates.
- **Depends on**: Stable pagination generators (completed TODO 012 in `archived/`).
- **Related**: TODO 010 (Testing with Mocks) for improving validation coverage.

## References

- `src/generate_pagination_pages.rs` (current implementation)
- `src/generate_category_pages.rs` (category pagination logic)
- `todo/archived/012-category-pagination-pages.md` (background on pagination work)

## Notes

- Consider extracting the post list markup into an include for reuse across archive types.
- Evaluate whether pagination context should expose previous/next page metadata to other templates.
- Align the messaging copy with site tone when moving to templates.

## Archive Instructions

When this TODO is completed, move it to the `todo/archived/` folder to keep the main todo directory clean and organized.
