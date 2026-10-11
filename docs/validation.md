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

## Disposable VM provisioning — 2026-10-11

- Created an isolated Windows 11 x64 VM with VirtualBox 7.2.20, four virtual
  CPUs, 8 GiB RAM and an 80 GiB dynamically allocated disk. EFI Secure Boot and
  virtual TPM 2.0 are configured; guest confirmation remains pending. NAT has
  no forwarding rules; shared folders, clipboard and drag-and-drop are disabled.
- The Spanish Windows ISO has SHA-256
  `9828703784880421e6602504dfd99064e4cb739cc9b20aba22e3934372a14f9a`.
  The unattended detector identified Windows 11 Pro, image index 6, build
  26300.9457. Installation templates request Spanish Spain input, a dedicated
  local test user and Guest Additions, without hardware-check bypass commands.
  Credentials and generated installation answers remain outside version control.
- Windows Setup was observed progressing. After the user reported completion,
  the running guest displayed a black console. Waking the display did not clear
  it; VirtualBox reported that Guest Additions were not installed or ready.
  Installation completion and the actual guest configuration are therefore
  **unverified**. Recovery investigation precedes a verified clean baseline.
- Preserved an unverified recovery snapshot before restarting. The first restart
  loaded `cdboot_noprompt.efi` from the unattended DVD. After detaching that DVD,
  the guest loaded `bootmgfw.efi` from disk and displayed a Windows loading
  spinner. This establishes disk boot, not completed setup or desktop readiness.
  The DVD boot did not respond to a normal shutdown request; it was powered off
  after preserving the recovery point. Native tests have not started.
- Disk-only boot subsequently displayed Spanish Windows Setup at 0%, 33% and
  42%, followed by loading screens. Setup was still in progress at the last
  observation; there is no verified desktop or working guest-control session.
- No project layout has been installed in the VM or host. Native installation,
  activation, logout/reboot persistence, rollback and removal remain pending;
  physical hardware validation is also pending.

## Setup failure diagnosis and retry — 2026-10-11

- A later boot displayed Windows' installation-cannot-complete dialog. Saved
  the failed VM state and extracted only setup logs from its VDI parent/child
  chain with read-only Dissect parsers, without mounting or editing the disk.
  Full logs and installation credentials remain in ignored local storage.
- `Panther/UnattendGC/setuperr.log` identifies an invalid
  `Microsoft-Windows-Shell-Setup/OOBE/NetworkLocation` value (`Public`) and
  error `0x80220005`. The invalid value was introduced while preparing the
  experiment's local template; this is a provisioning failure, not a keyboard
  layout failure. The next boot also reports failed OOBE pass `0x8030000C`.
- Removed the obsolete setting rather than assigning another network policy.
  [Microsoft documents NetworkLocation as deprecated since Windows 10](https://learn.microsoft.com/en-us/windows-hardware/customize/desktop/unattend/microsoft-windows-shell-setup-oobe-networklocation).
  Prepared a fresh retry VM/disk with the same resources, retaining the failed
  VM, recovery snapshot and logs. The retry has started with the corrected
  template; installation success and guest security settings are not yet verified.
- No native layout lifecycle tests have run. They remain gated on a verified
  guest, Guest Additions access and a clean baseline snapshot.

## Live translation probe preparation — 2026-10-11

- Added a VM-only Rust example comparing 8,192 live ToUnicodeEx translations
  and 768 MapVirtualKeyEx scan mappings with the original Spanish Spain layout.
  It uses the existing ownership/enrollment checks before selection, expects
  only AltGr+Z/X character changes, and attempts to restore the calling-thread
  layout on exit. It reports native API differences and restoration separately.
- Rustfmt, Clippy with warnings denied for all workspace targets, and the release
  example build passed. Executing it without the required VM flag correctly
  failed before accessing keyboard APIs. No installed-layout probe run occurred.
  Application input, dead-key sequences, system default/order restoration and
  persistence still need independent integration tests.
- Retry setup logs showed the file-install phase completed and requested reboot.
  Preserved that state in a recovery snapshot and cold-booted with the unattended
  DVD detached. The next Windows installation phase progressed to 54%; desktop
  readiness and Guest Additions remain unverified.
- VirtualBox rejected attempts to hot-mount post-install optical media. A standard
  nonbootable ISO was built locally and its four Windows installer hashes checked
  against Oracle's existing Guest Additions ISO, but attaching it returned
  `VERR_INVALID_POINTER`. It has not run. Further optical-drive changes are
  deferred while setup continues; this does not establish a Guest Additions or
  keyboard-layout defect.

## Retry desktop and guest-access handoff — 2026-10-11

- Corrected retry setup reached 94%, restarted automatically, and displayed the
  Spanish Windows desktop. This confirms the prior invalid NetworkLocation value
  no longer blocked reaching the desktop. Actual OS/security/profile baseline
  collection remains pending; desktop display alone does not verify usability.
- The desktop did not respond to attempted Start-menu input or normal ACPI
  shutdown. Saved its state in a separate recovery snapshot, then cold-booted
  with the verified nonbootable post-install DVD attached. Cold attachment
  succeeded; earlier hot attachment had failed even for Oracle's original ISO.
  The recovery boot reached the Windows lock screen. Guest sign-in and installer
  security prompts require human interaction before automated access is possible.
- Windows Authenticode verification of the unchanged Guest Additions launcher
  and amd64 installer returned Valid, signer Oracle America, Inc. The DVD is
  labelled ESQ_GUEST_TOOLS. Guest Additions installation remains unverified.
- [CI for the live probe commit passed](https://github.com/palmaner/easy-spanish-quotes/actions/runs/38151805486).
  No ESQ layout was installed in the guest or host. A verified clean baseline,
  native loader/input tests, persistence and removal are still pending.

## Authorized guest sign-in and administrator-access retry — 2026-10-11

- With explicit user authorization for the disposable guest, completed sign-in
  and observed a usable desktop and the ESQ_GUEST_TOOLS optical drive. Host
  automation initially returned GetCursorPos access denied; foreground access
  subsequently recovered. Console text/key automation did not establish reliable
  input. Native VirtualBox keyboard input with separately paced make/break
  scancodes successfully signed in. Two earlier string-input attempts were
  rejected; a harmless aA!123 sample verified the expected character mapping.
- The Oracle-signed Guest Additions launcher ran but reported that administrator
  rights were required. Launching it as administrator produced a guest UAC prompt
  with only No, with no available administrator credential fields. Installation
  did not complete. This is an account/elevation blocker, not an ESQ layout test
  failure.
- The generated answer file specified administrators;users. Microsoft's
  [LocalAccount Group documentation](https://learn.microsoft.com/en-us/windows-hardware/customize/desktop/unattend/microsoft-windows-shell-setup-useraccounts-localaccounts-localaccount-group)
  documents the language-neutral names Administrators and Users. A new template
  uses Administrators exactly. The earlier spelling is a candidate cause; its
  causality has not been established from guest group membership or setup logs.
- Preserved the retry VM's current disk, TPM and saved state. Read-only inspection
  of its complete VDI parent/child chain found an FVE-FS signature on the Windows
  partition; the NTFS-only parser could not extract current setup logs. No offline
  account, registry, encryption or host security changes were made.
- Created a separate retry2 VM with the same four CPUs, 8 GiB RAM, 80 GiB dynamic
  disk, EFI/Secure Boot configuration, TPM 2.0, NAT, disabled clipboard and no
  shared folders. Its generated answer file passed the new preflight, and setup
  was started. Both previous VMs remain preserved. Successful setup, actual
  administrator membership, Guest Additions and the security/profile baseline
  remain to be observed.
- Added a credential-safe answer-file preflight rejecting deprecated
  NetworkLocation, a missing expected account/password and noncanonical or absent
  Administrators membership configuration. All 11 Python test methods passed.
  The prior answer file is rejected as expected. This check does not prove actual
  privileges, installation success or language-dependent behavior on Windows.
- Native installation, activation, app input, logout/reboot persistence,
  original-layout preservation, rollback and clean removal remain untested. No
  ESQ layout has been installed on either the guest or host.
