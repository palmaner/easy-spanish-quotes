"""Verify every ZIP entry against the distribution hash manifest."""
import argparse
import hashlib
import json
import zipfile
from pathlib import PurePosixPath

parser=argparse.ArgumentParser()
parser.add_argument('archive')
args=parser.parse_args()
with zipfile.ZipFile(args.archive) as archive:
    names=archive.namelist()
    if len(names)!=len(set(names)): raise ValueError('Duplicate archive entry')
    for name in names:
        path=PurePosixPath(name)
        if path.is_absolute() or '..' in path.parts or '\\' in name:
            raise ValueError('Unsafe or non-portable archive path')
    hashes=json.loads(archive.read('SHA256.json'))
    if set(names)!=set(hashes)|{'SHA256.json'}:
        raise ValueError('Unaccounted archive contents')
    for name,expected in hashes.items():
        if hashlib.sha256(archive.read(name)).hexdigest()!=expected:
            raise ValueError('Hash mismatch: '+name)
    if 'licenses/toolchain-notices/rust/COPYRIGHT-library.html' not in names:
        raise ValueError('Missing Rust runtime notices')
    print(f'Verified {len(hashes)} bundled files and hashes')
