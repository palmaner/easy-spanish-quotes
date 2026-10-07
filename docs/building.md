# Building and verification

Rust stable, Python 3.12+ and a native Windows C toolchain are development
dependencies. The prototype was built with Rust 1.99.0 GNU and LLVM-MinGW
20260908. The maintainer distributes LLVM-MinGW at
https://github.com/mstorsjo/llvm-mingw/releases. These tools are not shipped to users.

For a GNU Rust installation, its bundled MinGW linker can be selected with
`RUSTFLAGS=-C link-self-contained=yes`. MSVC Rust requires Visual Studio build
tools and SDK. Dependencies are locked in Cargo.lock.

```powershell
./tools/build-layout.ps1
python -m unittest discover -s tests -v
cargo test --workspace --locked
cargo run --locked -- status
```

The layout emitter accepts only the committed Spanish fixture, not arbitrary
layouts. It produces C source and an unsigned DLL in build/layout. Header ABI
declarations are independently authored and must be compared to the current
WDK before supported release. The LLVM build proves toolchain feasibility; it
does not replace the planned supported SDK/WDK build and integration gate.

The fixture was captured through public MapVirtualKeyEx/ToUnicodeEx APIs. It
contains keyboard behavior, not a copied Windows binary. Stateful dead-key
probes run on the probe's dedicated thread without injecting events or changing
the user's registration/selection. Alt+numpad interpretation is performed by
Windows outside the ordinary character tables and remains a native test case.

The descriptor test loads only the freshly compiled, checked, no-entrypoint
local candidate. It checks table data, not kernel keyboard-loader behavior.

To create an unsigned local experiment bundle after building the layout:

```powershell
cargo build --release --workspace --locked
python tools/package-prototype.py
```

The script refuses to overwrite an existing bundle. It includes CLI/GUI binaries,
documentation, crate/toolchain licenses, dependency metadata and SHA-256 hashes.
It neither signs nor publishes the bundle. The first supported public release
remains gated on native lifecycle acceptance and signing.
