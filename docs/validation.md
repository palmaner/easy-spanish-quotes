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

## Automated and read-only results (2026-10-08)

- Rust 1.99.0 GNU portable toolchain: SHA-256 checked against official channel metadata.
- LLVM-MinGW 20260908: SHA-256 checked against maintainer release API digest.
- `cargo test --workspace`: 4 core tests passed (mapping validation, SHA-256,
  ownership path validation and preservation of independent changes).
- `cargo run -- status`: live calling-thread Spanish KLID and empty AltGr+Z/X
  detected; foreground HKL reported separately; activation remains unknown.
- Layout compiled as AMD64, zero PE timestamp, no entrypoint/imports, exported
  KbdLayerDescriptor. Artifact is unsigned and not installed.
- `python -m unittest discover -s tests -v`: 3 tests passed, including compiled
  descriptor differential checks against all fixture modifier/caps/num states.
  Exactly 8 state differences: two intended keys × caps × num, all mods=6.
- Dead composition table matches the committed public-API fixture. Alt+numpad
  behavior and scan flags still require native Windows integration validation.
- W1 is partially checked, NOT passed: WDK ABI comparison, native loader and
  real dead-key sequences under the candidate remain outstanding.

## Further prototype work

- Native enrollment enumeration observed `0c0a:0000040a`. The prototype now
  captures the enrollment language separately from the keyboard KLID.
- GUI opened locally; Spanish labels, status refresh and accessible test-field
  identification checked. Text-entry automation was interrupted by user input;
  no candidate keyboard mapping was exercised. DPI/high-contrast acceptance is
  still pending. GUI is a preview, not a completed first public release.
- Eight Rust tests pass, including simulated intent failure, crash after native
  mutation, completion-write failure and foreign-resource preservation. These
  are shared-core protocol tests, not real privileged Windows fault tests.
- Lifecycle commands compile but have NOT been run: they require an elevated
  same-user terminal and explicit disposable single-user VM flag. Default policy
  selection, every native rollback boundary, upgrades and production privilege
  separation remain incomplete; see docs/experiments/windows.md.
- Original key names added through public-API capture. Final unsigned candidate
  size: 10,240 bytes; ZIP: 3,662 bytes. Signing cost remains unmeasured.
- No VM runtime was found. User requested hardware tests remain documented as
  pending. No reboot, installation or removal test was attempted on the host.
- Final local checks: 10 Rust tests and 3 Python layout tests passed; rustfmt
  check and Clippy with warnings denied passed. Native ID collision/exhaustion
  tests were added. CI is configured separately; local results do not claim CI success.
