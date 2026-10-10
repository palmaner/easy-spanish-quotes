# Catalog decision — preliminary measurement

Measured locally on 2026-10-08: one unsigned x64 preset DLL is **10,240 bytes**;
a ZIP containing it is **3,662 bytes**. The artifact includes the original key
names captured through GetKeyNameText and has no runtime imports.
Run `python tools/measure-catalog.py` to reproduce the size report.

650 identical-size unsigned artifacts would total 6,656,000 bytes. This is an
extrapolation, not a measured 650-variant catalog. Signing size/time, actual
catalog compression, other architectures and full matrix testing are unmeasured.

Unsigned storage alone appears modest. It does not establish that 650 separately
signed and validated variants are a worthwhile first-release commitment.
Keep the single Z/X preset until native lifecycle acceptance; then measure a
small preset set against all ordered pairs before expanding customization.
No signing certificate is configured and no signed-size estimate is presented.

After the E1 Pause flag correction on 2026-10-10, the DLL remains 10,240 bytes;
its ZIP is 3,658 bytes. Payload SHA-256:
`80137fab8f19a63cc9fd97c51f7cb915ed6b3493247203986436a7b97c4fde77`.
Earlier measured artifacts are retained; they are not the corrected candidate.
