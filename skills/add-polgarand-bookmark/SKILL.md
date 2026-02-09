---
name: add-polgarand-bookmark
description: Add or update a bookmark in polgarand.org from a URL by fetching page title and description, then writing into sites/polgarand.org/data/bookmarks.json. Use when asked to "add this as a bookmark" or similar.
---

# Add Polgarand Bookmark

## Workflow

1. Run from repo root (`/home/andormade/andor.cool`).
2. Extract a single URL from the user's prompt.
3. Execute:
```bash
python3 sites/polgarand.org/scripts/add_bookmark.py "<url>"
```
4. Report:
- Command executed
- Exit code
- `added:` or `updated:` line
- Final `title:` and `description:` lines

## Notes

- The script fetches metadata from the page (`title` and `description`) and writes to:
  - `sites/polgarand.org/data/bookmarks.json`
- It does not fetch or modify `favicon`; favicon updates are handled by the separate favicon workflow/script.
- If metadata fetch fails, the script writes fallback values and still completes.

## Exit Code Handling

- `0`: Bookmark add/update completed.
- `2`: Invalid URL or bookmarks input file/setup issue.
