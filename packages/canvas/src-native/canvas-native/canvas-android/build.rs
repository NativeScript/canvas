fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/lib.rs");

    // println!("cargo:rustc-link-lib=EGL"); // the "-l" flag
    // println!("cargo:rustc-link-lib=GLESv2"); // the "-l" flag
    // println!("cargo:rustc-link-lib=GLESv3"); // the "-l" flag
    // println!("cargo:rustc-link-lib=jnigraphics"); // the "-l" flag
    // println!("cargo:rustc-link-lib=android"); // the "-l" flag

    let _ = cxx_build::bridges(["src/lib.rs"]);
}
