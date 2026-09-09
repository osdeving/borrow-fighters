//! Embeds the Windows process settings used by the game and its native file tests.
//!
//! System: Build boundary. Raylib receives UTF-8 paths from Rust, so its narrow
//! C file APIs must use the same encoding without changing the user's system locale.

fn main() {
    let manifest = "packaging/windows/app.manifest";
    println!("cargo::rerun-if-changed={manifest}");
    if std::env::var("CARGO_CFG_TARGET_OS").as_deref() == Ok("windows")
        && std::env::var("CARGO_CFG_TARGET_ENV").as_deref() == Ok("msvc")
    {
        let path =
            std::path::Path::new(&std::env::var_os("CARGO_MANIFEST_DIR").unwrap()).join(manifest);
        println!("cargo::rustc-link-arg=/MANIFEST:EMBED");
        println!("cargo::rustc-link-arg=/MANIFESTINPUT:{}", path.display());
    }
}
