# TODO: Implicit Liquid Include Extensions

**Priority**: 🟡
**Estimated Effort**: 3-4 hours
**Created**: 2025-09-22
**Status**: Completed
**Completed**: 2025-09-22

## Problem Description

The include system currently requires every `{% include %}` tag and the loader map to reference explicit `.liquid` filenames. This differs from other Liquid engines where the extension is assumed, so authors rarely type the suffix. Because of this mismatch, templates need redundant `.liquid` suffixes and the planned folder-aware includes work must keep exposing implementation details instead of just the logical include name. We only target Liquid partials, so custom extensions are unnecessary, but the current ergonomics still get in the way and diverge from Liquid docs that always show quoted include names.

## Proposed Solution

Standardise on implicit `.liquid` resolution for include tags.
- Teach the include loader to normalise template keys without the `.liquid` suffix while continuing to read `.liquid` files from disk.
- Update include tag parsing so templates can be referenced without typing the extension, canonicalise away any accidental suffix, and preserve the canonical `{% include 'path' %}` quoting style from Liquid docs.
- Adjust the include processor to append the extension automatically during lookup and update documentation to reflect the extensionless calling convention.

## Implementation Plan

### Step 1: Harden include loading
- Walk the includes directory and store every `.liquid` file under a canonical key that omits the suffix and uses forward slashes.
- Detect and report any collisions where multiple files would map to the same canonical key.

### Step 2: Normalize tag parsing and lookup
- Update the include tag parser to require wrapping single quotes, strip them during lookup, and discard any trailing `.liquid` before performing the lookup.
- Ensure the processor always appends `.liquid` during lookup so custom extensions are neither expected nor supported.
- Add error logging or diagnostics when no template is found after normalization.

### Step 3: Refresh fixtures and docs
- Extend unit tests and snapshot fixtures to cover extensionless include usage and nested directories.
- Document the implicit extension behaviour in developer docs and user-facing configuration notes.
- Add regression tests confirming existing quoted references continue working while enforcing the quoted form.

## Success Criteria

- [ ] Include tags written as `{% include 'path/to/partial' %}` resolve correctly across nested directories without requiring `.liquid`.
- [ ] Automated tests cover loader normalization and processor resolution paths.

## Affected Components

- `src/load_includes.rs` - Normalize include keys by stripping the `.liquid` suffix.
- `src/template_processors/liquid/process_includes.rs` - Append `.liquid` during lookup.
- `sites/test/includes/` - Update fixtures to exercise extensionless usage.
- `todo/README.md` - Keep the queue updated with the new work item.
- `docs/` or `README.md` - Mention the implicit extension behaviour and the required quoted syntax.

## Risks & Considerations

- **Collision Risk**: Different files could normalize to the same key (e.g., `foo.liquid` and `foo/index.liquid` when flattened). Need deterministic conflict handling.
- **Migration Effort**: Existing templates must drop the `.liquid` suffix and adopt the quoted syntax, so documentation and examples need to change in sync.
- **Testing Coverage**: Additional tests required to avoid regressions when new include features are added later.
- **Dependencies**: Directory include work depends on canonical, extensionless include keys.

## Related Items

- **Blocks**: TODO 012 (Category Pagination Pages) may rely on cleaner include ergonomics when composing partials.
- **Depends on**: None.
- **Related**: Planned nested include support work.

## References

- [Shopify Liquid Include Tag](https://shopify.dev/docs/api/liquid/tags/theme-tags#include)
- [Jekyll Include Documentation](https://jekyllrb.com/docs/includes/)

## Notes

- Requested by users who want to mirror standard Liquid ergonomics.
- Complementary to upcoming folder-aware include capabilities.
- Keep an eye on performance when storing duplicate keys in the include map.

## Archive Instructions

When this TODO is completed, move it to the `todo/archived/` folder to keep the main todo directory clean and organized.
