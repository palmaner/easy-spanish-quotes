use std::{env, fs, path::PathBuf};
fn main() {
    let source = PathBuf::from("../../build/layout/esq-layout.dll");
    println!("cargo:rerun-if-changed={}", source.display());
    let out = PathBuf::from(env::var_os("OUT_DIR").unwrap());
    let data = fs::read(source).unwrap_or_default();
    fs::write(out.join("layout.dll"), data).unwrap();
}
