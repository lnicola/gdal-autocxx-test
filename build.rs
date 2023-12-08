fn main() -> miette::Result<()> {
    let path = std::path::PathBuf::from("src"); // include path
    let mut b = autocxx_build::Builder::new("src/lib.rs", &[&path])
        // .auto_allowlist(true)
        // .extra_clang_args(&["-Dprotected=public"])
        .build()?;
    // This assumes all your C++ bindings are in main.rs
    b.flag_if_supported("-std=c++14")
        .compile("autocxx-gdal-test"); // arbitrary library name, pick anything
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/utils.h");

    // Add instructions to link to any C++ libraries you need.
    println!("cargo:rustc-link-lib=dylib=gdal");
    // println!("cargo:rustc-link-search=.");

    Ok(())
}
