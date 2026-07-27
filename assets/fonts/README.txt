Bundled fonts for SVG→PPTX text measurement / outlining / ODTTF embedding.

Latin (required for cross-platform CI — Windows/macOS runners lack Liberation):
- LiberationSans-*.ttf  — Arial/Helvetica metrics stand-in (SIL OFL)
- LiberationSerif-*.ttf — Times New Roman metrics stand-in (SIL OFL)
- DejaVuSans-*.ttf      — monospace / generic sans fallback (Bitstream Vera)

CJK (ODTTF-safe TrueType for MS PowerPoint):
- NotoSansSC-*.ttf: static instances of Noto Sans SC (Source Han Sans SC).
  TrueType, fsType=0 (installable embedding). Prefer these over system
  Noto CJK TTC/OTTO extracts for Microsoft PowerPoint compatibility.
