# TODO: Category-Based Pagination Pages

**Priority**: 🟡
**Estimated Effort**: 2-3 days
**Created**: 2025-09-22
**Status**: Open
**Completed**: N/A

## Problem Description

Currently the generator only emits a single global pagination flow under `/pageN`. Posts can declare a `category`/`categories` field in their front matter, but that metadata is ignored once the content is rendered. Without category-specific pagination, visitors cannot browse posts filtered by a theme, and the generator offers no way to surface topic hubs (e.g., `/categories/music/`). This makes the archive harder to navigate and wastes the categorisation work already captured in the posts.

## Proposed Solution

- Consume the normalised category metadata produced in `todo/011-category-front-matter-normalisation.md` when grouping posts by category.
- Build a grouping map `category_slug -> Vec<ContentItem>` while we are collecting posts for the site, ignoring posts that fail to resolve to a category.
- Introduce a companion generator (e.g., `generate_category_pages`) that reuses the existing pagination renderer to emit pages per category under a predictable folder structure (`/category/<slug>/pageN.html`, plus an index at `/category/<slug>/index.html`).
- Ensure category pages receive contextual metadata (human-readable category name, page number, canonical URLs) and link back to the global index.
- Extend tests to cover category extraction, slug generation, and paginated output to avoid regressions when the pagination logic changes.


## Implementation Plan

### Step 1: Integrate normalised category metadata
- Consume the singular category field exposed by `todo/011-category-front-matter-normalisation.md` when loading posts and surface it on `ContentItem`.
- Expected outcome: Category-aware generation can rely on a consistent optional string.
- Dependencies or prerequisites: Depends on `todo/011-category-front-matter-normalisation.md` delivering the normalised field.

### Step 2: Group posts by category
- While collecting non-draft posts, accumulate them into a `HashMap` keyed by slugified category names, preserving insertion order for deterministic output.
- Expected outcome: Data structure ready for per-category pagination, including display titles and slugs.
- Dependencies or prerequisites: Depends on Step 1 surfacing the normalised category strings.

### Step 3: Generate category pagination pages
- Add a generator function mirroring `generate_pagination_pages` that writes paginated HTML/JSON for each category under `/category/<slug>/`.
- Expected outcome: Category archives with pagination controls integrated into the site output directories.
- Dependencies or prerequisites: Depends on Step 2 for the grouped posts.

### Step 4: Cover with automated tests
- Create integration-style tests that feed posts with overlapping categories and assert the correct files/links are produced.
- Expected outcome: Regression coverage that fails if category pagination breaks in future refactors.
- Dependencies or prerequisites: Steps 1-3 must define the new behaviour.

## Success Criteria

- [ ] Category metadata from front matter (via `todo/011-category-front-matter-normalisation.md`) is accessible in templates for pagination.
- [ ] Generator outputs paginated archives per category under `/category/<slug>/`.
- [ ] Pagination controls include working previous/next/index links scoped to each category.
- [ ] Tests exercise category grouping and page generation.

## Affected Components

- `src/content_loader.rs` (or the module responsible for parsing post front matter) - Expose the normalised category field to generators.
- `src/generate.rs` - Wire category grouping into the generation pipeline.
- `src/generate_pagination_pages.rs` (and potential new module) - Reuse or extend pagination logic for per-category archives.
- `out/<site>/category/**` - New output tree produced by the generator.

## Risks & Considerations

- **Risk**: Upstream normalisation gaps could leave posts without categories; mitigate with guard clauses and logging when grouping.
- **Risk**: Large category counts may create many pages; mitigate with deduplication and optional configuration to cap generation.
- **Dependencies**: Relies on stable post parsing and pagination helpers already present in the codebase.
- **Breaking Changes**: Directory layout grows; ensure any CDN or server configs include the new `/category/` path.

## Related Items

- **Depends on**: `todo/011-category-front-matter-normalisation.md`.
- **Related**: `src/generate_pagination_pages.rs` for existing pagination implementation that will be extended/reused.

## References

- Existing pagination generator in `src/generate_pagination_pages.rs` for overall flow.
- Site content under `sites/*/posts` demonstrating current `categories` metadata usage.

## Notes

- Consider emitting category metadata into the JSON companions for client-side routing parity.
- Cross-link category pages from post footers or navigation once the archive exists.

## Archive Instructions

When this TODO is completed, move it to the `todo/archived/` folder to keep the main todo directory clean and organized.
