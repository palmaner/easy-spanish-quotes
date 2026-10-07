# Third-party notices

Own code: Apache-2.0. No Microsoft keyboard-layout source, proprietary layout
binary, MSKLC executable, or reverse-engineered layout is redistributed.

Rust standard library/toolchain: MIT OR Apache-2.0; development dependency.
The statically linked Rust runtime is also covered by the toolchain copyright
and MIT notices in third-party/rust (from the verified Rust 1.99.0 artifact).
Windows SDK/WDK: Microsoft terms; development dependency, not bundled.

Cargo.lock pins serde/serde_json, sha2 and transitive dependencies, Windows
bindings and winreg. Their actual license metadata and license files must be
included in binary bundles. Packaging collects these from Cargo metadata and
cached crate sources; see the generated licenses directory in each bundle.

Any future reused source requires a verified license and retained notices.
Microsoft Windows-driver-samples is MS-PL, lelegard/winkbdlayouts is BSD-2-Clause,
and divvun/kbdgen is MIT OR Apache-2.0. They are research references, not copied
dependencies. Ukelele's publisher describes it as freeware; do not treat it as
permissively licensed source.
