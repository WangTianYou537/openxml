#!/usr/bin/env python3
"""Subset a TTF to the given Unicode codepoints (hex or chars on stdin).

Usage:
  subset_ttf.py in.ttf out.ttf 4E00 4E8C ...
  subset_ttf.py in.ttf out.ttf --chars '中文ABC'
"""
from __future__ import annotations
import sys
from pathlib import Path

def main():
    if len(sys.argv) < 3:
        print("usage: subset_ttf.py in.ttf out.ttf [codepoints...|--chars TEXT]", file=sys.stderr)
        return 2
    src, dst = sys.argv[1], sys.argv[2]
    args = sys.argv[3:]
    unicodes = set()
    # Always keep basic ASCII printable + common punctuation for editability.
    unicodes.update(range(0x20, 0x7F))
    unicodes.update(ord(c) for c in "，。：；？！（）【】《》—…·、""''℃%‰±×÷\t\n")
    i = 0
    while i < len(args):
        a = args[i]
        if a == "--chars" and i + 1 < len(args):
            unicodes.update(ord(c) for c in args[i + 1])
            i += 2
            continue
        if a.startswith("U+") or a.startswith("0x"):
            unicodes.add(int(a.replace("U+", "0x"), 16))
        else:
            try:
                unicodes.add(int(a, 16) if all(c in "0123456789abcdefABCDEF" for c in a) else ord(a))
            except Exception:
                unicodes.update(ord(c) for c in a)
        i += 1

    from fontTools import subset
    options = subset.Options()
    options.layout_features = "*"
    options.name_IDs = "*"
    options.name_legacy = True
    options.name_languages = "*"
    options.notdef_outline = True
    options.recalc_bounds = True
    options.recalc_timestamp = False
    options.canonical_order = True
    font = subset.load_font(src, options)
    subsetter = subset.Subsetter(options=options)
    subsetter.populate(unicodes=sorted(unicodes))
    subsetter.subset(font)
    # Keep installable embedding
    if "OS/2" in font:
        font["OS/2"].fsType = 0
    subset.save_font(font, dst, options)
    print(f"subset {src} -> {dst} glyphs={font['maxp'].numGlyphs} bytes={Path(dst).stat().st_size}")
    return 0

if __name__ == "__main__":
    raise SystemExit(main())
