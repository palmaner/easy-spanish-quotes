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
- Hosted GitHub Actions run [37701178137](https://github.com/palmaner/easy-spanish-quotes/actions/runs/37701178137)
  passed all four jobs for implementation commit 078b1dd: Windows/macOS/Linux
  core checks and native Windows data/build checks. These do not install a layout.
- Final review added hash/registration verification before enrollment/activation
  and extended the lifecycle lock through error recording. Local checks passed
  again. Detailed Rust standard-library license notices are included in bundles.

## Recovery review — 2026-10-10

- Cleanup now durably preserves interrupted-copy evidence before replacing the
  pending intent. Simulated receipt serialization/retry tests accept only the
  original payload prefix, reject foreign bytes and other builds, and reject
  partial deletion after a completed copy. Older receipts default to no partial
  ownership unless their original interrupted-copy intent is still present.
- Native cleanup distinguishes explicit missing-file/key errors from access
  failures. Unknown observations retain the receipt; dangling links are observed
  and rejected rather than interpreted as absence. Artifact diagnostics use the
  same reparse-point checks as enrollment verification.
- Repeated disable/uninstall/repair with no receipt returns absent. These native
  branches compile but remain untested on an installed Windows layout. No host
  registration, enrollment, removal or reboot was performed.
- Local verification: 13 Rust tests, three compiled-layout Python tests, rustfmt
  and Clippy with warnings denied pass. This is automated evidence only; W1–W5
  and physical-machine acceptance remain pending.

## Observed-state diagnostics — 2026-10-10

- Reports distinguish durable journal state from resource observations. Enabled
  requires verified artifact/registration and enrollment for the receipt owner;
  enrollment for another account is never treated as owner evidence.
- Six shared-core scenario tests cover stale enabled receipts, modified/missing
  resources, unreadable resources, interrupted intents, unknown owner enrollment,
  and receipt cleanup after a pending-reboot DLL disappears. These simulate
  observations; they do not exercise privileged Windows resources.
- `doctor` is read-only, declares native validation pending and reports findings.
  Returned repair/reboot states use exit codes 7/8. Full native error-code
  classification remains pending. See diagnostics.md for the prototype contract.
- Local verification: all 19 Rust tests and three Python layout tests passed;
  rustfmt, Clippy with warnings denied, and release builds passed. Live read-only
  `doctor` reported the original Spanish enrollment and no project installation.
  No mutation or reboot test was run.

## Argument and bundle isolation checks — 2026-10-10

- All command arguments are parsed before dispatch, including the GUI's elevated
  helper arguments. Three parser scenarios per binary cover unknown/duplicate
  flags, malformed or missing owner values, defaults and unsupported commands.
- Three packaging tests verify rejected traversal labels and preservation of
  existing archives/directories before build-tool invocation. Tests use an
  isolated workspace directory because the sandbox's default temporary directory
  denied writes; no external directory permissions were changed.
- Local Rust tests (19 core plus three parser scenarios in each binary), Clippy
  and all six Python tests passed. Native lifecycle validation remains pending.

## Header ABI and scan flags — 2026-10-10

- External reference: Microsoft.Windows.SDK.cpp 10.0.28000.2526 from the official
  NuGet endpoint. Package SHA-256:
  `be1b419491607eae6f7c57844ebab39face9643c51e2af1d9176a3ba0d0b23fc`.
  kbd.h SHA-256: `45c584d1df780dc149a5dea0a8cda38a35fd9b7fd82eb0d91c4aeab2018a1493`.
  These are observed download hashes, now pinned for repeat checks; NuGet package
  signature verification was not performed. Headers remain in ignored tools.
- Compile-only assertions with pinned LLVM-MinGW passed for all descriptor
  offsets/member sizes, subsidiary structures, modifier-tail offset, flags and
  character sentinels. Windows primitive types came from LLVM-MinGW; a full
  Microsoft compiler/SDK/WDK build remains pending.
- [Microsoft's keyboard sample](https://github.com/microsoft/Windows-driver-samples/blob/main/input/layout/all_kbds/kbdgr/kbdgr.c)
  distinguishes extended E0 entries from E1 Pause. The emitter incorrectly added
  KBDEXT to both prefixes; corrected E1 entries now preserve the unflagged VK.
  Compiled-descriptor tests check all scan high flags and E0/E1 tables, alongside
  fixture translations. All six Python tests passed after rebuilding.
- This is an automated W1 improvement, not native-loader acceptance. App input,
  installation, logout/reboot persistence and complete removal remain untested.
- Acquisition references: [Microsoft WDK NuGet guidance](https://learn.microsoft.com/en-us/windows-hardware/drivers/install-the-wdk-using-nuget)
  and [the pinned SDK package](https://www.nuget.org/packages/Microsoft.Windows.SDK.cpp/10.0.28000.2526).
