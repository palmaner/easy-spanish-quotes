"""Read-only Windows layout inventory. No registration or input synthesis."""
import argparse
import ctypes as C
import json
import platform
from pathlib import Path


def inspect():
    if platform.system() != "Windows":
        raise RuntimeError("This probe requires Windows")
    u = C.WinDLL("user32", use_last_error=True)
    u.GetKeyboardLayout.argtypes = [C.c_uint32]
    u.GetKeyboardLayout.restype = C.c_void_p
    u.GetKeyboardLayoutNameW.argtypes = [C.c_wchar_p]
    u.GetKeyboardLayoutNameW.restype = C.c_int
    u.MapVirtualKeyExW.argtypes = [C.c_uint32, C.c_uint32, C.c_void_p]
    u.MapVirtualKeyExW.restype = C.c_uint32
    u.ToUnicodeEx.argtypes = [C.c_uint32, C.c_uint32, C.POINTER(C.c_ubyte),
                             C.c_wchar_p, C.c_int, C.c_uint32, C.c_void_p]
    u.ToUnicodeEx.restype = C.c_int
    u.GetKeyNameTextW.argtypes = [C.c_int32, C.c_wchar_p, C.c_int]
    u.GetKeyNameTextW.restype = C.c_int
    hkl = u.GetKeyboardLayout(0)
    name = C.create_unicode_buffer(9)
    if not u.GetKeyboardLayoutNameW(name):
        raise C.WinError(C.get_last_error())
    if name.value != "0000040A":
        raise RuntimeError("Only Spanish Spain 0000040A is supported; got " + name.value)
    translations = []
    # Shift, Ctrl, Alt are bits 1,2,4. Caps/Num separate toggles.
    for vk in range(1, 255):
        scan = u.MapVirtualKeyExW(vk, 0, hkl)
        for mods in range(8):
            for caps in (False, True):
                for num in (False, True):
                    state = (C.c_ubyte * 256)()
                    for bit, key in ((1, 16), (2, 17), (4, 18)):
                        if mods & bit:
                            state[key] = 128
                    if mods & 6 == 6:
                        state[165] = 128
                    state[20] = int(caps)
                    state[144] = int(num)
                    out = C.create_unicode_buffer(16)
                    count = u.ToUnicodeEx(vk, scan, state, out, 16, 4, hkl)
                    if count:
                        translations.append({"vk": vk, "mods": mods, "caps": caps,
                                             "num": num, "count": count,
                                             "units": [ord(out[i]) for i in range(abs(count))]})
    # Stateful translation is confined to this dedicated process/thread. It does
    # not send events or alter registration, activation or user preferences.
    dead_keys = {}
    seeds = {(x["vk"], x["mods"], x["units"][0]) for x in translations
             if x["count"] < 0 and not x["caps"] and not x["num"]}
    def translate(vk, mods):
        state = (C.c_ubyte * 256)()
        for bit, key in ((1, 16), (2, 17), (4, 18)):
            if mods & bit: state[key] = 128
        if mods & 6 == 6: state[165] = 128
        out = C.create_unicode_buffer(16)
        count = u.ToUnicodeEx(vk, u.MapVirtualKeyExW(vk, 0, hkl), state, out, 16, 0, hkl)
        return count, [ord(out[i]) for i in range(abs(count))]
    for seed_vk, seed_mods, accent in sorted(seeds):
        for vk in range(1, 255):
            for mods in (0, 1, 2, 3, 6, 7):
                translate(32, 0); translate(32, 0)
                plain_count, plain = translate(vk, mods)
                translate(32, 0); translate(32, 0)
                translate(seed_vk, seed_mods)
                count, units = translate(vk, mods)
                if plain_count == 1 and count == 1:
                    dead_keys[f"{accent}:{plain[0]}"] = units[0]
        translate(32, 0); translate(32, 0)
    scans = {str(scan): u.MapVirtualKeyExW(scan, 3, hkl)
             for scan in list(range(128)) + [0xE000 | s for s in range(128)]
             + [0xE100 | s for s in range(128)]}
    names = {}
    for ext in (0, 1):
        for scan in range(1,128):
            text = C.create_unicode_buffer(128)
            count = u.GetKeyNameTextW((scan << 16) | (ext << 24), text,128)
            if count: names[f"{ext}:{scan}"] = text.value
    return {"schema": 1, "klid": name.value, "architecture": platform.machine(),
            "os": platform.version(), "scans": scans, "translations": translations,
            "dead_keys": dead_keys, "key_names": names}


if __name__ == "__main__":
    parser = argparse.ArgumentParser()
    parser.add_argument("--output", type=Path)
    args = parser.parse_args()
    data = json.dumps(inspect(), ensure_ascii=False, indent=2)
    if args.output:
        # Prevent silently replacing an existing fixture.
        with args.output.open("x", encoding="utf-8") as file:
            file.write(data + "\n")
    else:
        print(data)
