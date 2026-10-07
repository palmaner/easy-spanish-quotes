"""Validate data in the compiled candidate without registering it with Windows."""
import ctypes as C
import importlib.util
import json
import platform
import struct
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('emitter', ROOT/'tools/generate_layout.py')
emitter = importlib.util.module_from_spec(spec)
spec.loader.exec_module(emitter)


class Row(C.Structure):
    _fields_ = [('vk', C.c_uint8), ('attributes', C.c_uint8), ('chars', C.c_uint16 * 6)]
class CharTable(C.Structure):
    _fields_ = [('rows', C.POINTER(Row)), ('mods', C.c_uint8), ('stride', C.c_uint8)]
class VkBit(C.Structure):
    _fields_ = [('vk', C.c_uint8), ('bits', C.c_uint8)]
class Modifiers(C.Structure):
    _fields_ = [('keys', C.POINTER(VkBit)), ('max', C.c_uint16), ('numbers', C.c_uint8 * 8)]
class Dead(C.Structure):
    _fields_ = [('pair', C.c_uint32), ('composed', C.c_uint16), ('flags', C.c_uint16)]
class Scan(C.Structure):
    _fields_ = [('scan', C.c_uint8), ('vk', C.c_uint16)]
class Name(C.Structure):
    _fields_ = [('scan', C.c_uint8), ('text', C.POINTER(C.c_uint16))]
class Tables(C.Structure):
    _fields_ = [('mods', C.POINTER(Modifiers)), ('chars', C.POINTER(CharTable)),
                ('dead', C.POINTER(Dead)), ('names', C.c_void_p), ('names_ext', C.c_void_p),
                ('dead_names', C.c_void_p), ('scans', C.POINTER(C.c_uint16)),
                ('scan_count', C.c_uint8), ('e0', C.POINTER(Scan)), ('e1', C.POINTER(Scan)),
                ('flags', C.c_uint32), ('lig_max', C.c_uint8), ('lig_stride', C.c_uint8),
                ('ligatures', C.c_void_p), ('type', C.c_uint32), ('subtype', C.c_uint32)]


class LayoutTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.fixture = json.loads((ROOT/'tests/fixtures/windows-es-26300.json').read_text('utf-8'))

    def test_preset_conflict_is_rejected(self):
        data = json.loads(json.dumps(self.fixture))
        data['translations'].append({'vk':90,'mods':6,'caps':False,'num':False,
                                      'count':1,'units':[8364]})
        with self.assertRaises(ValueError): emitter.tables(data)

    def test_native_model_accepts_reviewed_states(self):
        self.assertEqual(len(emitter.tables(self.fixture)), 73)

    @unittest.skipUnless(platform.system() == 'Windows', 'native descriptor requires Windows')
    def test_compiled_descriptor_and_differential_output(self):
        path = ROOT/'build/layout/esq-layout.dll'
        blob = path.read_bytes()
        pe = struct.unpack_from('<I', blob, 0x3c)[0]
        self.assertEqual(blob[pe:pe+4], b'PE\0\0')
        self.assertEqual(struct.unpack_from('<H',blob,pe+4)[0], 0x8664)
        optional = pe+24
        self.assertEqual(struct.unpack_from('<I',blob,optional+16)[0], 0)  # no entry point
        self.assertEqual(struct.unpack_from('<II',blob,optional+120)[0:2], (0,0))  # no imports
        # Only load our locally built no-entrypoint payload, after PE checks.
        dll = C.WinDLL(str(path))
        descriptor = dll.KbdLayerDescriptor
        descriptor.restype = C.POINTER(Tables)
        table = descriptor().contents
        self.assertEqual(C.sizeof(Tables),104)
        self.assertEqual(table.flags,0x10001)
        self.assertEqual(table.scan_count,128)
        self.assertEqual(table.chars[0].stride,C.sizeof(Row))
        self.assertEqual(table.chars[1].mods,0)
        rows = {}
        ptr = table.chars[0].rows
        i = 0
        while ptr[i].vk:
            row = ptr[i]
            if row.vk != 255:
                rows[row.vk] = (row.attributes, list(row.chars),
                                list(ptr[i+1].chars) if 0xf001 in row.chars else None)
            i += 1
            self.assertLess(i,256)
        source = {(x['vk'],x['mods'],x['caps'],x['num']): (x['count'],x['units'])
                  for x in self.fixture['translations']}
        changes = []
        for vk in range(1,255):
            for mods in range(8):
                for caps in (False,True):
                    for num in (False,True):
                        old = source.get((vk,mods,caps,num),(0,[]))
                        if 96 <= vk <=105 and mods in (4,5): continue  # USER Alt+numpad
                        attr, chars, accents = rows.get(vk,(0,[0xf000]*6,None))
                        lookup = mods ^ 1 if attr & 1 and caps and mods in (0,1,4,5) else mods
                        col = table.mods.contents.numbers[lookup]
                        value = chars[col]
                        new = (0,[]) if value == 0xf000 else (-1,[accents[col]]) if value == 0xf001 else (1,[value])
                        if new != old:
                            changes.append((vk,mods,caps,num))
                            self.assertIn(vk,(90,88)); self.assertEqual(mods,6)
                            self.assertEqual(new,(1,[0xab if vk == 90 else 0xbb]))
        self.assertEqual(len(changes),8)  # two keys × caps × num
        native_dead = {}
        i = 0
        while table.dead[i].pair:
            row = table.dead[i]
            native_dead[f'{row.pair >> 16}:{row.pair & 65535}'] = row.composed
            self.assertEqual(row.flags,0)
            i += 1
            self.assertLess(i,1024)
        self.assertEqual(native_dead,self.fixture['dead_keys'])
        for scan in range(128):
            self.assertEqual(table.scans[scan] & 255,self.fixture['scans'][str(scan)] or 255)
        names={}
        for ext, address in enumerate((table.names,table.names_ext)):
            ptr=C.cast(address,C.POINTER(Name))
            i=0
            while ptr[i].scan:
                units=[]; j=0
                while ptr[i].text[j]:
                    units.append(ptr[i].text[j]); j+=1
                    self.assertLess(j,128)
                names[f'{ext}:{ptr[i].scan}']=bytes(b for unit in units for b in unit.to_bytes(2,'little')).decode('utf-16-le')
                i+=1; self.assertLess(i,128)
        self.assertEqual(names,self.fixture['key_names'])


if __name__ == '__main__': unittest.main()
