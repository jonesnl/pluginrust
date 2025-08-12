mod bindings;

#[test]
fn bindgen() {
    let args = [
        "--in",
        "default",
        ".windows/pluginauthenticator.winmd",
        "--filter",
        "Windows.PluggablePasskeys",
        "--out",
        "src/bindings.rs",
    ];

    windows_bindgen::bindgen(args).unwrap();
}


fn main() {
    println!("Hello, world!");
}
