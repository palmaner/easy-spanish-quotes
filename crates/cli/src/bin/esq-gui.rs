#![cfg_attr(windows, windows_subsystem = "windows")]
// Reuse the CLI dispatcher for bounded elevated operations, without a console
// window during ordinary GUI startup.
#[path = "../main.rs"]
mod application;
fn main() {
    application::main();
}
