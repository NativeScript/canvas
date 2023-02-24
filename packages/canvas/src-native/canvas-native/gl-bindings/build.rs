use std::env;
use std::fs::File;
use std::path::PathBuf;

use cfg_aliases::cfg_aliases;
use gl_generator::{Api, Fallbacks, GlobalGenerator, Profile, Registry};

fn main() {
    // XXX this is taken from glutin/build.rs.

    // Setup alias to reduce `cfg` boilerplate.
    cfg_aliases! {
        // Systems.
        android: { target_os = "android" },
        wasm: { target_arch = "wasm32" },
        macos: { target_os = "macos" },
        ios: { target_os = "ios" },
        apple: { any(target_os = "ios", target_os = "macos") },
        free_unix: { all(unix, not(apple), not(android)) },

        // Native displays.
        x11_platform: { all(feature = "x11", free_unix, not(wasm)) },
        wayland_platform: { all(feature = "wayland", free_unix, not(wasm)) },

        // Backends.
        egl_backend: { all(feature = "egl", any(windows, unix), not(apple), not(wasm)) },
        glx_backend: { all(feature = "glx", x11_platform, not(wasm)) },
        wgl_backend: { all(feature = "wgl", windows, not(wasm)) },
        cgl_backend: { all(macos, not(wasm)) },
    }

    let dest = PathBuf::from(&env::var("OUT_DIR").unwrap());

    println!("cargo:rerun-if-changed=build.rs");

    let mut file = File::create(dest.join("gl_bindings.rs")).unwrap();
    Registry::new(Api::Gles2, (4, 0), Profile::Core, Fallbacks::All, [
        "GL_AMD_compressed_ATC_texture",
        "GL_OES_compressed_ETC1_RGB8_texture",
        "GL_EXT_disjoint_timer_query",
        "GL_EXT_sRGB",
        "GL_OES_compressed_ETC1_RGB8_texture",
        "GL_EXT_texture_compression_s3tc",
        "GL_EXT_texture_compression_s3tc_srgb",
        "GL_IMG_texture_compression_pvrtc"
    ])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();
}