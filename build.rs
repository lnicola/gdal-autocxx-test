fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("src"); // include path
    let mut b = autocxx_build::Builder::new("src/lib.rs", &[&path]).build()?;
    // This assumes all your C++ bindings are in main.rs
    b.flag_if_supported("-std=c++14").compile("autocxx-demo"); // arbitrary library name, pick anything
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/utils.h");

    // Add instructions to link to any C++ libraries you need.
    println!("cargo:rustc-link-lib=dylib=gdal");

    Ok(())
}
