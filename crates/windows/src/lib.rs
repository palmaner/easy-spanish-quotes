#[cfg(windows)]
mod native;
#[cfg(windows)]
pub use native::*;

#[cfg(not(windows))]
pub fn inspect() -> Result<serde_json::Value, String> {
    Err("Este prototipo requiere Windows; macOS y Linux están pendientes.".into())
}
