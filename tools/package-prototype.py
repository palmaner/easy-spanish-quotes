"""Package built experimental binaries, actual dependency licenses and hashes.

Never signs, publishes or installs. Fails if a dependency license is missing.
"""
import argparse
import hashlib
import json
import shutil
import subprocess
import zipfile
from pathlib import Path

ROOT=Path(__file__).resolve().parents[1]


def package(cargo):
    destination=ROOT/'dist/easy-spanish-quotes-0.1.0-experimental-x64'
    # Never silently replace a previous bundle or user file.
    destination.mkdir(parents=True,exist_ok=False)
    for binary in ('easy-spanish-quotes.exe','esq-gui.exe'):
        shutil.copyfile(ROOT/'target/release'/binary,destination/binary)
    for file in ('README.md','IMPLEMENTATION.md','LICENSE','THIRD_PARTY_NOTICES.md'):
        shutil.copyfile(ROOT/file,destination/file)
    shutil.copytree(ROOT/'docs',destination/'docs')
    shutil.copytree(ROOT/'third-party',destination/'licenses'/'toolchain-notices')
    metadata=json.loads(subprocess.check_output([cargo,'metadata','--locked','--offline',
        '--format-version','1','--filter-platform','x86_64-pc-windows-gnu'],cwd=ROOT,text=True))
    sbom=[]
    for dependency in metadata['packages']:
        if dependency['source'] is None: continue
        source=Path(dependency['manifest_path']).parent
        licenses=[file for pattern in ('LICENSE*','COPYING*','COPYRIGHT*')
                  for file in source.glob(pattern) if file.is_file()]
        if not licenses:
            raise RuntimeError('Missing license file: '+dependency['name'])
        folder=destination/'licenses'/f"{dependency['name']}-{dependency['version']}"
        folder.mkdir(parents=True)
        for file in licenses: shutil.copyfile(file,folder/file.name)
        sbom.append({key:dependency[key] for key in ('name','version','license','source')})
    (destination/'dependencies.json').write_text(json.dumps(sbom,indent=2)+'\n',encoding='utf-8')
    (destination/'EXPERIMENTAL.txt').write_text(
        'Unsigned prototype. Native install, reboot persistence and complete removal are UNTESTED.\n'
        'Automatic selection after reboot is not implemented. Not a public supported release.\n'
        'Use lifecycle operations only in a disposable single-user VM.\n',encoding='utf-8')
    hashes={str(file.relative_to(destination)):hashlib.sha256(file.read_bytes()).hexdigest()
            for file in sorted(destination.rglob('*')) if file.is_file()}
    (destination/'SHA256.json').write_text(json.dumps(hashes,indent=2)+'\n',encoding='utf-8')
    archive=Path(str(destination)+'.zip')
    with zipfile.ZipFile(archive,'x',compression=zipfile.ZIP_DEFLATED,compresslevel=9) as zip_file:
        for file in sorted(destination.rglob('*')):
            if file.is_file(): zip_file.write(file,str(file.relative_to(destination)))
    print(archive)


if __name__=='__main__':
    parser=argparse.ArgumentParser()
    parser.add_argument('--cargo',default='cargo')
    package(parser.parse_args().cargo)
