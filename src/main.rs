mod bindings;

#[test]
fn bindgen() {
    let args = [
        "--in",
        ".windows/out.winmd",
        "--filter",
        "Windows.PluggablePasskeys",
        "--out",
        "src/bindings.rs",
        "--config",
        "flatten",
    ];

    windows_bindgen::bindgen(args).unwrap();
}


fn main() {
    println!("Hello, world!");
}
