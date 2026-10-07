#[cfg(windows)]
mod native;
#[cfg(windows)]
pub use native::*;
#[cfg(windows)]
mod lifecycle;
#[cfg(windows)]
pub use lifecycle::*;
#[cfg(windows)]
mod gui;
#[cfg(windows)]
pub use gui::gui;

#[cfg(not(windows))]
pub fn inspect() -> Result<serde_json::Value, String> {
    Err("Este prototipo requiere Windows; macOS y Linux están pendientes.".into())
}
