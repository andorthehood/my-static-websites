#!/usr/bin/env python3
from __future__ import annotations

import argparse
import json
import re
import sys
import urllib.parse
import urllib.request
from html.parser import HTMLParser
from pathlib import Path
from typing import Optional

BOOKMARKS_PATH = Path("sites/polgarand.org/data/bookmarks.json")
TIMEOUT_S = 15.0
MAX_BYTES = 1_500_000
USER_AGENT = "polgarand.org bookmark-fetcher/1.0"


def _clean_text(text: str) -> str:
    text = re.sub(r"\s+", " ", text).strip()
    return text


def _normalize_url(raw_url: str) -> str:
    candidate = raw_url.strip()
    parsed = urllib.parse.urlparse(candidate)
    if not parsed.scheme and parsed.path and "." in parsed.path:
        candidate = f"https://{candidate}"
        parsed = urllib.parse.urlparse(candidate)

    if parsed.scheme not in {"http", "https"} or not parsed.netloc:
        raise ValueError(f"unsupported or invalid URL: {raw_url}")

    normalized = parsed._replace(fragment="")
    return urllib.parse.urlunparse(normalized)


def _fallback_title(url: str) -> str:
    parsed = urllib.parse.urlparse(url)
    host = parsed.netloc
    path = parsed.path.rstrip("/")
    if path:
        return f"{host}{path}"
    return host


def _fallback_description(url: str) -> str:
    parsed = urllib.parse.urlparse(url)
    return f"Bookmark for {parsed.netloc}"


class _MetadataParser(HTMLParser):
    def __init__(self) -> None:
        super().__init__()
        self._in_title = False
        self._title_parts: list[str] = []
        self.meta_title: Optional[str] = None
        self.meta_description: Optional[str] = None
        self.og_title: Optional[str] = None
        self.og_description: Optional[str] = None
        self.twitter_title: Optional[str] = None
        self.twitter_description: Optional[str] = None

    def handle_starttag(self, tag: str, attrs: list[tuple[str, Optional[str]]]) -> None:
        tag = tag.lower()
        if tag == "title":
            self._in_title = True
            return
        if tag != "meta":
            return

        attr_map = {k.lower(): (v or "").strip() for k, v in attrs}
        name = attr_map.get("name", "").lower()
        prop = attr_map.get("property", "").lower()
        content = _clean_text(attr_map.get("content", ""))
        if not content:
            return

        if prop == "og:title":
            self.og_title = content
        elif prop == "og:description":
            self.og_description = content
        elif name == "twitter:title":
            self.twitter_title = content
        elif name == "twitter:description":
            self.twitter_description = content
        elif name == "description":
            self.meta_description = content
        elif name == "title":
            self.meta_title = content

    def handle_endtag(self, tag: str) -> None:
        if tag.lower() == "title":
            self._in_title = False

    def handle_data(self, data: str) -> None:
        if self._in_title:
            self._title_parts.append(data)

    def get_title(self) -> Optional[str]:
        title_tag = _clean_text("".join(self._title_parts))
        for value in (self.og_title, self.twitter_title, title_tag, self.meta_title):
            if value:
                return value
        return None

    def get_description(self) -> Optional[str]:
        for value in (self.og_description, self.twitter_description, self.meta_description):
            if value:
                return value
        return None


def _fetch_metadata(url: str) -> tuple[str, str]:
    req = urllib.request.Request(
        url,
        headers={
            "User-Agent": USER_AGENT,
            "Accept": "text/html,application/xhtml+xml;q=0.9,*/*;q=0.8",
        },
    )

    with urllib.request.urlopen(req, timeout=TIMEOUT_S) as resp:
        body = resp.read(MAX_BYTES + 1)
        if len(body) > MAX_BYTES:
            raise ValueError(f"response too large (> {MAX_BYTES} bytes)")
        charset = resp.headers.get_content_charset() or "utf-8"
        html = body.decode(charset, errors="replace")

    parser = _MetadataParser()
    parser.feed(html)
    title = parser.get_title() or _fallback_title(url)
    description = parser.get_description() or _fallback_description(url)
    return title, description


def _normalize_for_match(url: str) -> str:
    parsed = urllib.parse.urlparse(url)
    path = parsed.path.rstrip("/")
    if not path:
        path = "/"
    normalized = parsed._replace(
        scheme=parsed.scheme.lower(),
        netloc=parsed.netloc.lower(),
        path=path,
        fragment="",
    )
    return urllib.parse.urlunparse(normalized)


def _find_existing_index(bookmarks: list[dict], normalized_url: str) -> Optional[int]:
    for idx, item in enumerate(bookmarks):
        if not isinstance(item, dict):
            continue
        raw = item.get("url")
        if not isinstance(raw, str):
            continue
        try:
            if _normalize_for_match(_normalize_url(raw)) == normalized_url:
                return idx
        except ValueError:
            continue
    return None


def _reorder_keys(item: dict) -> None:
    fields = ["url", "title", "description"]
    ordered = {}
    for key in fields:
        if key in item:
            ordered[key] = item[key]
    for key, value in item.items():
        if key not in ordered:
            ordered[key] = value
    item.clear()
    item.update(ordered)


def _load_bookmarks() -> list[dict]:
    try:
        data = json.loads(BOOKMARKS_PATH.read_text(encoding="utf-8"))
    except FileNotFoundError as exc:
        raise RuntimeError(f"input not found: {BOOKMARKS_PATH}") from exc
    except json.JSONDecodeError as exc:
        raise RuntimeError(f"invalid json in {BOOKMARKS_PATH}: {exc}") from exc

    if not isinstance(data, list):
        raise RuntimeError(f"expected a JSON list in {BOOKMARKS_PATH}")
    return data


def _save_bookmarks(data: list[dict]) -> None:
    BOOKMARKS_PATH.write_text(
        json.dumps(data, indent=4, ensure_ascii=False) + "\n",
        encoding="utf-8",
    )


def main() -> int:
    parser = argparse.ArgumentParser(
        description="Add or update a bookmark in sites/polgarand.org/data/bookmarks.json",
    )
    parser.add_argument("url", help="Bookmark URL")
    args = parser.parse_args()

    try:
        normalized_url = _normalize_url(args.url)
        normalized_for_match = _normalize_for_match(normalized_url)
        data = _load_bookmarks()
    except (RuntimeError, ValueError) as exc:
        print(f"error: {exc}", file=sys.stderr)
        return 2

    try:
        title, description = _fetch_metadata(normalized_url)
    except Exception as exc:
        print(f"warn: metadata fetch failed, using fallback values ({exc})", file=sys.stderr)
        title = _fallback_title(normalized_url)
        description = _fallback_description(normalized_url)

    existing_index = _find_existing_index(data, normalized_for_match)
    if existing_index is None:
        entry: dict = {"url": normalized_url}
        data.insert(0, entry)
        action = "added"
    else:
        entry = data[existing_index]
        action = "updated"

    entry["url"] = normalized_url
    entry["title"] = _clean_text(title)
    entry["description"] = _clean_text(description)
    _reorder_keys(entry)

    _save_bookmarks(data)

    print(f"{action}: {normalized_url}")
    print(f"title: {entry['title']}")
    print(f"description: {entry['description']}")
    print(f"bookmarks_path: {BOOKMARKS_PATH}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
