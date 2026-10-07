# Windows native lifecycle experiment

**Status: not run. Use a disposable, single-user Windows 11 x64 VM.**
Snapshot the VM first. Do not use a working computer. The prototype is unsigned,
has not passed the WDK/native-loader gate, and cannot establish support claims.

1. Select Spanish Spain, build the layout and manager, run all automated checks.
2. Save `easy-spanish-quotes status` output, original profiles/order/default
   preference, keyboard shortcuts and registry values. Note exact OS/security
   policy/toolchain and snapshot identifier.
3. In an elevated terminal of the **same user**, run
   `easy-spanish-quotes install --single-user-vm`. The helper refuses a different
   owner SID. This flag explicitly asserts the disposable single-user assumption;
   it is not a general-purpose installation mode.
4. Run `easy-spanish-quotes gui --single-user-vm`. Use Seleccionar y probar.
   The GUI checks actual calling-thread KLID and guillemet translation after
   LoadKeyboardLayout; a nonzero native handle alone is not success.
5. Type «Hola», all Spanish punctuation, accents, ñ/Ñ, dead-key combinations,
   Ctrl shortcuts and Alt+numpad in Notepad, Edge, Firefox, Terminal, VS Code
   and elevated Win32 apps. Close the manager and repeat. Record failures.
6. Logout and reboot manually. Inspect enrollment/default and independently
   select/test the layout. The prototype enrolls persistently but intentionally
   does not change default policy until W3 proves a reversible mechanism.
   Therefore automatic selection after reboot is currently UNIMPLEMENTED.
7. Disable, re-enable, then uninstall with the same gated commands. Inspect
   original selection/profile order/default, registry and System32 filename.
   Compare to the original snapshot; do not count scheduled removal as deletion.
8. Keep a layout-using app open during uninstall. If pending-reboot is returned,
   reboot, then run repair to verify deletion and complete receipt cleanup.
9. Repeat with interrupted operations, denied elevation, independently changed
   user settings, modified payload/registration and another loaded user.
   Changed resources must be retained with a diagnostic, not forcibly restored.

Prototype receipts are durable JSON registry values under the product's HKLM
software key, rather than filesystem JSON. Each native step writes intent and
completion and flushes the registry. This bounds privileged paths in M1.
Partial registration with missing ownership values is deliberately retained for
manual W4 inspection; automatic recovery of every possible boundary is NOT yet
implemented. Incomplete-copy recovery accepts only a prefix of the embedded
payload with a matching pending-copy receipt. No arbitrary external DLL is used.

No broad snapshot restoration is performed. If user settings differ after
removal, the receipt is retained and the command reports repair-required. W3
must determine precise owned deltas and preserve independent edits before
production. Unloaded-user references are not provably excluded; the explicit
single-user VM restriction remains mandatory.

Signing/HVCI/Secure Boot/Smart App Control, physical x64, ARM64, feature upgrades
and multiuser support remain separate untested gates. Record results in
docs/validation.md; retain VM snapshots until restoration has been checked.
