# easy-spanish-quotes — implementation specification

Approved 2026-10-08. Own code: Apache-2.0. Technical documentation: English;
README and user experience: Spanish. Windows feasibility precedes investment in
the complete infrastructure. This document specifies intended behavior, not a
claim that the operating-system integration has passed.

## 1. Product goals and non-goals

Make U+00AB and U+00BB easy to type through persistent native keyboard layouts,
without a background application where the platform permits. Preserve the
original input configuration. Support inspect, install, enable, disable,
configure and uninstall, with useful diagnostics and recovery.

The first target is Windows 11 x64, Spanish (Spain), KLID 0000040A, AltGr+Z and
AltGr+X. The first public release must include a small Spanish GUI. CLI-only
builds are development prototypes. macOS and Linux remain production goals.
This is not a general remapper, IME, key logger, or universal hotkey engine.
Do not replace Microsoft layouts or ship a resident hook just for flexibility.

## 2. Research evidence and terminology

Every claim below uses these evidence categories: **D** documented behavior;
**S** confirmed in upstream source; **T** third-party implementation evidence;
**H** hypothesis requiring native validation. Test results belong in
`docs/validation.md`; source evidence never substitutes for integration tests.

**D:** Windows layout DLLs export `KbdLayerDescriptor` and describe static
keyboard tables. [Microsoft samples](https://learn.microsoft.com/en-us/samples/microsoft/windows-driver-samples/keyboard-layout-samples/)
and [sample source](https://github.com/microsoft/Windows-driver-samples/tree/main/input/layout)
demonstrate the mechanism. Ignore historical advice to overwrite system DLLs.
**H:** our DLL will install and survive reboot on supported current Windows 11.

**D:** `GetKeyboardLayout` returns an HKL for a thread; HKL is not a KLID.
`GetKeyboardLayoutNameW` identifies the calling thread, not the foreground app.
[HKL API](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getkeyboardlayout),
[name API](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-getkeyboardlayoutnamew).
**D:** `LoadKeyboardLayout` has focus-dependent behavior on Windows 8+ and may
return a fallback. Verify selected identity and translation, not just nonzero
return. [Load API](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-loadkeyboardlayouta).

**D:** `ToUnicodeEx` flag 4 avoids mutating dead-key state on Windows 10 1607+.
Zero means no output; a negative count means a dead key. Inspect AltGr as
Ctrl+Alt with the right-Alt state; do not synthesize input during inspection.
[translation API](https://learn.microsoft.com/en-us/windows/win32/api/winuser/nf-winuser-tounicodeex).
**Local observation:** the initial machine returned no output for AltGr+Z/X on
KLID 0000040A. This does not establish compatibility with other layouts.

**D:** Input.dll provides `InstallLayoutOrTip`, `EnumEnabledLayoutOrTip` and
`SetDefaultLayoutOrTip`. Resolve dynamically from a trusted system path. Their
documentation contains inconsistent example prototypes, so verify current
SDK declarations and ABI before using defaults. Never use broad default-user
or clean-install flags. [install](https://learn.microsoft.com/en-us/windows/win32/tsf/installlayoutortip),
[enumerate](https://learn.microsoft.com/en-us/windows/win32/tsf/enumenabledlayoutortip),
[default](https://learn.microsoft.com/en-us/windows/win32/tsf/setdefaultlayoutortip).

**D:** delayed deletion through `MoveFileEx` requires privilege and reports
scheduling, not completed deletion. [API](https://learn.microsoft.com/en-us/windows/win32/api/winbase/nf-winbase-movefileexw).
**D:** Scancode Map maps scan codes rather than Unicode; PowerToys Keyboard
Manager needs a running process. [Microsoft keyboard drivers](https://learn.microsoft.com/en-us/previous-versions/windows/hardware/hid/keyboard-and-mouse-class-drivers),
[PowerToys](https://learn.microsoft.com/en-us/windows/powertoys/keyboard-manager).

## 3. Alternatives considered

| Approach | Persistence | Resident process | Reversal | Decision |
|---|---|---|---|---|
| Native layout tables | OS-managed | No | Separate owned layout | Selected, real-system gate |
| Scancode Map | Registry/reboot | No | System-wide | Cannot emit guillemets |
| Low-level hooks / PowerToys | Tool-managed | Yes | Stop tool | Reject for primary solution |
| TSF/IME | OS-managed | Usually runtime component | More complex | Excessive for two symbols |
| MSKLC/kbdutool | Native layout | No | Installer-dependent | Research/build reference |
| Binary patch existing DLL | Native | No | Fragile | Reject provenance/ABI risk |
| Compose sequences | Platform-dependent | Varies | Native setting | Useful Linux fallback |

Rust is selected for lifecycle/configuration logic and FFI. C emits Windows
layout tables. C++ has good Win32 access but adds memory-safety burden; Go adds
runtime size and awkward layout ABI generation; Zig is viable but introduces
toolchain maintenance; Swift is selected only for the native macOS shell.
Build-time SDK/WDK dependencies are acceptable; no user compiler dependency.

## 4. Decision matrix and architecture gate

Native layouts best meet persistence, small runtime footprint and focused UX.
Reversibility and modern Windows installation remain experiment-driven risks.
Build one reviewed profile first. Do not build a 650-artifact catalog or a
large backend framework before W1–W5 pass. Implement independent verification
and the first GUI while hardware validation is pending, but label the backend
unvalidated and do not publish a supported release.

## 5. Cross-platform architecture

Use a Cargo workspace: core product logic, CLI, Windows backend, Windows GUI,
then macOS and Linux backends. Native layout source and build tools are separate
from the controller. Platform adapters expose inspection, capabilities,
installation planning, application, installation verification and compensation.
Represent native actions explicitly; do not conceal platform-specific restart,
privilege, group or input-method constraints.

The shared core owns mapping validation, status transitions, journals,
three-way restoration, diagnostics and version migrations. Native backends own
OS paths, FFI, desktop settings and artifact formats. UI shells consume the
same operations and outcomes as CLI. No daemon, tray service or global capture.

## 6. Windows implementation

Author and review a Spanish Spain baseline in C, preserving all scan codes,
modifier layers, caps/num behavior, dead keys and composed characters. Add only
AltGr+Z U+00AB and AltGr+X U+00BB. Do not reconstruct arbitrary binary layouts.
Do not redistribute or patch KBDSP.DLL. Before installation, differential-test
the complete candidate against the original, permitting only intended changes.

Compile a native x64 data DLL with static KBDTABLES, an exported descriptor,
no entrypoint and no CRT imports. Validate pointer widths, descriptor/export,
sentinels and PE imports. Keep safety settings enabled for the manager;
layout-specific linker requirements must not weaken the controller.

Allocate a collision-free custom Spanish KLID in Axxx040A and a globally
unique nonzero Layout Id. Use an immutable product/version/hash filename in
native System32 and an owned registry key under HKLM
`SYSTEM\CurrentControlSet\Control\Keyboard Layouts`. Register Layout File,
Layout Text and Layout Id. Registry values and payload hash must match the
receipt before deletion. Never reuse or overwrite an existing unknown key/file.

Machine-wide registration needs an elevated bounded helper. Enrollment and
selection run as the original user. UAC with another administrator must not
apply input settings to that administrator. MVP supports one owner SID; retain
machine resources if other-user references cannot be safely excluded.

Enroll through Input.dll after machine registration. Keep original profiles;
never replace the user's entire language list. Separate enabled/installed
status from active state. Show calling-thread and foreground-thread state
distinctly; foreground KLID cannot be inferred blindly from an HKL.
Activation must verify translation in the focused test window.

Capture source order and automatic versus explicit default policy before
mutation. W3 establishes the supported precise persistence mechanism. Until
then, enrollment/selection is experimental and full restoration is unproven.
Disable switches to the retained original before removing enrollment;
uninstall restores owned user deltas, then unregisters owned machine data.
Locked files remain pending with receipt until deletion is verified after reboot.

## 7. macOS implementation

**D:** user layouts can live in `~/Library/Keyboard Layouts`; Apple documents
XML keylayout structures, dead-key actions and modifier maps in archived
[TN2056](https://developer.apple.com/library/archive/technotes/tn2056/_index.html).
The document is historical: current caching/logout behavior is **H**.

Ship a reviewed Spanish and Spanish ISO keylayout bundle, preserving Option,
Command, dead-key and ANSI/ISO behavior. Do not assume built-in layouts have
extractable XML. Use TIS to distinguish selected source from underlying keyboard
source when an IME is active. Validate register/enable/select/disable capability
and prototypes against the current SDK. Use UCKeyTranslate only for inspection,
not event interception. [Apple translation API](https://developer.apple.com/documentation/coreservices/1390584-uckeytranslate).

Install owned files per user, preserve original selected source, remove through
TIS and owned bundle deletion. Return pending-logout when required; do not edit
private preference plists. SwiftUI shell, universal native manager, architecture
independent layout data, Developer ID signing and notarized DMG. Login/FileVault
input is outside the first supported scope. No Accessibility/key-capture TCC.
[Developer ID](https://developer.apple.com/developer-id/).

## 8. Linux implementation

**D:** libxkbcommon supports additive user XKB directories; discoverability and
X11 differ. [custom configuration](https://xkbcommon.org/doc/current/custom-configuration.html),
[compatibility](https://xkbcommon.org/doc/current/xkbcommon-compatibility.html).
Do not edit distribution-owned symbols/rules. Generate a small product symbols
file and required registry metadata only for consumers that use libxkbregistry.

Use desktop adapters: GNOME Wayland GSettings, Plasma configuration and tested
reload/switch APIs, Sway additive include plus IPC. **S:** GNOME sources uses
`a(ss)` and `current` is deprecated/ignored.
[schema](https://github.com/GNOME/gsettings-desktop-schemas/blob/master/schemas/org.gnome.desktop.input-sources.gschema.xml.in).
Preserve sources, order, groups, model and options. Do not use Shell eval or
extensions to fake a public activation API. Active state may be unknown until
a focused native test window observes the supplied keymap.

**D:** Wayland clients receive compositor keymaps, not a universal configuration
API. [protocol](https://wayland.freedesktop.org/docs/html/apa.html).
**S:** Sway xkb_file overrides other XKB settings; avoid unintentionally
discarding groups/options. [manual source](https://github.com/swaywm/sway/blob/master/sway/sway-input.5.scd).
X11 follows tested desktop adapters; setxkbmap on XWayland is not a global
Wayland solution. IBus/Fcitx remain untouched unless explicitly supported.
Offer Compose+<+< and Compose+>+> as a native alternative where available.
Do not assume Linux AltGr+Z/X is empty without querying the actual keymap.
GTK4 native UI and distribution packages; no Flatpak-first privileged installer.

## 9. Data and configuration model

Use a versioned JSON configuration with profile ID, physical opening/closing
keys, platform-supported modifier layer and explicit replacement consent.
MVP has one fixed preset, so no arbitrary paths or DLLs in the configuration.
Physical key identity is separate from printed character identity.

Receipts include schema/product version, original owner SID/UID, baseline user
state, owned native IDs, immutable artifact names/hashes, last applied state,
pending/completed native actions and pending cleanup. Persist before each
mutation and after read-back. Status: absent, installed-disabled, enabled,
pending-logout, pending-reboot, repair-required. Activation: active, inactive,
unknown. Do not infer success from planned state.

## 10. Installation, rollback and recovery

Plan first, inspect conflicts, validate payload and ownership, journal intent,
apply one bounded action, verify, then mark completion. Reverse completed owned
actions on failure. A crash between mutation and completion requires read-back
to determine actual state. Atomic receipt replacement and durable flush are
required. Do not delete the recovery record while cleanup is pending.

Restoration compares baseline, last applied and current state: restore only
settings that still equal our applied value. Preserve later independent user
changes; surface divergence rather than forcing a snapshot over them. Idempotent
retry must converge or return repair-required with exact safe actions.

## 11. Permissions and elevation

Keep user actions unprivileged. Helper accepts only product operations and
trusted compiled catalog identifiers, never arbitrary registry paths or DLLs.
Validate SID, ACLs, reparse points, canonical paths, signatures/hashes and
product ownership inside the helper. User-writable receipts are not authority
for privileged deletion. Store machine receipts in a protected location.
No automatic machine installation on the developer's working system.

## 12. CLI and graphical experience

Commands: status, doctor, install, enable, disable, uninstall, configure, repair.
Support JSON diagnostics and dry-run; a prototype may expose a smaller subset
but must reject unsupported commands clearly. Exit codes: 0 completed, 2 invalid
input, 3 unsupported, 4 denied, 5 conflict, 6 failed/rolled back, 7 repair required,
8 restart/logout/user action required. Explicit overwrite consent is distinct
from unattended execution.

GUI: Spanish labels, original layout identification, fixed preset selector,
Activar/Desactivar/Quitar, native UAC when necessary, readable status and a
focused test field for «Hola». No developer jargon in ordinary flows. Do not
show success while pending restart. Keyboard navigation, accessible names,
high contrast and scaling are first-release requirements. Close leaves native
mapping functional. No tray process.

## 13. Repository structure

Cargo workspace for shared core, CLI and platform adapters; native Windows C
tables/build scripts; platform UI shells; reviewed profiles; fixtures and
failure-injection tests; research, experiment and compatibility documents;
packaging and CI. Keep generated artifacts ignored. Commit verified source and
lockfile, never signing keys, machine receipts, personal diagnostics or binaries
from the user's operating system.

## 14. Dependencies and licensing

Own code Apache-2.0. Start with standard-library-only Rust where feasible.
Introduce dependencies only for concrete needs, with locked versions and
license review. SDK/WDK are development tools, never bundled runtime installers.
Microsoft samples are MS-PL; copied derivatives need those notices and cannot
be silently relicensed. Third-party winkbdlayouts is BSD-2-Clause and kbdgen is
MIT/Apache-2.0; study rather than copy whole installers.
[winkbdlayouts](https://github.com/lelegard/winkbdlayouts),
[kbdgen](https://github.com/divvun/kbdgen),
[Microsoft license](https://github.com/microsoft/Windows-driver-samples/blob/main/LICENSE).
Ukelele is freeware according to [its publisher](https://software.sil.org/ukelele/).
The specific TecladoComillasLatinasAngulares project is research evidence only
until its actual source/license provenance has been verified.

## 15. Testing strategy

Core: invalid pairs, conflicts/dead keys, state transitions, three-way restore,
all transaction boundaries, replay, divergence, migrations and corruption.
Layout: PE ABI/import/export checks and differential native translation across
scan codes, modifiers, caps/num and dead-key transitions. Isolate stateful tests.
Never blindly load an untrusted DLL just to validate it.

Integration: disposable snapshot-based Windows VM then physical hardware;
Notepad, Edge, Firefox, Terminal, VS Code, Win32 and elevated applications.
Observe actual input, logout/reboot persistence and removal. Test denial,
locked file, interrupted install and independent settings changes. Hosted CI
does not install a global layout. macOS/Linux require native desktop sessions,
not merely compilation. Record exact OS, architecture, baseline and toolchain.

## 16. CI and release

Run core tests on Windows/macOS/Linux and native compilation on applicable
runners. Pin build tools/lockfile, produce hashes, dependency notices, SBOM and
provenance. Separate reproducible unsigned content from timestamped signing.
Early artifacts are clearly experimental. First public Windows release includes
GUI, signed payloads and setup/removal entry point; zip prototypes are not that
release. Signing does not guarantee SmartScreen reputation. No fabricated
certificates or signing claims. macOS notarized DMG; Linux native packages.

Make meaningful commits by verified change/milestone. Push completed milestones
to the personal public repository. No force-push or replacement of existing
work. Keep incomplete hardware gates visible in README and release notes.

## 17. Security

No keyboard logging, network telemetry, arbitrary DLL execution, binary patching,
background hooks, vendor-file modification or broad registry import. Treat
layouts as trusted native code. Validate build and payload provenance. Prevent
path traversal, symlink/reparse substitution and confused-deputy privilege use.
Use immutable artifacts and least privilege. Never restore unrelated settings.

## 18. Limitations

Windows custom layouts do not represent arbitrary hotkeys; AltGr can collide
with application Ctrl+Alt shortcuts. Unknown layouts/IMEs are unsupported until
reviewed. Signing and OS security policies may block artifacts. macOS caches
and secure-login input differ from user-session behavior. Linux desktop/input
stack compatibility is explicit, not universal. Architecture-specific Windows
DLLs require real native testing; Windows 10 is a later compatibility target.

## 19. Risks and unresolved questions

Precise default-policy restoration, layout discovery, modern security acceptance,
OS upgrades, multiuser references, locked deletion, complete Spanish baseline,
ARM emulation, macOS cache behavior and Linux registry consumers remain gates.
The 650-pair catalog is provisional: measure actual compressed/uncompressed
size, signing duration, build/test cost and maintenance burden before expansion.
Compare fixed preset, a few presets and all ordered letter pairs. Default to a
small signed catalog. Never compile on end-user machines as an easy workaround.

## 20. Concrete experiments

W1: build one native Spanish candidate; inspect ABI and exhaustive differential
translation. W2: install/enroll/select in VM, representative apps, logout/reboot.
W3: automatic and explicit default policy, multiple sources/order, exact revert.
W4: fail/crash before and after every machine/user action; recover idempotently.
W5: keep DLL loaded, uninstall, restart and verify final cleanup.
W6: signing, HVCI/Secure Boot/Smart App Control/Defender. W7: physical x64/ARM64
and x86/x64 emulated apps. W8: feature upgrade and subsequent removal.
W9: another user's enrollment, refusal to delete referenced resources.

M1: current SDK TIS discovery/selection/persistence; M2: cache/logout removal;
M3: ANSI/ISO, dead keys and Option conflicts. L1: GNOME/Plasma/Sway persistence;
L2: package/OS upgrades; L3: X11 adapter; L4: IBus/Fcitx, groups, hotplug and
toolkits. Each experiment records expected state, observed state and restoration.

## 21. MVP acceptance

M1 requires native DLL W1 and real W2–W5, unchanged original layout, only two
translation differences, no resident process, reversible lifecycle, truthful
status and retained recovery information. VM evidence plus principal lifecycle
on physical hardware is required. Missing access is documented as untested;
automated checks alone do not accept M1 or authorize supported publication.

## 22. Milestones through production

M0: specification, licenses, reproducible profile/build and read-only probes.
M1: one Windows layout and minimal reversible lifecycle with real-system gate.
M2: durable recovery, security/elevation, upgrades and immutable artifacts.
M3: first public Windows release with Spanish GUI and signed distribution.
M4: measured preset/customization expansion. M5: macOS native backend/UI.
M6: Linux Wayland adapters then tested X11. M7: ARM64, additional reviewed
layouts, multiuser and Windows compatibility expansion. 1.0 requires maintained
cross-platform compatibility matrix, accessible UIs and verified release process.

Continue independent implementation and automated verification when hardware
access is unavailable. Do not claim milestone completion or release support
until its acceptance evidence exists. Preserve the product's cross-platform
vision without treating three native environments as identical.
