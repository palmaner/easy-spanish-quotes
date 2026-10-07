"""Report actual unsigned artifact size, with clearly labelled extrapolation."""
import io
import json
import hashlib
import zipfile
from pathlib import Path

root=Path(__file__).resolve().parents[1]
data=(root/'build/layout/esq-layout.dll').read_bytes()
buffer=io.BytesIO()
with zipfile.ZipFile(buffer,'w',compression=zipfile.ZIP_DEFLATED,compresslevel=9) as archive:
    archive.writestr('esq-layout.dll',data)
print(json.dumps({
    'measurement':'one unsigned x64 preset',
    'sha256':hashlib.sha256(data).hexdigest(),
    'dll_bytes':len(data),
    'zip_bytes':len(buffer.getvalue()),
    '650_identical_size_unsigned_projection_bytes':650*len(data),
    'projection_is_not_a_650_variant_measurement':True,
    'signed_size_and_signing_time':'unmeasured; no signing certificate configured',
    'decision':'keep one preset until native lifecycle gate; signing/test complexity remains unmeasured'
},indent=2))
