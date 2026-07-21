#!/usr/bin/env python3
"""Extract Schematron rules from C# schematrons.json into Rust tables.

Generates:
  - src/validation/schematron_rules.rs       (relationship + unique-attribute)
  - src/validation/schematron_constraints.rs  (numeric range + string length + matches)
"""
from __future__ import annotations

import argparse
import json
import re
from pathlib import Path


def local(q: str) -> str:
    return q.split(":")[-1]


REL_ID = re.compile(
    r"document\(rels\)//r:Relationship\[@Id\s*=\s*current\(\)/@([\w:]+)\]", re.I
)
REL_TYPE = re.compile(r"@Type\s*=\s*'([^']+)'", re.I)
UNIQUE_LOWER = re.compile(
    r"count\(distinct-values\(lower-case\(//([\w:]+)/@([\w:]+)\)\)\)\s*=\s*count\(lower-case\(//([\w:]+)/@([\w:]+)\)\)"
)
UNIQUE = re.compile(
    r"count\(distinct-values\(//([\w:]+)/@([\w:]+)\)\)\s*=\s*count\(//([\w:]+)/@([\w:]+)\)"
)
# Path-prefixed unique: count(distinct-values(//a/b/c/@id)) = count(//a/b/c/@id)
# Also lower-case(...) variants. Extract leaf element + attribute.
UNIQUE_PATH = re.compile(
    r"count\(distinct-values\((?:lower-case\()?(//(?:[\w:]+/)*[\w:]+/@[\w:]+)\)?\)\)"
    r"\s*=\s*count\((?:lower-case\()?(//(?:[\w:]+/)*[\w:]+/@[\w:]+)\)?\)",
    re.I,
)
PATH_LEAF_ATTR = re.compile(r"//(?:[\w:]+/)*([\w:]+)/@([\w:]+)")

# Scoped uniqueness under an ancestor:
# count(distinct-values(ancestor::X//Y/@a)) = count(ancestor::X//Y/@a)
ANCESTOR_UNIQUE = re.compile(
    r"count\(distinct-values\((?:lower-case\()?"
    r"ancestor::([\w:]+)//([\w:]+)/@([\w:]+)\)?\)\)"
    r"\s*=\s*count\((?:lower-case\()?"
    r"ancestor::([\w:]+)//([\w:]+)/@([\w:]+)\)?\)",
    re.I,
)

# Implication: (@requiredAttr and @flag = val) or @flag != val
IMPLICATION = re.compile(
    r"^\(@([\w:]+)\s+and\s+@([\w:]+)\s*=\s*"
    r"(?:'([^']*)'|\"([^\"]*)\"|([\w]+))\)"
    r"\s+or\s+@\2\s*!=\s*"
    r"(?:'([^']*)'|\"([^\"]*)\"|([\w]+))$",
    re.I,
)

# @guid != nil UUID
NONZERO_GUID = re.compile(
    r"^@([\w:]+)\s*!=\s*00000000-0000-0000-0000-000000000000$",
    re.I,
)

# @attr >= N / <= M  (also number(@attr) / xs:integer(@attr) casts)
NUM_GE = re.compile(
    r"(?:(?:number|xs:(?:integer|double|decimal|float))\(\s*)?@([\w:]+)\s*\)?\s*>=\s*(-?[\d.]+(?:[eE][+-]?\d+)?)",
    re.I,
)
NUM_LE = re.compile(
    r"(?:(?:number|xs:(?:integer|double|decimal|float))\(\s*)?@([\w:]+)\s*\)?\s*<=\s*(-?[\d.]+(?:[eE][+-]?\d+)?)",
    re.I,
)
NUM_GT = re.compile(
    r"(?:(?:number|xs:(?:integer|double|decimal|float))\(\s*)?@([\w:]+)\s*\)?\s*>\s*(-?[\d.]+(?:[eE][+-]?\d+)?)",
    re.I,
)
NUM_LT = re.compile(
    r"(?:(?:number|xs:(?:integer|double|decimal|float))\(\s*)?@([\w:]+)\s*\)?\s*<\s*(-?[\d.]+(?:[eE][+-]?\d+)?)",
    re.I,
)
# string-length(@attr) >= N / <= M
STR_GE = re.compile(r"string-length\(\s*@([\w:]+)\s*\)\s*>=\s*(\d+)", re.I)
STR_LE = re.compile(r"string-length\(\s*@([\w:]+)\s*\)\s*<=\s*(\d+)", re.I)
STR_GT = re.compile(r"string-length\(\s*@([\w:]+)\s*\)\s*>\s*(\d+)", re.I)
STR_LT = re.compile(r"string-length\(\s*@([\w:]+)\s*\)\s*<\s*(\d+)", re.I)
# matches(@attr, 'pattern') or matches(@attr, "pattern")
MATCHES = re.compile(r"matches\(\s*@([\w:]+)\s*,\s*['\"]([^'\"]+)['\"]\s*\)", re.I)

# Skip tests that reference other axes / functions we can't evaluate without XPath
SKIP_IF = re.compile(
    r"index-of|document\(|for\s+\$|every\s+\$|some\s+\$|ancestor::|following::|preceding::|parent::|//",
    re.I,
)


def rust_f64(v: float) -> str:
    if v == float("inf"):
        return "f64::INFINITY"
    if v == float("-inf"):
        return "f64::NEG_INFINITY"
    # Prefer plain decimal when exact
    if abs(v - int(v)) < 1e-12 and abs(v) < 1e15:
        return f"{int(v)}.0"
    s = repr(float(v))
    if s in ("inf", "Infinity"):
        return "f64::INFINITY"
    if s in ("-inf", "-Infinity"):
        return "f64::NEG_INFINITY"
    return s if ("." in s or "e" in s or "E" in s) else f"{s}.0"


def rust_usize(n: int, is_max: bool = False) -> str:
    if is_max and n >= 2**63:
        return "usize::MAX"
    return str(n)


def extract_numeric(test: str, ctx: str, app: str):
    """Return list of (element, attr, min, max, app) if test is pure numeric bounds on attrs."""
    if SKIP_IF.search(test):
        return []
    # Collect per-attribute bounds
    bounds: dict[str, list[float | None]] = {}  # attr -> [min, max]

    def ensure(a: str):
        a = local(a)
        if a not in bounds:
            bounds[a] = [float("-inf"), float("inf")]
        return a

    for m in NUM_GE.finditer(test):
        a = ensure(m.group(1))
        bounds[a][0] = max(bounds[a][0], float(m.group(2)))
    for m in NUM_GT.finditer(test):
        a = ensure(m.group(1))
        # exclusive -> treat as min = v + tiny for ints we keep as v+1 if int-like
        v = float(m.group(2))
        if abs(v - int(v)) < 1e-12:
            bounds[a][0] = max(bounds[a][0], int(v) + 1)
        else:
            bounds[a][0] = max(bounds[a][0], v)
    for m in NUM_LE.finditer(test):
        a = ensure(m.group(1))
        bounds[a][1] = min(bounds[a][1], float(m.group(2)))
    for m in NUM_LT.finditer(test):
        a = ensure(m.group(1))
        v = float(m.group(2))
        if abs(v - int(v)) < 1e-12:
            bounds[a][1] = min(bounds[a][1], int(v) - 1)
        else:
            bounds[a][1] = min(bounds[a][1], v)

    # Only accept if the test is essentially just these numeric predicates
    # (optionally joined by `and`, whitespace, and optional `@attr castable as ...`)
    cleaned = test
    for rx in (NUM_GE, NUM_LE, NUM_GT, NUM_LT):
        cleaned = rx.sub("", cleaned)
    cleaned = re.sub(
        r"@[\w:]+\s+castable\s+as\s+xs:\w+", "", cleaned, flags=re.I
    )
    cleaned = re.sub(r"\band\b|\bor\b|\(|\)|\s+", "", cleaned, flags=re.I)
    if cleaned not in ("", "true()", "true"):
        # leftover logic — skip
        return []
    out = []
    for attr, (lo, hi) in bounds.items():
        if lo == float("-inf") and hi == float("inf"):
            continue
        out.append((ctx, attr, lo, hi, app))
    return out


def extract_string_len(test: str, ctx: str, app: str):
    if SKIP_IF.search(test):
        return []
    bounds: dict[str, list[int | None]] = {}

    def ensure(a: str):
        a = local(a)
        if a not in bounds:
            bounds[a] = [0, None]  # min, max (None = unbounded)
        return a

    for m in STR_GE.finditer(test):
        a = ensure(m.group(1))
        bounds[a][0] = max(bounds[a][0] or 0, int(m.group(2)))
    for m in STR_GT.finditer(test):
        a = ensure(m.group(1))
        bounds[a][0] = max(bounds[a][0] or 0, int(m.group(2)) + 1)
    for m in STR_LE.finditer(test):
        a = ensure(m.group(1))
        v = int(m.group(2))
        bounds[a][1] = v if bounds[a][1] is None else min(bounds[a][1], v)
    for m in STR_LT.finditer(test):
        a = ensure(m.group(1))
        v = int(m.group(2)) - 1
        bounds[a][1] = v if bounds[a][1] is None else min(bounds[a][1], v)

    if not bounds:
        return []
    cleaned = test
    for rx in (STR_GE, STR_LE, STR_GT, STR_LT):
        cleaned = rx.sub("", cleaned)
    cleaned = re.sub(r"\band\b|\bor\b|\(|\)|\s+", "", cleaned, flags=re.I)
    if cleaned not in ("", "true()", "true"):
        return []
    out = []
    for attr, (lo, hi) in bounds.items():
        if hi is None and lo == 0:
            continue
        out.append((ctx, attr, lo or 0, hi, app))
    return out


def extract_matches(test: str, ctx: str, app: str):
    if SKIP_IF.search(test) and "matches(" not in test.lower():
        return []
    # Only pure matches(@attr, 'pat') optionally with not()
    ms = list(MATCHES.finditer(test))
    if not ms:
        return []
    cleaned = MATCHES.sub("", test)
    cleaned = re.sub(r"\bnot\b|\(|\)|\s+", "", cleaned, flags=re.I)
    if cleaned not in ("", "true()", "true"):
        # complex — skip (or accept if only matches)
        if re.search(r"[a-zA-Z]", cleaned):
            return []
    out = []
    for m in ms:
        pat = m.group(2)
        # skip unicode property classes we can't evaluate
        if r"\p{" in pat or r"\P{" in pat:
            continue
        out.append((ctx, local(m.group(1)), pat, app))
    return out


def extract_unique(test: str, app: str):
    """Return (element, attr, case_insensitive, app) or None."""
    m = UNIQUE_LOWER.search(test) or UNIQUE.search(test)
    if m:
        el, attr, el2, attr2 = map(local, m.groups())
        if el == el2 and attr == attr2:
            return (el, attr, "lower-case" in test, app)
    m = UNIQUE_PATH.search(test)
    if m:
        left, right = m.group(1), m.group(2)
        pl, pr = PATH_LEAF_ATTR.search(left), PATH_LEAF_ATTR.search(right)
        if pl and pr and pl.groups() == pr.groups():
            return (local(pl.group(1)), local(pl.group(2)), "lower-case" in test, app)
    return None


def extract_enum(test: str, ctx: str, app: str):
    """@attr = v1 or @attr = v2 or ... → enumeration constraint."""
    if "document(" in test or "count(" in test or "matches(" in test:
        return []
    if " or " not in test.lower():
        return []
    parts = re.split(r"\s+or\s+", test.strip(), flags=re.I)
    if len(parts) < 2:
        return []
    attrs: set[str] = set()
    vals: list[str] = []
    for p in parts:
        m = re.match(
            r"@([\w:]+)\s*=\s*(?:'([^']*)'|\"([^\"]*)\"|(-?[\w.]+))\s*$",
            p.strip(),
        )
        if not m:
            return []
        attrs.add(local(m.group(1)))
        vals.append(m.group(2) if m.group(2) is not None else (m.group(3) if m.group(3) is not None else m.group(4)))
    if len(attrs) != 1:
        return []
    return [(ctx, list(attrs)[0], tuple(vals), app)]


def extract_ancestor_unique(test: str, app: str):
    """Return (ancestor_el, element, attr, case_insensitive, app) or None."""
    m = ANCESTOR_UNIQUE.search(test)
    if not m:
        return None
    a1, e1, at1, a2, e2, at2 = map(local, m.groups())
    if (a1, e1, at1) != (a2, e2, at2):
        return None
    return (a1, e1, at1, "lower-case" in test, app)


def extract_implication(test: str, ctx: str, app: str):
    """(@req and @flag = val) or @flag != val → ConditionalAttrRule."""
    t = " ".join(test.split())
    m = IMPLICATION.match(t)
    if not m:
        return []
    req = local(m.group(1))
    flag = local(m.group(2))
    val = m.group(3) or m.group(4) or m.group(5)
    val2 = m.group(6) or m.group(7) or m.group(8)
    if val != val2:
        return []
    return [(ctx, req, flag, val, app)]


def extract_nonzero_guid(test: str, ctx: str, app: str):
    m = NONZERO_GUID.match(test.strip())
    if not m:
        return []
    return [(ctx, local(m.group(1)), app)]



# @a <= @b  (same-element attribute comparison)
ATTR_CMP = re.compile(
    r"^@([\w:]+)\s*(<=|>=|<|>)\s*@([\w:]+)$",
    re.I,
)

# @attr = true|false
FIXED_BOOL = re.compile(
    r"^@([\w:]+)\s*=\s*(true|false)$",
    re.I,
)

# Index-of(document('Part:...')//path/@attr, @localAttr)
CROSS_INDEX = re.compile(
    r"[Ii]ndex-of\(\s*document\(\s*['\"]Part:([^'\"]+)['\"]\s*\)"
    r"//((?:[\w:]+/)*[\w:]+)/@([\w:]+)\s*,\s*@([\w:]+)\s*\)",
)

# @attr < count(document('Part:...')//path) + N
CROSS_COUNT = re.compile(
    r"@([\w:]+)\s*<\s*count\(\s*document\(\s*['\"]Part:([^'\"]+)['\"]\s*\)"
    r"//((?:[\w:]+/)*[\w:]+)(?:/@[\w:]+)?\s*\)\s*(?:\+\s*(\d+))?",
    re.I,
)


def extract_attr_cmp(test: str, ctx: str, app: str):
    m = ATTR_CMP.match(test.strip())
    if not m:
        return []
    return [(ctx, local(m.group(1)), m.group(2), local(m.group(3)), app)]


def extract_fixed_bool(test: str, ctx: str, app: str):
    m = FIXED_BOOL.match(test.strip())
    if not m:
        return []
    return [(ctx, local(m.group(1)), m.group(2).lower() == "true", app)]


def extract_cross_index(test: str, ctx: str, app: str):
    m = CROSS_INDEX.search(test)
    if not m:
        return []
    part = m.group(1)
    path = m.group(2)
    path_attr = local(m.group(3))
    local_attr = local(m.group(4))
    leaf = local(path.split("/")[-1])
    return [(ctx, local_attr, part, leaf, path_attr, app)]


def extract_cross_count(test: str, ctx: str, app: str):
    m = CROSS_COUNT.search(test)
    if not m:
        return []
    cleaned = CROSS_COUNT.sub("", test.strip())
    cleaned = re.sub(r"\s+", "", cleaned)
    if cleaned not in ("",):
        return []
    attr = local(m.group(1))
    part = m.group(2)
    path = m.group(3)
    offset = int(m.group(4) or "0")
    leaf = local(path.split("/")[-1])
    return [(ctx, attr, part, leaf, offset, app)]



# @attr = literal (non-bool)
FIXED_VALUE = re.compile(
    r"^@([\w:]+)\s*=\s*(?:'([^']*)'|\"([^\"]*)\"|(-?[\w.]+))$",
    re.I,
)

# @attr != literal (single)
FIXED_NE = re.compile(
    r"^@([\w:]+)\s*!=\s*(?:'([^']*)'|\"([^\"]*)\"|(-?[\w.]+))$",
    re.I,
)

# @attr != a and @attr != b ...
MULTI_NE = re.compile(
    r"^@([\w:]+)\s*!=\s*(?:'([^']*)'|\"([^\"]*)\"|(-?[\w.]+))"
    r"(?:\s+and\s+@\1\s*!=\s*(?:'([^']*)'|\"([^\"]*)\"|(-?[\w.]+)))+$",
    re.I,
)

# both attributes must be present: @a and @b  (exactly two, no equals)
BOTH_PRESENT = re.compile(
    r"^@([\w:]+)\s+and\s+@([\w:]+)$",
    re.I,
)

# hex-ish range: @attr > 0 and @attr < 0x80000000
HEX_RANGE = re.compile(
    r"^@([\w:]+)\s*>\s*0\s+and\s+@\1\s*<\s*0x([0-9a-fA-F]+)$",
    re.I,
)

# float with trailing f: @attr >= -32767f and @attr <= 32767f
FLOAT_F_RANGE = re.compile(
    r"^@([\w:]+)\s*>=\s*(-?[\d.]+)f?\s+and\s+@\1\s*<=\s*(-?[\d.]+)f?$",
    re.I,
)

# NaN/INF exclusion: @attr != NaN and @attr != INF and @attr != -INF
NAN_INF = re.compile(
    r"^@([\w:]+)\s*!=\s*NaN\s+and\s+@\1\s*!=\s*INF\s+and\s+@\1\s*!=\s*-INF$",
    re.I,
)
NAN_INF2 = re.compile(
    r"^@([\w:]+)\s*!=\s*INF\s+and\s+@\1\s*!=\s*-INF\s+and\s+@\1\s*!=\s*NaN$",
    re.I,
)


def extract_fixed_value(test: str, ctx: str, app: str):
    t = test.strip()
    # skip bools (handled elsewhere) and multi-part
    if " and " in t.lower() or " or " in t.lower():
        return []
    m = FIXED_VALUE.match(t)
    if not m:
        return []
    val = m.group(2) if m.group(2) is not None else (m.group(3) if m.group(3) is not None else m.group(4))
    if val is None:
        return []
    if val.lower() in ("true", "false"):
        return []  # fixed bool
    return [(ctx, local(m.group(1)), val, app)]


def extract_fixed_ne(test: str, ctx: str, app: str):
    t = test.strip()
    if " and " in t.lower() or " or " in t.lower():
        return []
    m = FIXED_NE.match(t)
    if not m:
        return []
    val = m.group(2) if m.group(2) is not None else (m.group(3) if m.group(3) is not None else m.group(4))
    if val is None:
        return []
    return [(ctx, local(m.group(1)), val, app)]


def extract_multi_ne(test: str, ctx: str, app: str):
    t = " ".join(test.split())
    # parse manually: split by and, each @attr != val
    if " and " not in t.lower() or "!=" not in t:
        return []
    if " or " in t.lower() or "document(" in t or "count(" in t:
        return []
    parts = re.split(r"\s+and\s+", t, flags=re.I)
    if len(parts) < 2:
        return []
    attrs = set()
    vals = []
    for p in parts:
        m = re.match(
            r"@([\w:]+)\s*!=\s*(?:'([^']*)'|\"([^\"]*)\"|(-?[\w.]+))\s*$",
            p.strip(),
            re.I,
        )
        if not m:
            return []
        attrs.add(local(m.group(1)))
        vals.append(m.group(2) if m.group(2) is not None else (m.group(3) if m.group(3) is not None else m.group(4)))
    if len(attrs) != 1:
        return []
    return [(ctx, list(attrs)[0], tuple(vals), app)]


def extract_both_present(test: str, ctx: str, app: str):
    m = BOTH_PRESENT.match(test.strip())
    if not m:
        return []
    return [(ctx, local(m.group(1)), local(m.group(2)), app)]


def extract_hex_range(test: str, ctx: str, app: str):
    m = HEX_RANGE.match(" ".join(test.split()))
    if not m:
        return []
    hi = int(m.group(2), 16)
    # exclusive upper: attr < 0x80000000 → max = hi-1, min = 1
    return [(ctx, local(m.group(1)), 1.0, float(hi - 1), app)]


def extract_float_f_range(test: str, ctx: str, app: str):
    m = FLOAT_F_RANGE.match(" ".join(test.split()))
    if not m:
        return []
    return [(ctx, local(m.group(1)), float(m.group(2)), float(m.group(3)), app)]


def extract_nan_inf(test: str, ctx: str, app: str):
    t = " ".join(test.split())
    m = NAN_INF.match(t) or NAN_INF2.match(t)
    if not m:
        return []
    return [(ctx, local(m.group(1)), app)]



# bare @attr — attribute must be present
REQUIRED_ATTR = re.compile(r"^@([\w:]+)$")


def extract_required_attr(test: str, ctx: str, app: str):
    m = REQUIRED_ATTR.match(test.strip())
    if not m:
        return []
    return [(ctx, local(m.group(1)), app)]



# 1.15: @absent and @cond != v1 and @cond != v2 ...
# → absent must be missing when cond is not in {v1,v2,...}
ABSENT_WHEN_NOT = re.compile(
    r"^@([\w:]+)\s+and\s+@([\w:]+)\s*!=\s*(?:'([^']*)'|\"([^\"]*)\"|([\w.]+))"
    r"(?:\s+and\s+@\2\s*!=\s*(?:'([^']*)'|\"([^\"]*)\"|([\w.]+)))*$",
    re.I,
)

# 1.16: (@a and @b) or (@a and @c) or (@b and @c) → mutual exclusive attrs
MUTUAL_PAIR = re.compile(r"\(@([\w:]+)\s+and\s+@([\w:]+)\)", re.I)

# 1.14-ish: @a and @b = val  (already ATTR_AND_FLAG) — when b==val require a?
# C# AttributeRequiredConditionToValue is (@a and @b=v) or @b!=v which is IMPLICATION

# @req and @flag = val  (presence of both) — when flag equals val, req must be present
# Covered by IMPLICATION as (@req and @flag=val) or @flag!=val
# Residual forms without outer or: @hierarchy and @set = 1  means both present with set=1
# Treat as: if set present and set==1 then hierarchy required; if hierarchy present then set must be 1
# Simpler: assert both conditions — if either attr present, both conditions must hold
# Practical: required_when_eq already; for bare `@a and @b = 1`:
#   when b is 1, a must be present; when a present, b must be 1
ATTR_AND_EQ = re.compile(
    r"^@([\w:]+)\s+and\s+@([\w:]+)\s*=\s*(?:'([^']*)'|\"([^\"]*)\"|(-?[\w.]+))$",
    re.I,
)


def extract_absent_when_not(test: str, ctx: str, app: str):
    """@absent and @cond != v1 [and @cond != v2 ...]"""
    t = " ".join(test.split())
    # manual parse
    if " and " not in t.lower() or "!=" not in t:
        return []
    if " or " in t.lower() or "document(" in t or "count(" in t or "matches(" in t:
        return []
    parts = re.split(r"\s+and\s+", t, flags=re.I)
    if len(parts) < 2:
        return []
    # first must be bare @attr
    m0 = re.match(r"@([\w:]+)$", parts[0].strip())
    if not m0:
        return []
    absent = local(m0.group(1))
    cond_attr = None
    vals = []
    for p in parts[1:]:
        m = re.match(
            r"@([\w:]+)\s*!=\s*(?:'([^']*)'|\"([^\"]*)\"|(-?[\w.]+))$",
            p.strip(),
            re.I,
        )
        if not m:
            return []
        a = local(m.group(1))
        if cond_attr is None:
            cond_attr = a
        elif cond_attr != a:
            return []
        vals.append(m.group(2) if m.group(2) is not None else (m.group(3) if m.group(3) is not None else m.group(4)))
    if not vals or cond_attr is None:
        return []
    return [(ctx, absent, cond_attr, tuple(vals), app)]


def extract_mutual_exclusive(test: str, ctx: str, app: str):
    t = " ".join(test.split())
    if " or " not in t.lower():
        return []
    pairs = MUTUAL_PAIR.findall(t)
    if len(pairs) < 2:
        return []
    # strip all pairs and check only or/ws left
    cleaned = MUTUAL_PAIR.sub("", t)
    cleaned = re.sub(r"\bor\b|\(|\)|\s+", "", cleaned, flags=re.I)
    if cleaned:
        return []
    attrs = []
    seen = set()
    for a, b in pairs:
        for x in (local(a), local(b)):
            if x not in seen:
                seen.add(x)
                attrs.append(x)
    if len(attrs) < 2:
        return []
    return [(ctx, tuple(attrs), app)]


def extract_attr_and_eq(test: str, ctx: str, app: str):
    """@req and @flag = val → when flag==val, req required; when req present, flag must == val.
    Stored as ConditionalAttrRule-compatible: required when flag=val, plus fixed when req present.
    We emit as ConditionalAttrRule (req when flag=val) only — the dual is softer.
    """
    t = " ".join(test.split())
    m = ATTR_AND_EQ.match(t)
    if not m:
        return []
    req = local(m.group(1))
    flag = local(m.group(2))
    val = m.group(3) or m.group(4) or m.group(5)
    return [(ctx, req, flag, val, app)]



# (@a = x and @b = y) or @b != y — when b==y, a must equal x (manual parse)


def extract_bool_pair_impl(test: str, ctx: str, app: str):
    """(@a = x and @b = y) or @b != y — when b==y, a must equal x."""
    t = " ".join(test.split())
    if not (t.startswith("(@") and ") or @" in t):
        return []
    try:
        left, right = t.split(") or @", 1)
    except ValueError:
        return []
    left = left[1:]  # drop leading (
    # left: @a = x and @b = y
    lm = re.match(
        r"@([\w:]+)\s*=\s*(?:'([^']*)'|\"([^\"]*)\"|([\w.]+))\s+and\s+"
        r"@([\w:]+)\s*=\s*(?:'([^']*)'|\"([^\"]*)\"|([\w.]+))$",
        left.strip(),
        re.I,
    )
    rm = re.match(
        r"([\w:]+)\s*!=\s*(?:'([^']*)'|\"([^\"]*)\"|([\w.]+))$",
        right.strip(),
        re.I,
    )
    if not lm or not rm:
        return []
    a = local(lm.group(1))
    a_val = lm.group(2) or lm.group(3) or lm.group(4)
    b = local(lm.group(5))
    b_val = lm.group(6) or lm.group(7) or lm.group(8)
    b2 = local(rm.group(1))
    b_val2 = rm.group(2) or rm.group(3) or rm.group(4)
    if b != b2 or b_val != b_val2:
        return []
    return [(ctx, a, a_val, b, b_val, app)]


def extract_attr_and_enum(test: str, ctx: str, app: str):
    """@a and (@b=1 or @b=2) — when a present, b must be in values."""
    t = " ".join(test.split())
    m2 = re.match(r"^@([\w:]+)\s+and\s+\((.+)\)$", t, re.I)
    if not m2:
        return []
    req = local(m2.group(1))
    inner = m2.group(2)
    parts = re.split(r"\s+or\s+", inner, flags=re.I)
    attrs = set()
    vals = []
    for p in parts:
        mm = re.match(
            r"@([\w:]+)\s*=\s*(?:'([^']*)'|\"([^\"]*)\"|([\w.]+))$",
            p.strip(),
        )
        if not mm:
            return []
        attrs.add(local(mm.group(1)))
        vals.append(
            mm.group(2)
            if mm.group(2) is not None
            else (mm.group(3) if mm.group(3) is not None else mm.group(4))
        )
    if len(attrs) != 1:
        return []
    return [(ctx, req, list(attrs)[0], tuple(vals), app)]



# ((@a=v1 or @a=v2) and (@b=w1 or @b=w2)) or (@b!=w1 and @b!=w2)
# → when b in {w1,w2}, a must be in {v1,v2}
def extract_enum_when_flag_in(test: str, ctx: str, app: str):
    t = " ".join(test.split())
    # Form: ((@a=v1 or @a=v2) and @b = w) or @b != w
    m0 = re.match(
        r"^\(\((.+)\)\s+and\s+@([\w:.-]+)\s*=\s*(?:'([^']*)'|\"([^\"]*)\"|([\w.]+))\)\s+or\s+@\2\s*!=\s*(?:'([^']*)'|\"([^\"]*)\"|([\w.]+))$",
        t,
        re.I,
    )
    if m0:
        left_ors = m0.group(1)
        flag = local(m0.group(2))
        flag_val = m0.group(3) or m0.group(4) or m0.group(5)
        flag_val2 = m0.group(6) or m0.group(7) or m0.group(8)
        if flag_val != flag_val2:
            return []
        attrs, vals = set(), []
        for p in re.split(r"\s+or\s+", left_ors, flags=re.I):
            mm = re.match(
                r"@([\w:.-]+)\s*=\s*(?:'([^']*)'|\"([^\"]*)\"|([\w.]+))$",
                p.strip(),
            )
            if not mm:
                return []
            attrs.add(local(mm.group(1)))
            vals.append(mm.group(2) or mm.group(3) or mm.group(4))
        if len(attrs) != 1:
            return []
        return [(ctx, list(attrs)[0], tuple(vals), flag, (flag_val,), app)]
    # strip outer optional parens carefully
    m = re.match(
        r"^\(\((.+)\)\s+and\s+\((.+)\)\)\s+or\s+\((.+)\)$",
        t,
        re.I,
    )
    if not m:
        # sparkline form: (@a = 0 and (@b = x or @b = y)) or (@b != x and @b != y)
        m2 = re.match(
            r"^\(@([\w:.-]+)\s*=\s*(?:'([^']*)'|\"([^\"]*)\"|([\w.]+))\s+and\s+\((.+)\)\)\s+or\s+\((.+)\)$",
            t,
            re.I,
        )
        if not m2:
            return []
        other = local(m2.group(1))
        other_val = m2.group(2) or m2.group(3) or m2.group(4)
        flag_ors = m2.group(5)
        flag_nes = m2.group(6)
        # parse flag = vals from or chain
        flag_attrs = set()
        flag_vals = []
        for p in re.split(r"\s+or\s+", flag_ors, flags=re.I):
            mm = re.match(
                r"@([\w:.-]+)\s*=\s*(?:'([^']*)'|\"([^\"]*)\"|([\w.]+))$",
                p.strip(),
            )
            if not mm:
                return []
            flag_attrs.add(local(mm.group(1)))
            flag_vals.append(mm.group(2) or mm.group(3) or mm.group(4))
        if len(flag_attrs) != 1:
            return []
        # ne side should match same flag vals
        ne_attrs = set()
        ne_vals = []
        for p in re.split(r"\s+and\s+", flag_nes, flags=re.I):
            mm = re.match(
                r"@([\w:.-]+)\s*!=\s*(?:'([^']*)'|\"([^\"]*)\"|([\w.]+))$",
                p.strip(),
            )
            if not mm:
                return []
            ne_attrs.add(local(mm.group(1)))
            ne_vals.append(mm.group(2) or mm.group(3) or mm.group(4))
        if ne_attrs != flag_attrs or set(ne_vals) != set(flag_vals):
            return []
        # when flag in flag_vals, other must == other_val
        return [(ctx, other, (other_val,), list(flag_attrs)[0], tuple(flag_vals), app)]

    left_a = m.group(1)
    left_b = m.group(2)
    right = m.group(3)

    def parse_eq_ors(s):
        attrs, vals = set(), []
        for p in re.split(r"\s+or\s+", s, flags=re.I):
            mm = re.match(
                r"@([\w:.-]+)\s*=\s*(?:'([^']*)'|\"([^\"]*)\"|([\w.]+))$",
                p.strip(),
            )
            if not mm:
                return None
            attrs.add(local(mm.group(1)))
            vals.append(mm.group(2) or mm.group(3) or mm.group(4))
        if len(attrs) != 1:
            return None
        return list(attrs)[0], tuple(vals)

    def parse_ne_ands(s):
        attrs, vals = set(), []
        for p in re.split(r"\s+and\s+", s, flags=re.I):
            mm = re.match(
                r"@([\w:.-]+)\s*!=\s*(?:'([^']*)'|\"([^\"]*)\"|([\w.]+))$",
                p.strip(),
            )
            if not mm:
                return None
            attrs.add(local(mm.group(1)))
            vals.append(mm.group(2) or mm.group(3) or mm.group(4))
        if len(attrs) != 1:
            return None
        return list(attrs)[0], tuple(vals)

    pa = parse_eq_ors(left_a)
    pb = parse_eq_ors(left_b)
    pr = parse_ne_ands(right)
    if not pa or not pb or not pr:
        return []
    a_attr, a_vals = pa
    b_attr, b_vals = pb
    r_attr, r_vals = pr
    if b_attr != r_attr or set(b_vals) != set(r_vals):
        return []
    # when b in b_vals, a must be in a_vals
    return [(ctx, a_attr, a_vals, b_attr, b_vals, app)]


# Fixed value with hyphenated attribute names
FIXED_VALUE_HYPHEN = re.compile(
    r"^@([A-Za-z_][\w:.-]*)\s*=\s*(?:'([^']*)'|\"([^\"]*)\"|(-?[\w.-]+))$",
    re.I,
)


def extract_fixed_value_hyphen(test: str, ctx: str, app: str):
    t = test.strip()
    if " and " in t.lower() or " or " in t.lower():
        return []
    m = FIXED_VALUE_HYPHEN.match(t)
    if not m:
        return []
    val = m.group(2) if m.group(2) is not None else (m.group(3) if m.group(3) is not None else m.group(4))
    if val is None or val.lower() in ("true", "false"):
        return []
    return [(ctx, local(m.group(1)), val, app)]


# Sheet name / codeName matches patterns we implement specially
SHEET_NAME_PAT = '[^\\\'*\\[\\]/\\\\:?]{1}[^*\\[\\]/\\\\:?]*'
# also the escaped form in json


def extract_special_matches(test: str, ctx: str, app: str):
    t = test.strip()
    m = re.match(r'matches\(\s*@([\w:.-]+)\s*,\s*[\'"](.+)[\'"]\s*\)$', t, re.I)
    if not m:
        return []
    attr = local(m.group(1))
    pat = m.group(2)
    # sheet name forbidden-char pattern variants
    if ":*" in pat or "*\\[" in pat or "[^\\'" in pat or "[^'*\\[" in pat or "^*" in pat:
        if "codeName" in attr or "codeName" in test:
            return [(ctx, attr, "excel_codename", app)]
        return [(ctx, attr, "excel_sheet_name", app)]
    if r"\p{" in pat or r"\P{" in pat:
        return [(ctx, attr, "excel_codename", app)]
    return []


def rust_escape(s: str) -> str:
    return s.replace("\\", "\\\\").replace('"', '\\"')


def write_rules(data, out: Path) -> tuple[int, int]:
    rel_rules = []
    uniq_rules = []
    for d in data:
        test = d.get("Test", "")
        app = d.get("App", "All")
        ctx = local(d.get("Context", ""))
        m = REL_ID.search(test)
        if m:
            attr = local(m.group(1))
            mt = REL_TYPE.search(test)
            typ = mt.group(1) if mt else None
            rel_rules.append((ctx, attr, typ, app))
            continue
        u = extract_unique(test, app)
        if u:
            uniq_rules.append(u)

    def dedupe(items, keyfn):
        out_l, seen = [], set()
        for it in items:
            k = keyfn(it)
            if k not in seen:
                seen.add(k)
                out_l.append(it)
        return out_l

    rel_u = dedupe(rel_rules, lambda r: (r[0], r[1], r[2]))
    uniq_u = dedupe(uniq_rules, lambda r: (r[0], r[1], r[2]))

    lines = [
        "// @generated from Open-XML-SDK/data/schematrons.json — do not edit by hand",
        "// Relationship-existence and unique-attribute rules extracted from Schematron entries.",
        "// Regenerate: python3 scripts/generate_schematron_rules.py",
        "use super::semantic::{RelationshipExistRule, UniqueAttributeRule};",
        "",
        "/// All extractable relationship-existence rules from schematrons.json.",
        "pub fn schematron_relationship_rules() -> Vec<RelationshipExistRule> {",
        "    vec![",
    ]
    for el, attr, typ, app in rel_u:
        if typ:
            lines.append(
                f'        RelationshipExistRule::new("{el}", "{attr}", Some("{typ}")), // {app}'
            )
        else:
            lines.append(
                f'        RelationshipExistRule::new("{el}", "{attr}", None::<&str>), // {app}'
            )
    lines += [
        "    ]",
        "}",
        "",
        "/// All extractable unique-attribute rules from schematrons.json.",
        "pub fn schematron_unique_attribute_rules() -> Vec<UniqueAttributeRule> {",
        "    vec![",
    ]
    for el, attr, case, app in uniq_u:
        lines.append(
            f'        UniqueAttributeRule::new("{el}", "{attr}", {str(case).lower()}), // {app}'
        )
    lines += [
        "    ]",
        "}",
        "",
        f"/// Counts: {len(rel_u)} relationship rules, {len(uniq_u)} unique-attribute rules "
        f"(of {len(data)} source rules).",
        f"pub const SCHEMATRON_EXTRACTED_REL_COUNT: usize = {len(rel_u)};",
        f"pub const SCHEMATRON_EXTRACTED_UNIQUE_COUNT: usize = {len(uniq_u)};",
        f"pub const SCHEMATRON_TOTAL_SOURCE_RULES: usize = {len(data)};",
        "",
    ]
    out.write_text("\n".join(lines))
    return len(rel_u), len(uniq_u)


def write_constraints(data, out: Path):
    numeric = []
    lengths = []
    patterns = []
    enums = []
    ancestor_unique = []
    implications = []
    nonzero_guids = []
    attr_cmps = []
    fixed_bools = []
    cross_indexes = []
    cross_counts = []
    fixed_values = []
    fixed_nes = []
    multi_nes = []
    both_presents = []
    nan_infs = []
    required_attrs = []
    absent_when_nots = []
    mutual_exclusives = []
    bool_pair_impls = []
    attr_and_enums = []
    enum_when_flags = []
    special_matches = []
    for d in data:
        test = d.get("Test", "")
        app = d.get("App", "All")
        ctx = local(d.get("Context", ""))
        numeric.extend(extract_numeric(test, ctx, app))
        lengths.extend(extract_string_len(test, ctx, app))
        patterns.extend(extract_matches(test, ctx, app))
        enums.extend(extract_enum(test, ctx, app))
        au = extract_ancestor_unique(test, app)
        if au:
            ancestor_unique.append(au)
        implications.extend(extract_implication(test, ctx, app))
        nonzero_guids.extend(extract_nonzero_guid(test, ctx, app))
        attr_cmps.extend(extract_attr_cmp(test, ctx, app))
        fixed_bools.extend(extract_fixed_bool(test, ctx, app))
        cross_indexes.extend(extract_cross_index(test, ctx, app))
        cross_counts.extend(extract_cross_count(test, ctx, app))
        # fold hex/float ranges into numeric
        numeric.extend(extract_hex_range(test, ctx, app))
        numeric.extend(extract_float_f_range(test, ctx, app))
        fixed_values.extend(extract_fixed_value(test, ctx, app))
        fixed_nes.extend(extract_fixed_ne(test, ctx, app))
        multi_nes.extend(extract_multi_ne(test, ctx, app))
        both_presents.extend(extract_both_present(test, ctx, app))
        nan_infs.extend(extract_nan_inf(test, ctx, app))
        required_attrs.extend(extract_required_attr(test, ctx, app))
        absent_when_nots.extend(extract_absent_when_not(test, ctx, app))
        mutual_exclusives.extend(extract_mutual_exclusive(test, ctx, app))
        bool_pair_impls.extend(extract_bool_pair_impl(test, ctx, app))
        attr_and_enums.extend(extract_attr_and_enum(test, ctx, app))
        enum_when_flags.extend(extract_enum_when_flag_in(test, ctx, app))
        special_matches.extend(extract_special_matches(test, ctx, app))
        fixed_values.extend(extract_fixed_value_hyphen(test, ctx, app))
        # bare @a and @b=val also folds into conditional
        for item in extract_attr_and_eq(test, ctx, app):
            implications.append(item)

    def dedupe(items, keyfn):
        out_l, seen = [], set()
        for it in items:
            k = keyfn(it)
            if k not in seen:
                seen.add(k)
                out_l.append(it)
        return out_l

    numeric = dedupe(numeric, lambda r: (r[0], r[1], r[2], r[3]))
    lengths = dedupe(lengths, lambda r: (r[0], r[1], r[2], r[3]))
    patterns = dedupe(patterns, lambda r: (r[0], r[1], r[2]))
    enums = dedupe(enums, lambda r: (r[0], r[1], r[2]))
    ancestor_unique = dedupe(ancestor_unique, lambda r: (r[0], r[1], r[2], r[3]))
    implications = dedupe(implications, lambda r: (r[0], r[1], r[2], r[3]))
    nonzero_guids = dedupe(nonzero_guids, lambda r: (r[0], r[1]))
    attr_cmps = dedupe(attr_cmps, lambda r: (r[0], r[1], r[2], r[3]))
    fixed_bools = dedupe(fixed_bools, lambda r: (r[0], r[1], r[2]))
    cross_indexes = dedupe(cross_indexes, lambda r: (r[0], r[1], r[2], r[3], r[4]))
    cross_counts = dedupe(cross_counts, lambda r: (r[0], r[1], r[2], r[3], r[4]))
    fixed_values = dedupe(fixed_values, lambda r: (r[0], r[1], r[2]))
    fixed_nes = dedupe(fixed_nes, lambda r: (r[0], r[1], r[2]))
    multi_nes = dedupe(multi_nes, lambda r: (r[0], r[1], r[2]))
    both_presents = dedupe(both_presents, lambda r: (r[0], r[1], r[2]))
    nan_infs = dedupe(nan_infs, lambda r: (r[0], r[1]))
    required_attrs = dedupe(required_attrs, lambda r: (r[0], r[1]))
    absent_when_nots = dedupe(absent_when_nots, lambda r: (r[0], r[1], r[2], r[3]))
    mutual_exclusives = dedupe(mutual_exclusives, lambda r: (r[0], r[1]))
    bool_pair_impls = dedupe(bool_pair_impls, lambda r: (r[0], r[1], r[2], r[3], r[4]))
    attr_and_enums = dedupe(attr_and_enums, lambda r: (r[0], r[1], r[2], r[3]))
    enum_when_flags = dedupe(enum_when_flags, lambda r: (r[0], r[1], r[2], r[3], r[4]))
    special_matches = dedupe(special_matches, lambda r: (r[0], r[1], r[2]))

    lines = [
        "// @generated from Open-XML-SDK/data/schematrons.json — do not edit by hand",
        "// Numeric range, string-length, matches(), and enum constraints extractable without full XPath.",
        "// Regenerate: python3 scripts/generate_schematron_rules.py",
        "",
        "/// A numeric attribute range constraint: element/@attr ∈ [min, max].",
        "#[derive(Debug, Clone, Copy)]",
        "pub struct NumericRangeRule {",
        "    pub element: &'static str,",
        "    pub attribute: &'static str,",
        "    pub min: f64,",
        "    pub max: f64,",
        "}",
        "",
        "/// A string-length constraint on an attribute.",
        "#[derive(Debug, Clone, Copy)]",
        "pub struct StringLengthRule {",
        "    pub element: &'static str,",
        "    pub attribute: &'static str,",
        "    pub min: usize,",
        "    pub max: usize,",
        "}",
        "",
        "/// A regex-like pattern constraint (`matches(@attr, \"pattern\")`).",
        "/// Patterns are ECMAScript-ish Schematron strings; only simple subsets are enforced.",
        "#[derive(Debug, Clone, Copy)]",
        "pub struct PatternRule {",
        "    pub element: &'static str,",
        "    pub attribute: &'static str,",
        "    pub pattern: &'static str,",
        "}",
        "",
        "/// An enumeration constraint: attribute value must be one of the listed tokens.",
        "#[derive(Debug, Clone, Copy)]",
        "pub struct EnumRule {",
        "    pub element: &'static str,",
        "    pub attribute: &'static str,",
        "    pub values: &'static [&'static str],",
        "}",
        "",
        "/// Numeric range rules from schematrons.json.",
        "pub fn schematron_numeric_range_rules() -> Vec<NumericRangeRule> {",
        "    vec![",
    ]
    for el, attr, lo, hi, app in numeric:
        tag = "All"
        if lo == float("-inf"):
            tag = "max-only"
        elif hi == float("inf"):
            tag = "min-only"
        lines.append(
            f'        NumericRangeRule {{ element: "{el}", attribute: "{attr}", '
            f"min: {rust_f64(lo)}, max: {rust_f64(hi)} }}, // {app} {tag}"
        )
    lines += [
        "    ]",
        "}",
        "",
        "/// String-length rules from schematrons.json.",
        "pub fn schematron_string_length_rules() -> Vec<StringLengthRule> {",
        "    vec![",
    ]
    for el, attr, lo, hi, app in lengths:
        hi_s = rust_usize(hi if hi is not None else 2**64 - 1, is_max=hi is None)
        tag = "All"
        if hi is None:
            tag = "min-only"
        elif lo == 0:
            tag = "max-only"
        lines.append(
            f'        StringLengthRule {{ element: "{el}", attribute: "{attr}", '
            f"min: {lo}, max: {hi_s} }}, // {app} {tag}"
        )
    lines += [
        "    ]",
        "}",
        "",
        "/// Pattern (matches) rules from schematrons.json (no Unicode property classes).",
        "pub fn schematron_pattern_rules() -> Vec<PatternRule> {",
        "    vec![",
    ]
    for el, attr, pat, app in patterns:
        lines.append(
            f'        PatternRule {{ element: "{el}", attribute: "{attr}", '
            f'pattern: "{rust_escape(pat)}" }}, // {app}'
        )
    lines += [
        "    ]",
        "}",
        "",
        "/// Enumeration rules from schematrons.json (`@attr = a or @attr = b …`).",
        "pub fn schematron_enum_rules() -> Vec<EnumRule> {",
        "    vec![",
    ]
    for el, attr, vals, app in enums:
        vals_lit = ", ".join(f'"{rust_escape(v)}"' for v in vals)
        lines.append(
            f'        EnumRule {{ element: "{el}", attribute: "{attr}", '
            f"values: &[{vals_lit}] }}, // {app}"
        )
    lines += [
        "    ]",
        "}",
        "",
        "/// Ancestor-scoped uniqueness: within each ancestor, child @attr is unique.",
        "#[derive(Debug, Clone, Copy)]",
        "pub struct AncestorUniqueRule {",
        "    pub ancestor: &'static str,",
        "    pub element: &'static str,",
        "    pub attribute: &'static str,",
        "    pub case_insensitive: bool,",
        "}",
        "",
        "/// Conditional attribute: when `flag` equals `flag_value`, `required` must be present.",
        "#[derive(Debug, Clone, Copy)]",
        "pub struct ConditionalAttrRule {",
        "    pub element: &'static str,",
        "    pub required_attribute: &'static str,",
        "    pub flag_attribute: &'static str,",
        "    pub flag_value: &'static str,",
        "}",
        "",
        "/// Attribute must not be the nil UUID.",
        "#[derive(Debug, Clone, Copy)]",
        "pub struct NonZeroGuidRule {",
        "    pub element: &'static str,",
        "    pub attribute: &'static str,",
        "}",
        "",
        "/// Ancestor-scoped unique-attribute rules.",
        "pub fn schematron_ancestor_unique_rules() -> Vec<AncestorUniqueRule> {",
        "    vec![",
    ]
    for anc, el, attr, case, app in ancestor_unique:
        lines.append(
            f'        AncestorUniqueRule {{ ancestor: "{anc}", element: "{el}", '
            f'attribute: "{attr}", case_insensitive: {str(case).lower()} }}, // {app}'
        )
    lines += [
        "    ]",
        "}",
        "",
        "/// Conditional attribute presence rules.",
        "pub fn schematron_conditional_attr_rules() -> Vec<ConditionalAttrRule> {",
        "    vec![",
    ]
    for el, req, flag, val, app in implications:
        lines.append(
            f'        ConditionalAttrRule {{ element: "{el}", required_attribute: "{req}", '
            f'flag_attribute: "{flag}", flag_value: "{rust_escape(val)}" }}, // {app}'
        )
    lines += [
        "    ]",
        "}",
        "",
        "/// Non-zero GUID rules.",
        "pub fn schematron_nonzero_guid_rules() -> Vec<NonZeroGuidRule> {",
        "    vec![",
    ]
    for el, attr, app in nonzero_guids:
        lines.append(
            f'        NonZeroGuidRule {{ element: "{el}", attribute: "{attr}" }}, // {app}'
        )
    lines += [
        "    ]",
        "}",
        "",
        "/// Same-element attribute comparison: @left OP @right.",
        "#[derive(Debug, Clone, Copy)]",
        "pub struct AttrCompareRule {",
        "    pub element: &'static str,",
        "    pub left: &'static str,",
        "    pub op: &'static str,",
        "    pub right: &'static str,",
        "}",
        "",
        "/// Attribute must equal a fixed boolean.",
        "#[derive(Debug, Clone, Copy)]",
        "pub struct FixedBoolRule {",
        "    pub element: &'static str,",
        "    pub attribute: &'static str,",
        "    pub expected: bool,",
        "}",
        "",
        "/// Cross-part Index-of: context/@attr must exist among target part leaf/@path_attr.",
        "#[derive(Debug, Clone, Copy)]",
        "pub struct CrossPartIndexRule {",
        "    pub element: &'static str,",
        "    pub attribute: &'static str,",
        "    pub part_hint: &'static str,",
        "    pub target_element: &'static str,",
        "    pub target_attribute: &'static str,",
        "}",
        "",
        "/// Cross-part bound: @attr < count(target_element in part) + offset.",
        "#[derive(Debug, Clone, Copy)]",
        "pub struct CrossPartCountRule {",
        "    pub element: &'static str,",
        "    pub attribute: &'static str,",
        "    pub part_hint: &'static str,",
        "    pub target_element: &'static str,",
        "    pub offset: i64,",
        "}",
        "",
        "pub fn schematron_attr_compare_rules() -> Vec<AttrCompareRule> {",
        "    vec![",
    ]
    for el, left, op, right, app in attr_cmps:
        lines.append(
            f'        AttrCompareRule {{ element: "{el}", left: "{left}", op: "{op}", right: "{right}" }}, // {app}'
        )
    lines += [
        "    ]",
        "}",
        "",
        "pub fn schematron_fixed_bool_rules() -> Vec<FixedBoolRule> {",
        "    vec![",
    ]
    for el, attr, exp, app in fixed_bools:
        lines.append(
            f'        FixedBoolRule {{ element: "{el}", attribute: "{attr}", expected: {str(exp).lower()} }}, // {app}'
        )
    lines += [
        "    ]",
        "}",
        "",
        "pub fn schematron_cross_part_index_rules() -> Vec<CrossPartIndexRule> {",
        "    vec![",
    ]
    for el, attr, part, leaf, path_attr, app in cross_indexes:
        lines.append(
            f'        CrossPartIndexRule {{ element: "{el}", attribute: "{attr}", '
            f'part_hint: "{rust_escape(part)}", target_element: "{leaf}", '
            f'target_attribute: "{path_attr}" }}, // {app}'
        )
    lines += [
        "    ]",
        "}",
        "",
        "pub fn schematron_cross_part_count_rules() -> Vec<CrossPartCountRule> {",
        "    vec![",
    ]
    for el, attr, part, leaf, offset, app in cross_counts:
        lines.append(
            f'        CrossPartCountRule {{ element: "{el}", attribute: "{attr}", '
            f'part_hint: "{rust_escape(part)}", target_element: "{leaf}", '
            f"offset: {offset} }}, // {app}"
        )
    lines += [
        "    ]",
        "}",
        "",
        "/// Attribute must equal a fixed literal.",
        "#[derive(Debug, Clone, Copy)]",
        "pub struct FixedValueRule {",
        "    pub element: &'static str,",
        "    pub attribute: &'static str,",
        "    pub value: &'static str,",
        "}",
        "",
        "/// Attribute must not equal a fixed literal.",
        "#[derive(Debug, Clone, Copy)]",
        "pub struct FixedNeRule {",
        "    pub element: &'static str,",
        "    pub attribute: &'static str,",
        "    pub forbidden: &'static str,",
        "}",
        "",
        "/// Attribute must not be any of the listed values.",
        "#[derive(Debug, Clone, Copy)]",
        "pub struct MultiNeRule {",
        "    pub element: &'static str,",
        "    pub attribute: &'static str,",
        "    pub forbidden: &'static [&'static str],",
        "}",
        "",
        "/// Both attributes must be present together.",
        "#[derive(Debug, Clone, Copy)]",
        "pub struct BothPresentRule {",
        "    pub element: &'static str,",
        "    pub left: &'static str,",
        "    pub right: &'static str,",
        "}",
        "",
        "/// Attribute must not be NaN / INF / -INF.",
        "#[derive(Debug, Clone, Copy)]",
        "pub struct FiniteNumberRule {",
        "    pub element: &'static str,",
        "    pub attribute: &'static str,",
        "}",
        "",
        "pub fn schematron_fixed_value_rules() -> Vec<FixedValueRule> {",
        "    vec![",
    ]
    for el, attr, val, app in fixed_values:
        lines.append(
            f'        FixedValueRule {{ element: "{el}", attribute: "{attr}", value: "{rust_escape(val)}" }}, // {app}'
        )
    lines += [
        "    ]",
        "}",
        "",
        "pub fn schematron_fixed_ne_rules() -> Vec<FixedNeRule> {",
        "    vec![",
    ]
    for el, attr, val, app in fixed_nes:
        lines.append(
            f'        FixedNeRule {{ element: "{el}", attribute: "{attr}", forbidden: "{rust_escape(val)}" }}, // {app}'
        )
    lines += [
        "    ]",
        "}",
        "",
        "pub fn schematron_multi_ne_rules() -> Vec<MultiNeRule> {",
        "    vec![",
    ]
    for el, attr, vals, app in multi_nes:
        vals_lit = ", ".join(f'"{rust_escape(v)}"' for v in vals)
        lines.append(
            f'        MultiNeRule {{ element: "{el}", attribute: "{attr}", forbidden: &[{vals_lit}] }}, // {app}'
        )
    lines += [
        "    ]",
        "}",
        "",
        "pub fn schematron_both_present_rules() -> Vec<BothPresentRule> {",
        "    vec![",
    ]
    for el, left, right, app in both_presents:
        lines.append(
            f'        BothPresentRule {{ element: "{el}", left: "{left}", right: "{right}" }}, // {app}'
        )
    lines += [
        "    ]",
        "}",
        "",
        "pub fn schematron_finite_number_rules() -> Vec<FiniteNumberRule> {",
        "    vec![",
    ]
    for el, attr, app in nan_infs:
        lines.append(
            f'        FiniteNumberRule {{ element: "{el}", attribute: "{attr}" }}, // {app}'
        )
    lines += [
        "    ]",
        "}",
        "",
        "/// Attribute must be present.",
        "#[derive(Debug, Clone, Copy)]",
        "pub struct RequiredAttrRule {",
        "    pub element: &'static str,",
        "    pub attribute: &'static str,",
        "}",
        "",
        "pub fn schematron_required_attr_rules() -> Vec<RequiredAttrRule> {",
        "    vec![",
    ]
    for el, attr, app in required_attrs:
        lines.append(
            f'        RequiredAttrRule {{ element: "{el}", attribute: "{attr}" }}, // {app}'
        )
    lines += [
        "    ]",
        "}",
        "",
        "/// Attribute must be absent when condition attr is not in allowed values (1.15).",
        "#[derive(Debug, Clone, Copy)]",
        "pub struct AbsentWhenNotRule {",
        "    pub element: &'static str,",
        "    pub absent_attribute: &'static str,",
        "    pub condition_attribute: &'static str,",
        "    pub allowed_values: &'static [&'static str],",
        "}",
        "",
        "/// At most one of the listed attributes may be present (1.16).",
        "#[derive(Debug, Clone, Copy)]",
        "pub struct MutualExclusiveRule {",
        "    pub element: &'static str,",
        "    pub attributes: &'static [&'static str],",
        "}",
        "",
        "pub fn schematron_absent_when_not_rules() -> Vec<AbsentWhenNotRule> {",
        "    vec![",
    ]
    for el, absent, cond, vals, app in absent_when_nots:
        vals_lit = ", ".join(f'"{rust_escape(v)}"' for v in vals)
        lines.append(
            f'        AbsentWhenNotRule {{ element: "{el}", absent_attribute: "{absent}", '
            f'condition_attribute: "{cond}", allowed_values: &[{vals_lit}] }}, // {app}'
        )
    lines += [
        "    ]",
        "}",
        "",
        "pub fn schematron_mutual_exclusive_rules() -> Vec<MutualExclusiveRule> {",
        "    vec![",
    ]
    for el, attrs, app in mutual_exclusives:
        attrs_lit = ", ".join(f'"{a}"' for a in attrs)
        lines.append(
            f'        MutualExclusiveRule {{ element: "{el}", attributes: &[{attrs_lit}] }}, // {app}'
        )
    lines += [
        "    ]",
        "}",
        "",
        "/// When flag==flag_val, other must equal other_val.",
        "#[derive(Debug, Clone, Copy)]",
        "pub struct BoolPairImplRule {",
        "    pub element: &'static str,",
        "    pub other_attribute: &'static str,",
        "    pub other_value: &'static str,",
        "    pub flag_attribute: &'static str,",
        "    pub flag_value: &'static str,",
        "}",
        "",
        "/// When required attr present, flag attr must be one of values.",
        "#[derive(Debug, Clone, Copy)]",
        "pub struct AttrAndEnumRule {",
        "    pub element: &'static str,",
        "    pub required_attribute: &'static str,",
        "    pub flag_attribute: &'static str,",
        "    pub flag_values: &'static [&'static str],",
        "}",
        "",
        "pub fn schematron_bool_pair_impl_rules() -> Vec<BoolPairImplRule> {",
        "    vec![",
    ]
    for el, other, other_val, flag, flag_val, app in bool_pair_impls:
        lines.append(
            f'        BoolPairImplRule {{ element: "{el}", other_attribute: "{other}", '
            f'other_value: "{rust_escape(other_val)}", flag_attribute: "{flag}", '
            f'flag_value: "{rust_escape(flag_val)}" }}, // {app}'
        )
    lines += [
        "    ]",
        "}",
        "",
        "pub fn schematron_attr_and_enum_rules() -> Vec<AttrAndEnumRule> {",
        "    vec![",
    ]
    for el, req, flag, vals, app in attr_and_enums:
        vals_lit = ", ".join(f'"{rust_escape(v)}"' for v in vals)
        lines.append(
            f'        AttrAndEnumRule {{ element: "{el}", required_attribute: "{req}", '
            f'flag_attribute: "{flag}", flag_values: &[{vals_lit}] }}, // {app}'
        )
    lines += [
        "    ]",
        "}",
        "",
        "/// When flag is one of flag_values, other must be one of other_values.",
        "#[derive(Debug, Clone, Copy)]",
        "pub struct EnumWhenFlagRule {",
        "    pub element: &'static str,",
        "    pub other_attribute: &'static str,",
        "    pub other_values: &'static [&'static str],",
        "    pub flag_attribute: &'static str,",
        "    pub flag_values: &'static [&'static str],",
        "}",
        "",
        "/// Special matches patterns (excel sheet name / codeName).",
        "#[derive(Debug, Clone, Copy)]",
        "pub struct SpecialPatternRule {",
        "    pub element: &'static str,",
        "    pub attribute: &'static str,",
        "    pub kind: &'static str,",
        "}",
        "",
        "pub fn schematron_enum_when_flag_rules() -> Vec<EnumWhenFlagRule> {",
        "    vec![",
    ]
    for el, other, other_vals, flag, flag_vals, app in enum_when_flags:
        ov = ", ".join(f'"{rust_escape(v)}"' for v in other_vals)
        fv = ", ".join(f'"{rust_escape(v)}"' for v in flag_vals)
        lines.append(
            f'        EnumWhenFlagRule {{ element: "{el}", other_attribute: "{other}", '
            f'other_values: &[{ov}], flag_attribute: "{flag}", flag_values: &[{fv}] }}, // {app}'
        )
    lines += [
        "    ]",
        "}",
        "",
        "pub fn schematron_special_pattern_rules() -> Vec<SpecialPatternRule> {",
        "    vec![",
    ]
    for el, attr, kind, app in special_matches:
        lines.append(
            f'        SpecialPatternRule {{ element: "{el}", attribute: "{attr}", kind: "{kind}" }}, // {app}'
        )
    lines += [
        "    ]",
        "}",
        "",
        f"pub const SCHEMATRON_NUMERIC_RANGE_COUNT: usize = {len(numeric)};",



        f"pub const SCHEMATRON_STRING_LENGTH_COUNT: usize = {len(lengths)};",
        f"pub const SCHEMATRON_PATTERN_COUNT: usize = {len(patterns)};",
        f"pub const SCHEMATRON_ENUM_COUNT: usize = {len(enums)};",
        f"pub const SCHEMATRON_ANCESTOR_UNIQUE_COUNT: usize = {len(ancestor_unique)};",
        f"pub const SCHEMATRON_CONDITIONAL_ATTR_COUNT: usize = {len(implications)};",
        f"pub const SCHEMATRON_NONZERO_GUID_COUNT: usize = {len(nonzero_guids)};",
        f"pub const SCHEMATRON_ATTR_COMPARE_COUNT: usize = {len(attr_cmps)};",
        f"pub const SCHEMATRON_FIXED_BOOL_COUNT: usize = {len(fixed_bools)};",
        f"pub const SCHEMATRON_CROSS_PART_INDEX_COUNT: usize = {len(cross_indexes)};",
        f"pub const SCHEMATRON_CROSS_PART_COUNT_COUNT: usize = {len(cross_counts)};",
        f"pub const SCHEMATRON_FIXED_VALUE_COUNT: usize = {len(fixed_values)};",
        f"pub const SCHEMATRON_FIXED_NE_COUNT: usize = {len(fixed_nes)};",
        f"pub const SCHEMATRON_MULTI_NE_COUNT: usize = {len(multi_nes)};",
        f"pub const SCHEMATRON_BOTH_PRESENT_COUNT: usize = {len(both_presents)};",
        f"pub const SCHEMATRON_FINITE_NUMBER_COUNT: usize = {len(nan_infs)};",
        f"pub const SCHEMATRON_REQUIRED_ATTR_COUNT: usize = {len(required_attrs)};",
        f"pub const SCHEMATRON_ABSENT_WHEN_NOT_COUNT: usize = {len(absent_when_nots)};",
        f"pub const SCHEMATRON_MUTUAL_EXCLUSIVE_COUNT: usize = {len(mutual_exclusives)};",
        f"pub const SCHEMATRON_BOOL_PAIR_IMPL_COUNT: usize = {len(bool_pair_impls)};",
        f"pub const SCHEMATRON_ATTR_AND_ENUM_COUNT: usize = {len(attr_and_enums)};",
        f"pub const SCHEMATRON_ENUM_WHEN_FLAG_COUNT: usize = {len(enum_when_flags)};",
        f"pub const SCHEMATRON_SPECIAL_PATTERN_COUNT: usize = {len(special_matches)};",
        "",
    ]
    out.write_text("\n".join(lines))
    return (
        len(numeric),
        len(lengths),
        len(patterns),
        len(enums),
        len(ancestor_unique),
        len(implications),
        len(nonzero_guids),
        len(attr_cmps),
        len(fixed_bools),
        len(cross_indexes),
        len(cross_counts),
        len(fixed_values),
        len(fixed_nes),
        len(multi_nes),
        len(both_presents),
        len(nan_infs),
        len(required_attrs),
        len(absent_when_nots),
        len(mutual_exclusives),
        len(bool_pair_impls),
        len(attr_and_enums),
        len(enum_when_flags),
        len(special_matches),
    )


def main() -> None:
    ap = argparse.ArgumentParser()
    ap.add_argument(
        "--schematrons",
        type=Path,
        default=Path("/opt/wp/Open-XML-SDK/data/schematrons.json"),
    )
    ap.add_argument(
        "--out",
        type=Path,
        default=Path(__file__).resolve().parents[1]
        / "src/validation/schematron_rules.rs",
    )
    ap.add_argument(
        "--constraints-out",
        type=Path,
        default=Path(__file__).resolve().parents[1]
        / "src/validation/schematron_constraints.rs",
    )
    args = ap.parse_args()
    data = json.loads(args.schematrons.read_text())

    n_rel, n_uniq = write_rules(data, args.out)
    counts = write_constraints(data, args.constraints_out)
    n_num, n_len, n_pat, n_enum, n_au, n_impl, n_guid = counts[:7]
    n_cmp, n_bool, n_idx, n_cnt = counts[7:11]
    n_fv, n_fne, n_mne, n_bp, n_fin, n_req = counts[11:17]
    n_awn, n_mx, n_bpi, n_aae = counts[17:21]
    n_ewf, n_sp = (counts[21], counts[22]) if len(counts) > 22 else (0, 0)
    print(
        f"wrote {args.out} ({n_rel} rel, {n_uniq} unique) and "
        f"{args.constraints_out} ({n_num} numeric, {n_len} length, {n_pat} pattern, "
        f"{n_enum} enum, {n_au} ancestor-unique, {n_impl} conditional, {n_guid} guid, "
        f"{n_cmp} attr-cmp, {n_bool} fixed-bool, {n_idx} cross-index, {n_cnt} cross-count, "
        f"{n_fv} fixed-val, {n_fne} fixed-ne, {n_mne} multi-ne, {n_bp} both-present, {n_fin} finite, {n_req} required-attr, {n_awn} absent-when-not, {n_mx} mutual-excl, {n_bpi} bool-pair, {n_aae} attr-enum, {n_ewf} enum-when-flag, {n_sp} special-pat) "
        f"of {len(data)} source"
    )


if __name__ == "__main__":
    main()
