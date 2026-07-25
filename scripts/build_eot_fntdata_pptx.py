#!/usr/bin/env python3
"""Build PPTX with MS-style EOT .fntdata font embedding (like with-font.pptx)."""
from __future__ import annotations

import io
import re
import struct
import zipfile
from pathlib import Path

from fontTools import subset
from fontTools.ttLib import TTFont
from lxml import etree

NS_P = "http://schemas.openxmlformats.org/presentationml/2006/main"
NS_R = "http://schemas.openxmlformats.org/officeDocument/2006/relationships"
NS_CT = "http://schemas.openxmlformats.org/package/2006/content-types"
NS_PR = "http://schemas.openxmlformats.org/package/2006/relationships"

ROOT = Path("/opt/wp/openxml")
BASE = ROOT / "slide-1-win-sysfonts.pptx"


def utf16_z(s: str) -> bytes:
    return s.encode("utf-16-le") + b"\x00\x00"


def read_name(tt: TTFont, nid: int) -> str:
    for rec in tt["name"].names:
        if rec.nameID == nid and rec.platformID == 3:
            try:
                return rec.toUnicode()
            except Exception:
                pass
    for rec in tt["name"].names:
        if rec.nameID == nid:
            try:
                return rec.toUnicode()
            except Exception:
                pass
    return ""


def panose_bytes(os2) -> bytes:
    p = os2.panose
    return bytes(
        [
            p.bFamilyType,
            p.bSerifStyle,
            p.bWeight,
            p.bProportion,
            p.bContrast,
            p.bStrokeVariation,
            p.bArmStyle,
            p.bLetterForm,
            p.bMidline,
            p.bXHeight,
        ]
    )


def make_eot(ttf_bytes: bytes, charset: int) -> tuple[bytes, str, bytes, int]:
    """Build EOT 0x00020002 with uncompressed FontData (no MicroType flag)."""
    tt = TTFont(io.BytesIO(ttf_bytes))
    os2 = tt["OS/2"]
    panose = panose_bytes(os2)
    italic = 1 if (tt["head"].macStyle & 2) else 0
    weight = int(os2.usWeightClass)
    fs_type = int(os2.fsType) & 0xFFFF
    ur1 = getattr(os2, "ulUnicodeRange1", 0) & 0xFFFFFFFF
    ur2 = getattr(os2, "ulUnicodeRange2", 0) & 0xFFFFFFFF
    ur3 = getattr(os2, "ulUnicodeRange3", 0) & 0xFFFFFFFF
    ur4 = getattr(os2, "ulUnicodeRange4", 0) & 0xFFFFFFFF
    cpr1 = getattr(os2, "ulCodePageRange1", 0) & 0xFFFFFFFF
    cpr2 = getattr(os2, "ulCodePageRange2", 0) & 0xFFFFFFFF
    checksum = tt["head"].checkSumAdjustment & 0xFFFFFFFF

    family = read_name(tt, 1) or "Font"
    style = read_name(tt, 2) or "Regular"
    version = read_name(tt, 5) or "Version 1.0"
    full = read_name(tt, 4) or family

    fam_b = utf16_z(family)
    sty_b = utf16_z(style)
    ver_b = utf16_z(version)
    full_b = utf16_z(full)

    flags = 0  # uncompressed
    version_eot = 0x00020002
    parts: list[bytes] = []
    parts.append(struct.pack("<I", 0))  # EotSize placeholder
    parts.append(struct.pack("<I", len(ttf_bytes)))  # FontDataSize
    parts.append(struct.pack("<I", version_eot))
    parts.append(struct.pack("<I", flags))
    parts.append(panose)
    parts.append(struct.pack("<BB", charset & 0xFF, italic & 0xFF))
    parts.append(struct.pack("<I", weight))
    parts.append(struct.pack("<HH", fs_type, 0x504C))
    parts.append(struct.pack("<4I", ur1, ur2, ur3, ur4))
    parts.append(struct.pack("<2I", cpr1, cpr2))
    parts.append(struct.pack("<I", checksum))
    parts.append(struct.pack("<4I", 0, 0, 0, 0))

    def name_block(b: bytes) -> bytes:
        return struct.pack("<HH", 0, len(b)) + b

    parts.append(name_block(fam_b))
    parts.append(name_block(sty_b))
    parts.append(name_block(ver_b))
    parts.append(name_block(full_b))
    parts.append(struct.pack("<H", 0))  # RootStringSize
    # EOT 2.2 extras
    parts.append(struct.pack("<II", 0, 0))  # RootStringCheckSum, EUDCCodePage
    parts.append(struct.pack("<HH", 0, 0))  # Padding5, SignatureSize
    parts.append(struct.pack("<II", 0, 0))  # EUDCFlags, EUDCFontSize

    header = b"".join(parts)
    eot = bytearray(header + ttf_bytes)
    struct.pack_into("<I", eot, 0, len(eot))
    return bytes(eot), family, panose, weight


def subset_ttf(src: str, text_chars: str) -> bytes:
    unicodes = sorted({ord(c) for c in text_chars} | set(range(0x20, 0x7F)))
    options = subset.Options()
    options.layout_features = "*"
    options.name_IDs = "*"
    options.name_legacy = True
    options.name_languages = "*"
    options.notdef_outline = True
    options.recalc_bounds = True
    font = subset.load_font(src, options)
    s = subset.Subsetter(options=options)
    s.populate(unicodes=unicodes)
    s.subset(font)
    font["OS/2"].fsType = 0
    bio = io.BytesIO()
    subset.save_font(font, bio, options)
    return bio.getvalue()


def build_fntdata_pkg(out: Path, fonts: list[dict], slide_xml: str) -> None:
    prepared = []
    for i, f in enumerate(fonts):
        eot, fam, panose, weight = make_eot(f["ttf"], f.get("charset", 0))
        name = f"font{i + 1}"
        prepared.append(
            {
                **f,
                "eot": eot,
                "name": name,
                "rid": f"rIdF{i + 1}",
                "panose_bytes": panose,
                "family_from_font": fam,
                "weight": weight,
            }
        )
        print(
            out.name,
            name,
            "typeface=",
            f["typeface"],
            "eot=",
            len(eot),
            "family=",
            fam,
            "weight=",
            weight,
        )

    with zipfile.ZipFile(BASE) as zin, zipfile.ZipFile(out, "w") as zout:
        for item in zin.infolist():
            data = zin.read(item.filename)
            if item.filename == "ppt/slides/slide1.xml":
                data = slide_xml.encode("utf-8")
            elif item.filename == "ppt/presentation.xml":
                root = etree.fromstring(data)
                for lst in root.xpath("//p:embeddedFontLst", namespaces={"p": NS_P}):
                    lst.getparent().remove(lst)
                root.set("embedTrueTypeFonts", "1")
                if "saveSubsetFonts" in root.attrib:
                    del root.attrib["saveSubsetFonts"]
                lst = etree.Element("{%s}embeddedFontLst" % NS_P)
                by: dict[str, list] = {}
                for p in prepared:
                    by.setdefault(p["typeface"], []).append(p)
                for tf, items in by.items():
                    entry = etree.SubElement(lst, "{%s}embeddedFont" % NS_P)
                    font_el = etree.SubElement(entry, "{%s}font" % NS_P)
                    font_el.set("typeface", tf)
                    cs = items[0].get("charset", 0)
                    if cs >= 128:
                        cs = cs - 256
                    font_el.set("charset", str(cs))
                    font_el.set("pitchFamily", str(items[0].get("pitchFamily", 2)))
                    font_el.set("panose", items[0]["panose_bytes"].hex())
                    for it in items:
                        el = etree.SubElement(entry, "{%s}%s" % (NS_P, it["slot"]))
                        el.set("{%s}id" % NS_R, it["rid"])
                kids = list(root)
                ins = len(kids)
                for i, c in enumerate(kids):
                    if c.tag.endswith("defaultTextStyle"):
                        ins = i
                        break
                root.insert(ins, lst)
                data = etree.tostring(
                    root, xml_declaration=True, encoding="UTF-8", standalone=True
                )
            elif item.filename == "ppt/_rels/presentation.xml.rels":
                root = etree.fromstring(data)
                for rel in list(root):
                    if rel.get("Type", "").endswith("/font"):
                        root.remove(rel)
                for p in prepared:
                    rel = etree.SubElement(root, "{%s}Relationship" % NS_PR)
                    rel.set("Id", p["rid"])
                    rel.set(
                        "Type",
                        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/font",
                    )
                    rel.set("Target", f"fonts/{p['name']}.fntdata")
                data = etree.tostring(
                    root, xml_declaration=True, encoding="UTF-8", standalone=True
                )
            elif item.filename == "[Content_Types].xml":
                root = etree.fromstring(data)
                for ov in list(root):
                    pn = ov.get("PartName", "") or ""
                    ext = ov.get("Extension", "") or ""
                    if "odttf" in pn or ext in ("odttf", "fntdata"):
                        root.remove(ov)
                default = etree.Element("{%s}Default" % NS_CT)
                default.set("Extension", "fntdata")
                default.set("ContentType", "application/x-fontdata")
                root.insert(0, default)
                data = etree.tostring(
                    root, xml_declaration=True, encoding="UTF-8", standalone=True
                )
            zi = zipfile.ZipInfo(item.filename)
            zi.compress_type = zipfile.ZIP_DEFLATED
            zout.writestr(zi, data)
        for p in prepared:
            zi = zipfile.ZipInfo(f"ppt/fonts/{p['name']}.fntdata")
            zi.compress_type = zipfile.ZIP_STORED
            zout.writestr(zi, p["eot"])
    print("wrote", out, out.stat().st_size)


def main() -> None:
    with zipfile.ZipFile(BASE) as z:
        slide = z.read("ppt/slides/slide1.xml").decode()
    runs = re.findall(r"<a:t>(.*?)</a:t>", slide)
    chars = "".join(runs)

    lib_reg = Path(
        "/usr/share/fonts/truetype/liberation/LiberationSans-Regular.ttf"
    ).read_bytes()
    lib_bol = Path(
        "/usr/share/fonts/truetype/liberation/LiberationSans-Bold.ttf"
    ).read_bytes()
    noto_reg = subset_ttf(str(ROOT / "assets/fonts/NotoSansSC-Regular.ttf"), chars)
    noto_bol = subset_ttf(str(ROOT / "assets/fonts/NotoSansSC-Bold.ttf"), chars)
    print("subset noto", len(noto_reg), len(noto_bol))

    slide_noto = slide.replace("Microsoft YaHei", "Noto Sans SC")
    build_fntdata_pkg(
        ROOT / "slide-1-eot-noto.pptx",
        [
            {
                "typeface": "Noto Sans SC",
                "slot": "regular",
                "ttf": noto_reg,
                "charset": 134,
                "pitchFamily": 2,
            },
            {
                "typeface": "Noto Sans SC",
                "slot": "bold",
                "ttf": noto_bol,
                "charset": 134,
                "pitchFamily": 2,
            },
        ],
        slide_noto,
    )

    slide_lib = slide.replace('typeface="Arial"', 'typeface="Liberation Sans"')
    build_fntdata_pkg(
        ROOT / "slide-1-eot-liberation.pptx",
        [
            {
                "typeface": "Liberation Sans",
                "slot": "regular",
                "ttf": lib_reg,
                "charset": 0,
                "pitchFamily": 34,
            },
            {
                "typeface": "Liberation Sans",
                "slot": "bold",
                "ttf": lib_bol,
                "charset": 0,
                "pitchFamily": 34,
            },
        ],
        slide_lib,
    )

    # Reference DengXian blobs into our openable slide
    with zipfile.ZipFile(ROOT / "with-font.pptx") as wz:
        f1 = wz.read("ppt/fonts/font1.fntdata")
        f2 = wz.read("ppt/fonts/font2.fntdata")
        f3 = wz.read("ppt/fonts/font3.fntdata")

    out = ROOT / "slide-1-eot-refdengxian.pptx"
    with zipfile.ZipFile(BASE) as zin, zipfile.ZipFile(out, "w") as zout:
        for item in zin.infolist():
            data = zin.read(item.filename)
            if item.filename == "ppt/presentation.xml":
                root = etree.fromstring(data)
                for lst in root.xpath("//p:embeddedFontLst", namespaces={"p": NS_P}):
                    lst.getparent().remove(lst)
                root.set("embedTrueTypeFonts", "1")
                if "saveSubsetFonts" in root.attrib:
                    del root.attrib["saveSubsetFonts"]
                lst = etree.Element("{%s}embeddedFontLst" % NS_P)
                for ent in [
                    (
                        {
                            "typeface": "等线",
                            "panose": "02010600030101010101",
                            "pitchFamily": "2",
                            "charset": "-122",
                        },
                        [("regular", "rIdF1"), ("bold", "rIdF2")],
                    ),
                    (
                        {
                            "typeface": "等线 Light",
                            "panose": "02010600030101010101",
                            "pitchFamily": "2",
                            "charset": "-122",
                        },
                        [("regular", "rIdF3")],
                    ),
                ]:
                    font_attrs, faces = ent
                    entry = etree.SubElement(lst, "{%s}embeddedFont" % NS_P)
                    font_el = etree.SubElement(entry, "{%s}font" % NS_P)
                    for k, v in font_attrs.items():
                        font_el.set(k, v)
                    for slot, rid in faces:
                        el = etree.SubElement(entry, "{%s}%s" % (NS_P, slot))
                        el.set("{%s}id" % NS_R, rid)
                kids = list(root)
                ins = len(kids)
                for i, c in enumerate(kids):
                    if c.tag.endswith("defaultTextStyle"):
                        ins = i
                        break
                root.insert(ins, lst)
                data = etree.tostring(
                    root, xml_declaration=True, encoding="UTF-8", standalone=True
                )
            elif item.filename == "ppt/_rels/presentation.xml.rels":
                root = etree.fromstring(data)
                for rel in list(root):
                    if rel.get("Type", "").endswith("/font"):
                        root.remove(rel)
                for name, rid in [("font1", "rIdF1"), ("font2", "rIdF2"), ("font3", "rIdF3")]:
                    rel = etree.SubElement(root, "{%s}Relationship" % NS_PR)
                    rel.set("Id", rid)
                    rel.set(
                        "Type",
                        "http://schemas.openxmlformats.org/officeDocument/2006/relationships/font",
                    )
                    rel.set("Target", f"fonts/{name}.fntdata")
                data = etree.tostring(
                    root, xml_declaration=True, encoding="UTF-8", standalone=True
                )
            elif item.filename == "[Content_Types].xml":
                root = etree.fromstring(data)
                for ov in list(root):
                    pn = ov.get("PartName", "") or ""
                    ext = ov.get("Extension", "") or ""
                    if "odttf" in pn or ext in ("odttf", "fntdata"):
                        root.remove(ov)
                d = etree.Element("{%s}Default" % NS_CT)
                d.set("Extension", "fntdata")
                d.set("ContentType", "application/x-fontdata")
                root.insert(0, d)
                data = etree.tostring(
                    root, xml_declaration=True, encoding="UTF-8", standalone=True
                )
            zi = zipfile.ZipInfo(item.filename)
            zi.compress_type = zipfile.ZIP_DEFLATED
            zout.writestr(zi, data)
        for name, blob in [("font1", f1), ("font2", f2), ("font3", f3)]:
            zi = zipfile.ZipInfo(f"ppt/fonts/{name}.fntdata")
            zi.compress_type = zipfile.ZIP_STORED
            zout.writestr(zi, blob)
    print("wrote", out, out.stat().st_size)


if __name__ == "__main__":
    main()
