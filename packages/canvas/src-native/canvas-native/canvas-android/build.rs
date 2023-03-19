use std::path::Path;

fn main() {
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/jni_compat/mod.rs");
    println!("cargo:rerun-if-changed=src/jni_compat/org_nativescript_canvas_NSCCanvas.rs");
    println!("cargo:rerun-if-changed=src/jni_compat/org_nativescript_canvas_NSCCanvasRenderingContext2D.rs");
    println!("cargo:rerun-if-changed=src/jni_compat/org_nativescript_canvas_NSCImageAsset.rs");
}
