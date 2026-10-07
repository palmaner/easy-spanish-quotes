"""Build a restricted Spanish profile from the committed public-API fixture.

This is a source emitter, not a DLL cloner. No arbitrary baseline/profile is
accepted. Native translation and WDK ABI validation remain release gates.
"""
import json
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
FIXTURE = ROOT / "tests/fixtures/windows-es-26300.json"
MODS = (0, 1, 2, 3, 6, 7)
NONE, DEAD = 0xF000, 0xF001


def tables(data):
    if data["klid"] != "0000040A" or data["schema"] != 1:
        raise ValueError("Unsupported reviewed baseline")
    source = {(x["vk"], x["mods"], x["caps"], x["num"]): x
              for x in data["translations"]}
    rows = []
    for vk in range(1, 255):
        chars, accents = [], []
        for mod in MODS:
            item = source.get((vk, mod, False, False))
            if item and (abs(item["count"]) != 1 or len(item["units"]) != 1):
                raise ValueError("Ligature requires explicit implementation")
            chars.append(NONE if not item else DEAD if item["count"] < 0 else item["units"][0])
            accents.append(item["units"][0] if item and item["count"] < 0 else NONE)
        def output(m, c=False, n=False):
            x = source.get((vk, m, c, n))
            return (x["count"], x["units"]) if x else (0, [])
        caps = output(0) != output(0, True)
        # Verify supported native caps and modifier model over every fixture state.
        for mod in range(8):
            for cap in (False, True):
                for num in (False, True):
                    # Alt+numpad is handled by USER's numeric-input machinery,
                    # before ordinary table lookup. Preserve VK_NUMPAD scans.
                    if 96 <= vk <= 105 and mod in (4, 5):
                        continue
                    model_mod = mod & ~4 if mod in (4, 5) else mod
                    if cap and caps and model_mod in (0, 1): model_mod ^= 1
                    if output(mod, cap, num) != output(model_mod):
                        raise ValueError(f"Unrepresented state vk={vk} mods={mod} caps={cap} num={num}")
        if vk in (90, 88):
            if chars[4] != NONE:
                raise ValueError("Preset would replace a useful AltGr character")
            chars[4] = 0xAB if vk == 90 else 0xBB
        if any(x != NONE for x in chars):
            rows.append((vk, int(caps), chars))
            if DEAD in chars: rows.append((255, 0, accents))
    return rows


def generate():
    data = json.loads(FIXTURE.read_text(encoding="utf-8"))
    rows = tables(data)
    out = ['/* Generated from the reviewed es-ES public-API fixture. */',
           '#include "layout_abi.h"',
           'static ESQ_VK_BIT bits[] = {{16,1},{17,2},{18,4},{0,0}};',
           'static ESQ_MODIFIERS mods = {bits,7,{0,1,2,3,0,1,4,5}};',
           'static ESQ_CHARS chars[] = {']
    for vk, attr, chars in rows:
        out.append(f'{{{vk},{attr},{{' + ','.join(hex(c) for c in chars) + '}},')
    out.extend(['{0,0,{0}}};', 'static ESQ_CHAR_TABLE character_tables[] = {{chars,6,sizeof(ESQ_CHARS)},{0,0,0}};',
                'static ESQ_DEAD dead[] = {'])
    for pair, composed in sorted(data["dead_keys"].items(), key=lambda x:tuple(map(int,x[0].split(':')))):
        accent, base = map(int, pair.split(':'))
        out.append(f'{{0x{((accent << 16) | base):08x},{composed},0}},')
    out.append('{0,0,0}};')
    scans = []
    for s in range(128):
        vk = data["scans"][str(s)] or 0xFF
        if s == 0x36: vk |= 0x100  # distinguish right shift
        if s in (0x37, 0x46): vk |= 0x200  # multi-VK processing
        if s == 0x45: vk |= 0x300
        if s in (0x47,0x48,0x49,0x4b,0x4c,0x4d,0x4f,0x50,0x51,0x52,0x53): vk |= 0xC00
        scans.append(vk)
    out.append('static uint16_t scans[] = {' + ','.join(hex(x) for x in scans) + '};')
    for prefix in (0xE000, 0xE100):
        name = 'e0' if prefix == 0xE000 else 'e1'
        out.append(f'static ESQ_SCAN {name}[] = {{')
        for s in range(128):
            vk = data["scans"][str(prefix | s)]
            if vk: out.append(f'{{{s},{vk | 0x100}}},')
        out.append('{0,0}};')
    for ext in (0,1):
        entries=[]
        for key,name in data['key_names'].items():
            key_ext,scan=map(int,key.split(':'))
            if key_ext!=ext: continue
            identifier=f'name_{ext}_{scan}'
            encoded=name.encode('utf-16-le')
            units=[int.from_bytes(encoded[i:i+2],'little') for i in range(0,len(encoded),2)]+[0]
            out.append(f'static const uint16_t {identifier}[] = {{' + ','.join(map(str,units)) + '};')
            entries.append(f'{{{scan},{identifier}}}')
        table_name='names' if ext==0 else 'extended_names'
        out.append(f'static ESQ_NAME {table_name}[] = {{' + ','.join(entries+['{0,0}']) + '};')
    out.extend(['static const uint16_t *dead_names[] = {0};',
                # Locale low word: AltGr bit; high word: layout version 1.
                'static ESQ_TABLES tables = {&mods,character_tables,dead,names,extended_names,dead_names,',
                'scans,128,e0,e1,0x00010001,0,0,0,4,0};',
                '__declspec(dllexport) ESQ_TABLES *KbdLayerDescriptor(void) { return &tables; }'])
    destination = ROOT / "build/layout"
    destination.mkdir(parents=True, exist_ok=True)
    (destination / "layout.c").write_text('\n'.join(out) + '\n', encoding='utf-8')
    return len(rows)


if __name__ == "__main__":
    print(f"Generated {generate()} native rows; installation remains unvalidated.")
