# Validation record

Evidence labels: **automated**, **local read-only observation**, **real-system
integration**, **untested**. Update this file with commands, versions and results.

## Initial environment (2026-10-08)

- Windows x64 reports OS version 10.0.26300.0. Edition not verified: CIM access denied.
- Read-only Win32 probe: calling-thread KLID `0000040A`, HKL `40A0C0A`.
- `ToUnicodeEx` with non-mutating flag 4: AltGr+Z and AltGr+X both return 0.
- Initial machine has Git; Rust, MSVC and Windows SDK/WDK were not on PATH.
- No custom layout has been installed, activated or removed on this working machine.

## Required integration evidence — all UNTESTED

| Experiment | Required evidence |
|---|---|
| W1 | Native DLL descriptor/ABI, complete Spanish differential translation |
| W2 | VM install, selection, representative apps, logout and reboot persistence |
| W3 | Exact restoration of profile order and automatic/explicit default policy |
| W4 | Failure at each mutation boundary; recovery after process termination |
| W5 | Locked DLL removal, reboot cleanup verification, retained ownership receipt |
| W6 | Signing, Secure Boot, HVCI, Smart App Control and Defender behavior |
| W7 | Physical x64 and ARM64; mixed emulated/native applications |
| W8 | Windows feature upgrade retains layout and permits clean removal |
| W9 | Other-user references prevent unsafe machine-wide deletion |
| M1–M3 | macOS TIS discovery, persistence, cache/logout, ANSI/ISO dead keys |
| L1–L4 | GNOME/Plasma/Sway, X11, updates, IBus/Fcitx and group/hotplug behavior |

M1 is not accepted until W1–W5 pass in a disposable Windows environment and the
principal lifecycle passes on physical hardware. Publishing remains gated.
