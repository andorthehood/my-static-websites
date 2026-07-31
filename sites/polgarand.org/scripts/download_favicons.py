#!/usr/bin/env python3
from __future__ import annotations

# Purpose: download favicons for bookmarks without a `favicon` field in
# `sites/polgarand.org/data/bookmarks.json`, save them into
# `sites/polgarand.org/.favicons/` (gitignored) using the domain as the filename,
# and write the resulting filename back onto each bookmark entry.

import json
import os
import re
import socket
import sys
from io import BytesIO
import urllib.error
import urllib.parse
import urllib.request
from dataclasses import dataclass
from html.parser import HTMLParser
from pathlib import Path
from typing import Optional

import boto3
import cairosvg
from botocore.exceptions import BotoCoreError, ClientError
from dotenv import load_dotenv
from PIL import Image, UnidentifiedImageError

BOOKMARKS_PATH = Path("sites/polgarand.org/data/bookmarks.json")
OUT_DIR = Path("sites/polgarand.org/.favicons")
DOTENV_PATH = Path("sites/polgarand.org/.env")

TIMEOUT_S = 10.0
MAX_BYTES = 1_000_000
USER_AGENT = "polgarand.org favicon-fetcher/1.0"
FAVICON_SIZE = (16, 16)
R2_FAVICON_DIR = "favicons"
CACHE_PURGE_PREFIX = "cdn.polgarand.org/favicons"


@dataclass(frozen=True)
class R2Config:
    account_id: str
    access_key_id: str
    secret_access_key: str
    bucket_name: str


@dataclass(frozen=True)
class CloudflareConfig:
    zone_id: str
    api_token: str


def _safe_domain_filename(netloc: str) -> str:
    domain = netloc.split("@")[-1]
    domain = domain.split(":")[0]
    domain = domain.strip().lower()
    return re.sub(r"[^a-z0-9.-]+", "_", domain).strip("._") or "unknown"


@dataclass(frozen=True)
class FaviconCandidate:
    url: str
    priority: int
    size_area: Optional[int] = None


def _load_r2_config() -> Optional[R2Config]:
    load_dotenv(DOTENV_PATH)
    names = (
        "R2_ACCOUNT_ID",
        "R2_ACCESS_KEY_ID",
        "R2_SECRET_ACCESS_KEY",
        "R2_BUCKET_NAME",
    )
    values = {name: os.environ.get(name, "").strip() for name in names}
    if not any(values.values()):
        return None

    missing = [name for name, value in values.items() if not value]
    if missing:
        raise ValueError(f"missing R2 configuration: {', '.join(missing)}")

    return R2Config(
        account_id=values["R2_ACCOUNT_ID"],
        access_key_id=values["R2_ACCESS_KEY_ID"],
        secret_access_key=values["R2_SECRET_ACCESS_KEY"],
        bucket_name=values["R2_BUCKET_NAME"],
    )


def _load_cloudflare_config() -> Optional[CloudflareConfig]:
    load_dotenv(DOTENV_PATH)
    names = ("CACHE_PURGE_ZONE_ID", "CACHE_PURGE_API_TOKEN")
    values = {name: os.environ.get(name, "").strip() for name in names}
    if not any(values.values()):
        return None

    missing = [name for name, value in values.items() if not value]
    if missing:
        raise ValueError(f"missing Cloudflare configuration: {', '.join(missing)}")

    return CloudflareConfig(
        zone_id=values["CACHE_PURGE_ZONE_ID"],
        api_token=values["CACHE_PURGE_API_TOKEN"],
    )


def _upload_favicons(config: R2Config) -> int:
    paths = sorted(OUT_DIR.glob("*.png"))
    client = boto3.client(
        "s3",
        endpoint_url=f"https://{config.account_id}.r2.cloudflarestorage.com",
        aws_access_key_id=config.access_key_id,
        aws_secret_access_key=config.secret_access_key,
        region_name="auto",
    )

    total = len(paths)
    for idx, path in enumerate(paths, start=1):
        object_key = f"{R2_FAVICON_DIR}/{path.name}"
        print(f"[{idx}/{total}] uploading favicon: {object_key}", flush=True)
        client.put_object(
            Bucket=config.bucket_name,
            Key=object_key,
            Body=path.read_bytes(),
            ContentType="image/png",
        )
    return total


def _purge_favicon_cache(config: CloudflareConfig) -> None:
    url = f"https://api.cloudflare.com/client/v4/zones/{config.zone_id}/purge_cache"
    request = urllib.request.Request(
        url,
        data=json.dumps({"prefixes": [CACHE_PURGE_PREFIX]}).encode("utf-8"),
        headers={
            "Authorization": f"Bearer {config.api_token}",
            "Content-Type": "application/json",
        },
        method="POST",
    )
    with urllib.request.urlopen(request, timeout=TIMEOUT_S) as response:
        result = json.loads(response.read().decode("utf-8"))
    if not result.get("success"):
        errors = result.get("errors") or []
        message = "; ".join(str(error.get("message", error)) for error in errors)
        raise ValueError(message or "Cloudflare rejected the cache purge")


class _IconLinkParser(HTMLParser):
    def __init__(self) -> None:
        super().__init__()
        self.candidates: list[FaviconCandidate] = []

    @staticmethod
    def _parse_size_area(value: str) -> Optional[int]:
        sizes = (value or "").strip().lower()
        if not sizes or sizes == "any":
            return None

        best: Optional[int] = None
        for token in sizes.split():
            if "x" not in token:
                continue
            left, right = token.split("x", 1)
            try:
                w = int(left)
                h = int(right)
            except ValueError:
                continue
            if w <= 0 or h <= 0:
                continue
            area = w * h
            if best is None or area < best:
                best = area
        return best

    def handle_starttag(self, tag: str, attrs: list[tuple[str, Optional[str]]]) -> None:
        if tag.lower() != "link":
            return
        attr_map = {k.lower(): (v or "") for k, v in attrs}
        rel = attr_map.get("rel", "").lower()
        href = attr_map.get("href", "").strip()
        size_area = self._parse_size_area(attr_map.get("sizes", ""))
        if not href:
            return

        if "icon" not in rel:
            return

        priority = 100
        if "shortcut" in rel:
            priority = 90
        if "apple-touch-icon" in rel:
            priority = 80
        self.candidates.append(FaviconCandidate(url=href, priority=priority, size_area=size_area))


def _http_get(url: str, *, max_bytes: int) -> tuple[bytes, str]:
    req = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    with urllib.request.urlopen(req, timeout=TIMEOUT_S) as resp:
        content_type = resp.headers.get("Content-Type", "")
        body = resp.read(max_bytes + 1)
        if len(body) > max_bytes:
            raise ValueError(f"response too large (> {max_bytes} bytes): {url}")
        return body, content_type


def _resize_raster_favicon(body: bytes) -> bytes:
    with Image.open(BytesIO(body)) as image:
        image.seek(0)
        resized = image.convert("RGBA").resize(FAVICON_SIZE, Image.Resampling.LANCZOS)
        output = BytesIO()
        resized.save(output, format="PNG")
        return output.getvalue()


def _resize_favicon(body: bytes) -> bytes:
    try:
        return _resize_raster_favicon(body)
    except (UnidentifiedImageError, OSError):
        try:
            svg_png = cairosvg.svg2png(
                bytestring=body,
                output_width=FAVICON_SIZE[0],
                output_height=FAVICON_SIZE[1],
            )
            return _resize_raster_favicon(svg_png)
        except Exception as error:
            raise ValueError("unsupported or invalid favicon image") from error


def _discover_icons_from_html(base_url: str, html: str) -> list[FaviconCandidate]:
    parser = _IconLinkParser()
    try:
        parser.feed(html)
    except Exception:
        return []

    out: list[FaviconCandidate] = []
    for c in parser.candidates:
        out.append(
            FaviconCandidate(
                url=urllib.parse.urljoin(base_url, c.url),
                priority=c.priority,
                size_area=c.size_area,
            )
        )
    return out


def _default_candidates(page_url: str) -> list[FaviconCandidate]:
    parsed = urllib.parse.urlparse(page_url)
    origin = f"{parsed.scheme}://{parsed.netloc}"
    return [
        FaviconCandidate(url=urllib.parse.urljoin(origin, "/favicon.ico"), priority=10),
        FaviconCandidate(url=urllib.parse.urljoin(origin, "/favicon.png"), priority=9),
        FaviconCandidate(url=urllib.parse.urljoin(origin, "/apple-touch-icon.png"), priority=5),
        FaviconCandidate(url=urllib.parse.urljoin(page_url, "favicon.ico"), priority=8),
        FaviconCandidate(url=urllib.parse.urljoin(page_url, "favicon.png"), priority=7),
        FaviconCandidate(url=urllib.parse.urljoin(page_url, "apple-touch-icon.png"), priority=4),
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

    def candidate_sort_key(c: FaviconCandidate) -> tuple[int, int]:
        size_area = c.size_area if c.size_area is not None else 2_147_483_647
        return (-c.priority, size_area)

    for candidate in sorted(candidates, key=candidate_sort_key):
        try:
            body, content_type = _http_get(candidate.url, max_bytes=MAX_BYTES)
            mime = (content_type.split(";", 1)[0] or "").strip().lower()
            if mime and not mime.startswith("image/") and mime != "application/octet-stream":
                continue

            resized_body = _resize_favicon(body)
            out_path = OUT_DIR / f"{domain_key}.png"

            for old in OUT_DIR.glob(f"{domain_key}.*"):
                try:
                    old.unlink()
                except OSError:
                    pass

            out_path.write_bytes(resized_body)
            return out_path.name
        except (urllib.error.URLError, urllib.error.HTTPError, socket.timeout, TimeoutError, ValueError):
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
        r2_config = _load_r2_config()
        cloudflare_config = _load_cloudflare_config()
    except ValueError as error:
        print(f"error: {error}", file=sys.stderr)
        return 2

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
    skipped = 0
    for entry in data:
        if not isinstance(entry, dict):
            continue
        url = entry.get("url")
        if isinstance(url, str) and url:
            if "favicon" in entry:
                skipped += 1
            else:
                entries.append(entry)

    total = len(entries)
    print(f"bookmarks: {total} missing favicon, {skipped} skipped", flush=True)
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

    if r2_config is None:
        print(f"uploads: skipped (configure {DOTENV_PATH} to enable R2 uploads)")
    else:
        try:
            uploaded = _upload_favicons(r2_config)
        except (BotoCoreError, ClientError, OSError) as error:
            print(f"error: R2 upload failed: {error}", file=sys.stderr)
            return 2
        print(
            f"uploads: uploaded {uploaded} files to "
            f"r2://{r2_config.bucket_name}/{R2_FAVICON_DIR}/"
        )
        if cloudflare_config is None:
            print(
                f"cache: skipped (configure {DOTENV_PATH} to enable cache purging)"
            )
        else:
            try:
                _purge_favicon_cache(cloudflare_config)
            except (
                urllib.error.URLError,
                urllib.error.HTTPError,
                socket.timeout,
                TimeoutError,
                ValueError,
            ) as error:
                print(f"error: cache purge failed: {error}", file=sys.stderr)
                return 2
            print(f"cache: purged prefix {CACHE_PURGE_PREFIX}")

    return 0 if failed == 0 else 1


if __name__ == "__main__":
    raise SystemExit(main())
