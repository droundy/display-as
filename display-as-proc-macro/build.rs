use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rustc-check-cfg=cfg(span_local_file)");
    // `proc_macro::Span::local_file` was stabilized in Rust 1.88.
    if rustc_minor_version().map_or(false, |minor| minor >= 88) {
        println!("cargo:rustc-cfg=span_local_file");
    }
}

fn rustc_minor_version() -> Option<u32> {
    let rustc = std::env::var_os("RUSTC")?;
    let output = Command::new(rustc).arg("--version").output().ok()?;
    let version = String::from_utf8(output.stdout).ok()?;
    // "rustc 1.88.0 (6b00bc388 2025-06-23)"
    let mut parts = version.split_whitespace().nth(1)?.split('.');
    if parts.next()? != "1" {
        return None;
    }
    let minor = parts.next()?.parse().ok()?;
    // Nightlies of 1.88 predate the stabilization, so they do not count.
    let prerelease = version.contains("-nightly") || version.contains("-dev");
    if minor == 88 && prerelease {
        return None;
    }
    Some(minor)
}
