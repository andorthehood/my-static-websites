# TODO: Support nested render paths

**Priority**: 🟡
**Estimated Effort**: 2 days
**Created**: 2025-09-23
**Status**: Open
**Completed**: -

## Problem Description

Liquid `{% render %}` tags currently assume all include templates live at the root of the includes directory. Any attempt to reference a template inside a subdirectory (e.g. `{% render 'components/buttons/cta' %}`) fails because we only load flat filenames and the parser normalises names without keeping path information. This blocks a common organisational pattern for larger sites: grouping partials by feature. It also makes it harder to share partials across multiple templates without cluttering the top-level directory, and prevents parity with Shopify/Liquid behaviour where nested include paths are permitted.

## Proposed Solution

- Recursively walk the includes directory, building keys from the relative path (minus the `.liquid` suffix) so subdirectories are preserved (use `/` as the canonical separator).
- Teach the render-tag parser to accept names containing `/` and normalise optional `.liquid` suffixes without collapsing directories.
- Ensure the render processor resolves templates using the new keys and extend tests to cover nested paths, including mixed quoting and extension variants.
- Consider adding guards to warn when two files normalise to the same key (e.g. OS case differences) to avoid silent overwrites.

## Implementation Plan

### Step 1: Recursively load includes with stable keys
- Replace the current flat `read_dir` in `load_liquid_includes` with a depth-first walk, computing keys from the relative path and storing templates in the includes map.
- Includes from nested folders are accessible via their folder-qualified key.
- Requires a helper to normalise path separators to `/` and strip trailing `.liquid`.

### Step 2: Expand render tag parsing behaviour
- Update `extract_template_name` so it preserves folder segments, validates relative paths (reject `..`), and still supports quoted/unquoted names.
- Allows `{% render 'foo/bar/baz' %}` and `{% render foo/bar/baz.liquid %}`.
- Depends on the helper ensuring consistent normalisation with loader logic.

### Step 3: Strengthen render resolution tests
- Add unit tests for parser and renderer covering nested names, missing templates, and conflicting keys.
- Add an integration-style fixture under `sites/test/includes` to confirm the full pipeline resolves nested renders via `render_page`.
- Run `cargo test` to verify coverage and prevent regressions.

## Success Criteria

- [ ] `{% render 'foo/bar/baz' %}` resolves when `foo/bar/baz.liquid` exists.
- [ ] Unit and integration tests cover nested render scenarios across parser, loader, and renderer layers.
- [ ] Missing nested renders produce a clear warning/error without panicking, and the page generation continues when appropriate.

## Affected Components

- `src/load_includes.rs` - Switch to recursive directory loading and path normalisation helper.
- `src/template_processors/liquid/parse_render_tag.rs` - Preserve folder-qualified template names when parsing.
- `src/template_processors/liquid/process_renders.rs` - Ensure lookup keys align with nested naming and add tests.
- `sites/test/includes/**` - Add fixtures for integration coverage.

## Risks & Considerations

- **Risk 1**: Path normalisation collisions (e.g. Windows vs. Unix case sensitivity) could overwrite entries; mitigate by detecting duplicates and surfacing warnings.
- **Risk 2**: Recursive directory walks might impact build times on large include trees; mitigate by using `WalkDir` or similar efficient traversal and limiting to `.liquid` files.
- **Dependencies**: Relies on current include loading logic; ensure no TODOs above in sequence require completion first.
- **Breaking Changes**: Keys now include subdirectory paths; confirm no existing templates rely on ambiguous names that would change resolution order.

## Related Items

- **Blocks**: Future enhancements that assume nested component organisation.
- **Depends on**: None.

## References

- [Liquid `render` tag reference](https://shopify.dev/docs/api/liquid/tags/theme-tags#render)
- [Discussion on nested includes](https://github.com/Shopify/liquid/discussions/1666)

## Notes

- Preserve backwards compatibility by continuing to support flat filenames without directory separators.
- Update documentation once implementation is ready.

## Archive Instructions

When this TODO is completed, move it to the `todo/archived/` folder to keep the main todo directory clean and organized.
