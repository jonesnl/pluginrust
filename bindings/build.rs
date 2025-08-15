fn main() {
    #[cfg(target_os = "windows")]
    windows();
}

#[cfg(target_os = "windows")]
fn windows() {
    println!("cargo::rerun-if-changed=build.rs");
    println!("cargo::rerun-if-changed=.windows\\pluginauthenticator.winmd");

    let out_dir = std::env::var("OUT_DIR").expect("OUT_DIR not set");

    let output_file = format!("{}\\windows_plugin_authenticator_bindings.rs", out_dir);
    let args: [&str; 8] = [
        "--in",
        "default",
        ".windows/pluginauthenticator.winmd",
        "--filter",
        "Windows.PluggablePasskeys",
        "--no-allow",
        "--out",
        &output_file,
    ];

    windows_bindgen::bindgen(args).unwrap();
}