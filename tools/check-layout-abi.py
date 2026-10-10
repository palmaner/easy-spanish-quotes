"""Compile assertions against an external official kbd.h; never installs a layout."""
import argparse
import hashlib
from pathlib import Path
import subprocess

ROOT=Path(__file__).resolve().parents[1]

if __name__=='__main__':
    parser=argparse.ArgumentParser()
    parser.add_argument('--kbd-header',type=Path,required=True)
    parser.add_argument('--clang',default='clang')
    args=parser.parse_args()
    header=args.kbd_header.resolve(strict=True)
    if header.name.lower()!='kbd.h' or not header.is_file():
        parser.error('--kbd-header must identify the externally acquired kbd.h file')
    output=ROOT/'build/layout-abi-check.o'
    output.parent.mkdir(parents=True,exist_ok=True)
    subprocess.run([args.clang,'--target=x86_64-w64-windows-gnu','-D_WIN32_WINNT=0x0601',
                    '-c',str(ROOT/'native/windows-layout/check_abi.c'),'-I',str(header.parent),
                    '-o',str(output)],check=True)
    print('AMD64 ABI assertions passed; reference header SHA256='+hashlib.sha256(header.read_bytes()).hexdigest())
