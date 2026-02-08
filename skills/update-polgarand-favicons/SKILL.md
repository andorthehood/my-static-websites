---
name: update-polgarand-favicons
description: Update favicon files and bookmark favicon mappings for polgarand.org by running the repository script and reporting results. Use when the user asks to update, refresh, re-download, or regenerate favicons/bookmark icons for polgarand.org.
---

# Update Polgarand Favicons

## Workflow

1. Run from repo root (`/home/andormade/andor.cool`).
2. Execute:
```bash
python3 sites/polgarand.org/scripts/download_favicons.py
```
3. Keep streaming output until completion because the script can take time on slow hosts.
4. Capture and report:
- Final summary line: `done: <ok> ok, <failed> failed, out_dir=...`
- Bookmark update line: `bookmarks: updated <n> items (favicon field)`
- Exit code

## Exit Code Handling

- `0`: All favicon fetches succeeded.
- `1`: Script finished but at least one favicon failed to download.
- `2`: Input/setup error (for example missing or invalid `sites/polgarand.org/data/bookmarks.json`).
- `130`: Interrupted (`Ctrl+C`) after partial progress.

Treat exit code `1` as a completed run with partial failures. Report failures clearly instead of calling the run aborted.

## Paths Affected

- Reads and rewrites: `sites/polgarand.org/data/bookmarks.json`
- Writes downloaded icons: `sites/polgarand.org/.favicons/`

## Reporting Back

Report concise operational details:
- Command executed
- Completion status with exit code
- `ok/failed` totals
- Number of bookmark entries updated
- Notable failed hosts if visible in output
