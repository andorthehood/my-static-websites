#!/usr/bin/env python3
from __future__ import annotations

# Purpose: download favicons for each bookmark in `sites/polgarand.org/data/bookmarks.json`,
# save them into `sites/polgarand.org/.favicons/` (gitignored) using the domain as the filename,
# and write the resulting favicon filename back onto each bookmark entry as the `favicon` field.

import json
import mimetypes
import re
import sys
import urllib.error
import urllib.parse
import urllib.request
from dataclasses import dataclass
from html.parser import HTMLParser
from pathlib import Path
from typing import Optional

BOOKMARKS_PATH = Path("sites/polgarand.org/data/bookmarks.json")
OUT_DIR = Path("sites/polgarand.org/.favicons")

TIMEOUT_S = 10.0
MAX_BYTES = 1_000_000
USER_AGENT = "polgarand.org favicon-fetcher/1.0"


def _safe_domain_filename(netloc: str) -> str:
    domain = netloc.split("@")[-1]
    domain = domain.split(":")[0]
    domain = domain.strip().lower()
    return re.sub(r"[^a-z0-9.-]+", "_", domain).strip("._") or "unknown"


def _guess_extension(url: str, content_type: Optional[str]) -> str:
    path = urllib.parse.urlparse(url).path
    ext = Path(path).suffix.lower()
    if ext in {".ico", ".png", ".svg", ".jpg", ".jpeg", ".webp", ".gif"}:
        return ".jpg" if ext == ".jpeg" else ext

    if content_type:
        mime = content_type.split(";", 1)[0].strip().lower()
        if mime == "image/x-icon":
            return ".ico"
        guessed = mimetypes.guess_extension(mime) or ""
        if guessed:
            return guessed

    return ".ico"


@dataclass(frozen=True)
class FaviconCandidate:
    url: str
    priority: int


class _IconLinkParser(HTMLParser):
    def __init__(self) -> None:
        super().__init__()
        self.candidates: list[FaviconCandidate] = []

    def handle_starttag(self, tag: str, attrs: list[tuple[str, Optional[str]]]) -> None:
        if tag.lower() != "link":
            return
        attr_map = {k.lower(): (v or "") for k, v in attrs}
        rel = attr_map.get("rel", "").lower()
        href = attr_map.get("href", "").strip()
        if not href:
            return

        if "icon" not in rel:
            return

        priority = 10
        if "shortcut" in rel:
            priority = 9
        if "apple-touch-icon" in rel:
            priority = 8
        self.candidates.append(FaviconCandidate(url=href, priority=priority))


def _http_get(url: str, *, max_bytes: int) -> tuple[bytes, str]:
    req = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    with urllib.request.urlopen(req, timeout=TIMEOUT_S) as resp:
        content_type = resp.headers.get("Content-Type", "")
        body = resp.read(max_bytes + 1)
        if len(body) > max_bytes:
            raise ValueError(f"response too large (> {max_bytes} bytes): {url}")
        return body, content_type


def _discover_icons_from_html(base_url: str, html: str) -> list[FaviconCandidate]:
    parser = _IconLinkParser()
    try:
        parser.feed(html)
    except Exception:
        return []

    out: list[FaviconCandidate] = []
    for c in parser.candidates:
        out.append(FaviconCandidate(url=urllib.parse.urljoin(base_url, c.url), priority=c.priority))
    return out


def _default_candidates(page_url: str) -> list[FaviconCandidate]:
    parsed = urllib.parse.urlparse(page_url)
    origin = f"{parsed.scheme}://{parsed.netloc}"
    return [
        FaviconCandidate(url=urllib.parse.urljoin(origin, "/favicon.ico"), priority=100),
        FaviconCandidate(url=urllib.parse.urljoin(origin, "/favicon.png"), priority=95),
        FaviconCandidate(url=urllib.parse.urljoin(origin, "/apple-touch-icon.png"), priority=50),
        FaviconCandidate(url=urllib.parse.urljoin(page_url, "favicon.ico"), priority=90),
        FaviconCandidate(url=urllib.parse.urljoin(page_url, "favicon.png"), priority=85),
        FaviconCandidate(url=urllib.parse.urljoin(page_url, "apple-touch-icon.png"), priority=40),
    ]


def download_favicon(page_url: str) -> Optional[str]:
    parsed = urllib.parse.urlparse(page_url)
    if not parsed.netloc:
        return None
    domain_key = _safe_domain_filename(parsed.netloc)
    OUT_DIR.mkdir(parents=True, exist_ok=True)

    origin = f"{parsed.scheme}://{parsed.netloc}"
    candidates: list[FaviconCandidate] = []
    candidates.extend(_default_candidates(page_url))

    try:
        html_bytes, _ct = _http_get(page_url, max_bytes=min(MAX_BYTES, 512_000))
        candidates.extend(_discover_icons_from_html(page_url, html_bytes.decode("utf-8", errors="replace")))
    except Exception:
        pass

    if page_url.rstrip("/") != origin.rstrip("/"):
        try:
            html_bytes, _ct = _http_get(origin, max_bytes=min(MAX_BYTES, 512_000))
            candidates.extend(_discover_icons_from_html(origin, html_bytes.decode("utf-8", errors="replace")))
        except Exception:
            pass

    for candidate in sorted(candidates, key=lambda x: x.priority, reverse=True):
        try:
            body, content_type = _http_get(candidate.url, max_bytes=MAX_BYTES)
            mime = (content_type.split(";", 1)[0] or "").strip().lower()
            if mime and not mime.startswith("image/") and mime != "application/octet-stream":
                continue

            ext = _guess_extension(candidate.url, content_type)
            out_path = OUT_DIR / f"{domain_key}{ext}"

            for old in OUT_DIR.glob(f"{domain_key}.*"):
                try:
                    old.unlink()
                except OSError:
                    pass

            out_path.write_bytes(body)
            return out_path.name
        except (urllib.error.URLError, urllib.error.HTTPError, TimeoutError, ValueError):
            continue

    return None


def _reorder_bookmark_item_keys(item: dict) -> None:
    if "url" in item and "favicon" in item:
        url = item.get("url")
        favicon = item.get("favicon")
        rest = {k: v for k, v in item.items() if k not in {"url", "favicon"}}
        item.clear()
        item["url"] = url
        item["favicon"] = favicon
        item.update(rest)


def main() -> int:
    try:
        data = json.loads(BOOKMARKS_PATH.read_text(encoding="utf-8"))
    except FileNotFoundError:
        print(f"error: input not found: {BOOKMARKS_PATH}", file=sys.stderr)
        return 2
    except json.JSONDecodeError as e:
        print(f"error: invalid json in {BOOKMARKS_PATH}: {e}", file=sys.stderr)
        return 2

    if not isinstance(data, list):
        print(f"error: expected a JSON list in {BOOKMARKS_PATH}", file=sys.stderr)
        return 2

    entries: list[dict] = []
    for entry in data:
        if not isinstance(entry, dict):
            continue
        url = entry.get("url")
        if isinstance(url, str) and url:
            entries.append(entry)

    total = len(entries)
    ok = 0
    failed = 0
    updated = 0
    interrupted = False
    try:
        for idx, entry in enumerate(entries, start=1):
            url = entry.get("url")
            assert isinstance(url, str)
            print(f"[{idx}/{total}] fetching favicon: {url}", flush=True)

            filename = download_favicon(url)
            if not filename:
                failed += 1
                print(f"[{idx}/{total}] fail", flush=True)
                continue

            ok += 1
            if entry.get("favicon") != filename:
                entry["favicon"] = filename
                updated += 1
            _reorder_bookmark_item_keys(entry)
            print(f"[{idx}/{total}] ok: {filename}", flush=True)
    except KeyboardInterrupt:
        interrupted = True
        print("\ninterrupted: writing partial updates...", flush=True)

    BOOKMARKS_PATH.write_text(json.dumps(data, indent=4, ensure_ascii=False) + "\n", encoding="utf-8")
    print(f"done: {ok} ok, {failed} failed, out_dir={OUT_DIR}")
    print(f"bookmarks: updated {updated} items (favicon field)")
    if interrupted:
        return 130
    return 0 if failed == 0 else 1


if __name__ == "__main__":
    raise SystemExit(main())
