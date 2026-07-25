#!/usr/bin/env python3
"""Rasterize SVG (via LibreOffice Draw if needed) and PPTX slide, report pixel delta.

Usage:
  svg_pptx_pixel_diff.py slide.svg slide.pptx [--out-dir /tmp/diff]
"""
from __future__ import annotations

import argparse
import os
import shutil
import subprocess
import sys
import tempfile
from pathlib import Path

try:
    from PIL import Image, ImageChops, ImageStat
except ImportError:
    print("Pillow required: pip install pillow", file=sys.stderr)
    sys.exit(2)


def run(cmd, **kw):
    return subprocess.run(cmd, check=True, capture_output=True, text=True, **kw)


def soffice_convert(src: Path, outdir: Path, fmt: str = "png") -> Path:
    outdir.mkdir(parents=True, exist_ok=True)
    run(
        [
            "soffice",
            "--headless",
            "--convert-to",
            fmt,
            "--outdir",
            str(outdir),
            str(src),
        ]
    )
    # LO names output after stem
    cand = outdir / f"{src.stem}.png"
    if cand.exists():
        return cand
    pngs = list(outdir.glob("*.png"))
    if not pngs:
        raise FileNotFoundError(f"no png from {src}")
    return pngs[0]


def raster_svg_rsvg(svg: Path, dest: Path, size=(1280, 720)) -> Path:
    """Preferred: librsvg via PyGObject (browser-like)."""
    import gi

    gi.require_version("Rsvg", "2.0")
    from gi.repository import Rsvg
    import cairo

    data = svg.read_bytes()
    handle = Rsvg.Handle.new_from_data(data)
    w, h = size
    surface = cairo.ImageSurface(cairo.FORMAT_ARGB32, w, h)
    ctx = cairo.Context(surface)
    vp = Rsvg.Rectangle()
    vp.x = 0
    vp.y = 0
    vp.width = w
    vp.height = h
    handle.render_document(ctx, vp)
    dest.parent.mkdir(parents=True, exist_ok=True)
    surface.write_to_png(str(dest))
    return dest


def raster_svg_chrome(svg: Path, dest: Path, size=(1280, 720)) -> Path:
    """Chromium headless screenshot — closest to browser ground truth."""
    import shutil
    import subprocess

    chrome = shutil.which("google-chrome") or shutil.which("chromium") or shutil.which(
        "chromium-browser"
    )
    if not chrome:
        raise RuntimeError("chrome not found")
    dest.parent.mkdir(parents=True, exist_ok=True)
    w, h = size
    uri = svg.resolve().as_uri()
    subprocess.run(
        [
            chrome,
            "--headless",
            "--disable-gpu",
            f"--screenshot={dest}",
            f"--window-size={w},{h}",
            uri,
        ],
        check=True,
        capture_output=True,
    )
    if not dest.exists():
        raise RuntimeError("chrome produced no screenshot")
    return dest


def raster_svg(svg: Path, outdir: Path, size=(1280, 720)) -> Path:
    """Best-effort SVG raster. Prefer Chrome, then librsvg, then LibreOffice."""
    dest = outdir / "svg" / f"{svg.stem}.png"
    errors = []
    for fn in (raster_svg_chrome, raster_svg_rsvg):
        try:
            return fn(svg, dest, size)
        except Exception as e:
            errors.append(f"{fn.__name__}: {e}")
    try:
        return soffice_convert(svg, outdir / "svg")
    except Exception as e:
        errors.append(f"soffice: {e}")
        raise RuntimeError("SVG raster failed: " + "; ".join(errors)) from e


def mean_abs_diff(a: Image.Image, b: Image.Image) -> tuple[float, float]:
    resample = getattr(Image, "Resampling", Image).BILINEAR
    a = a.convert("RGB").resize((1280, 720), resample)
    b = b.convert("RGB").resize((1280, 720), resample)
    diff = ImageChops.difference(a, b)
    stat = ImageStat.Stat(diff)
    # mean of R,G,B channels
    mad = sum(stat.mean) / 3.0
    # fraction of pixels with any channel delta > 16
    px = list(diff.getdata())
    n = len(px)
    changed = sum(1 for r, g, b in px if r > 16 or g > 16 or b > 16)
    return mad, changed / max(n, 1)


def main():
    ap = argparse.ArgumentParser()
    ap.add_argument("svg")
    ap.add_argument("pptx")
    ap.add_argument("--out-dir", default="/tmp/svg-pptx-diff")
    args = ap.parse_args()
    out = Path(args.out_dir)
    if out.exists():
        shutil.rmtree(out)
    out.mkdir(parents=True)

    svg = Path(args.svg)
    pptx = Path(args.pptx)
    svg_png = raster_svg(svg, out)
    pptx_png = soffice_convert(pptx, out / "pptx")

    # copy for inspection
    shutil.copy(svg_png, out / "ref-svg.png")
    shutil.copy(pptx_png, out / "pptx.png")

    a = Image.open(out / "ref-svg.png")
    b = Image.open(out / "pptx.png")
    mad, frac = mean_abs_diff(a, b)
    # write abs-diff preview
    resample = getattr(Image, "Resampling", Image).BILINEAR
    a_r = a.convert("RGB").resize((1280, 720), resample)
    b_r = b.convert("RGB").resize((1280, 720), resample)
    ImageChops.difference(a_r, b_r).save(out / "diff.png")

    print(f"svg_png={svg_png}")
    print(f"pptx_png={pptx_png}")
    print(f"mean_abs_diff={mad:.2f}")
    print(f"changed_frac={frac*100:.2f}%")
    # soft threshold guidance (not a hard fail for LO font/AA differences)
    if mad < 12 and frac < 0.15:
        print("verdict=close")
        return 0
    if mad < 25 and frac < 0.35:
        print("verdict=acceptable")
        return 0
    print("verdict=divergent")
    return 1


if __name__ == "__main__":
    sys.exit(main())
