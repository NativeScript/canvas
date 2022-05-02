use std::borrow::Borrow;
use std::fmt::{Display, Formatter};
use std::path::{Path, PathBuf};
use std::{env, fmt};

use bindgen;

#[derive(Clone, Debug)]
pub struct Target {
    pub architecture: String,
    pub vendor: String,
    pub system: String,
    pub abi: Option<String>,
}

impl Target {
    pub fn as_strs(&self) -> (&str, &str, &str, Option<&str>) {
        (
            self.architecture.as_str(),
            self.vendor.as_str(),
            self.system.as_str(),
            self.abi.as_ref().map(|s| s.as_str()),
        )
    }
}

impl Display for Target {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(
            f,
            "{}-{}-{}",
            &self.architecture, &self.vendor, &self.system
        )?;

        if let Some(ref abi) = self.abi {
            write!(f, "-{}", abi)
        } else {
            Result::Ok(())
        }
    }
}

pub fn ndk() -> String {
    std::env::var("ANDROID_NDK").expect("ANDROID_NDK variable not set")
}

const FLAGS_STR: &str = "-std=c++14 -Werror -Wno-unused-result -mstackrealign -fexceptions -fno-builtin-stpcpy -fno-rtti -O3 -fvisibility=hidden -ffunction-sections -fno-data-sections";

const CPP_SOURCE: [&str; 9] = [
    "src/Caches.cpp",
    "src/Helpers.cpp",
    "src/OnImageAssetLoadCallbackHolder.cpp",
    "src/ImageAssetImpl.cpp",
    "src/TextEncoderImpl.cpp",
    "src/TextDecoderImpl.cpp",
    "src/OnRafCallback.cpp",
    "src/RafImpl.cpp",
    "src/ObjectCacheEntry.cpp",
];

const CPP_SOURCE_HEADERS: [&str; 9] = [
    "src/Caches.h",
    "src/Helpers.h",
    "src/OnImageAssetLoadCallbackHolder.h",
    "src/ImageAssetImpl.h",
    "src/TextEncoderImpl.h",
    "src/TextDecoderImpl.h",
    "src/OnRafCallback.h",
    "src/RafImpl.h",
    "src/ObjectCacheEntry.h",
];

const CPP_2D_SOURCE: [&str; 8] = [
    "src/canvas2d/CanvasGradient.cpp",
    "src/canvas2d/CanvasPattern.cpp",
    "src/canvas2d/CanvasRenderingContext2DImpl.cpp",
    "src/canvas2d/ImageDataImpl.cpp",
    "src/canvas2d/Path2D.cpp",
    "src/canvas2d/MatrixImpl.cpp",
    "src/canvas2d/TextMetricsImpl.cpp",
    "src/canvas2d/Canvas2D.cpp",
];

const CPP_2D_SOURCE_HEADERS: [&str; 8] = [
    "src/canvas2d/CanvasGradient.h",
    "src/canvas2d/CanvasPattern.h",
    "src/canvas2d/CanvasRenderingContext2DImpl.h",
    "src/canvas2d/ImageDataImpl.h",
    "src/canvas2d/Path2D.h",
    "src/canvas2d/MatrixImpl.h",
    "src/canvas2d/TextMetricsImpl.h",
    "src/canvas2d/Canvas2D.h",
];

const CPP_WEBGL_EXT_SOURCE: [&str; 24] = [
    "src/webgl/extensions/ANGLE_instanced_arraysImpl.cpp",
    "src/webgl/extensions/EXT_blend_minmaxImpl.cpp",
    "src/webgl/extensions/EXT_color_buffer_half_floatImpl.cpp",
    "src/webgl/extensions/EXT_disjoint_timer_queryImpl.cpp",
    "src/webgl/extensions/EXT_shader_texture_lodImpl.cpp",
    "src/webgl/extensions/EXT_sRGBImpl.cpp",
    "src/webgl/extensions/EXT_texture_filter_anisotropicImpl.cpp",
    "src/webgl/extensions/OES_element_index_uintImpl.cpp",
    "src/webgl/extensions/OES_standard_derivativesImpl.cpp",
    "src/webgl/extensions/OES_texture_float_linearImpl.cpp",
    "src/webgl/extensions/OES_texture_floatImpl.cpp",
    "src/webgl/extensions/OES_texture_half_float_linearImpl.cpp",
    "src/webgl/extensions/OES_texture_half_floatImpl.cpp",
    "src/webgl/extensions/OES_vertex_array_objectImpl.cpp",
    "src/webgl/extensions/WEBGL_color_buffer_floatImpl.cpp",
    "src/webgl/extensions/WEBGL_compressed_texture_atcImpl.cpp",
    "src/webgl/extensions/WEBGL_compressed_texture_etc1Impl.cpp",
    "src/webgl/extensions/WEBGL_compressed_texture_etcImpl.cpp",
    "src/webgl/extensions/WEBGL_compressed_texture_pvrtcImpl.cpp",
    "src/webgl/extensions/WEBGL_compressed_texture_s3tc_srgbImpl.cpp",
    "src/webgl/extensions/WEBGL_compressed_texture_s3tcImpl.cpp",
    "src/webgl/extensions/WEBGL_depth_textureImpl.cpp",
    "src/webgl/extensions/WEBGL_draw_buffersImpl.cpp",
    "src/webgl/extensions/WEBGL_lose_contextImpl.cpp",
];

const CPP_WEBGL_EXT_SOURCE_HEADERS: [&str; 24] = [
    "src/webgl/extensions/ANGLE_instanced_arraysImpl.h",
    "src/webgl/extensions/EXT_blend_minmaxImpl.h",
    "src/webgl/extensions/EXT_color_buffer_half_floatImpl.h",
    "src/webgl/extensions/EXT_disjoint_timer_queryImpl.h",
    "src/webgl/extensions/EXT_shader_texture_lodImpl.h",
    "src/webgl/extensions/EXT_sRGBImpl.h",
    "src/webgl/extensions/EXT_texture_filter_anisotropicImpl.h",
    "src/webgl/extensions/OES_element_index_uintImpl.h",
    "src/webgl/extensions/OES_standard_derivativesImpl.h",
    "src/webgl/extensions/OES_texture_float_linearImpl.h",
    "src/webgl/extensions/OES_texture_floatImpl.h",
    "src/webgl/extensions/OES_texture_half_float_linearImpl.h",
    "src/webgl/extensions/OES_texture_half_floatImpl.h",
    "src/webgl/extensions/OES_vertex_array_objectImpl.h",
    "src/webgl/extensions/WEBGL_color_buffer_floatImpl.h",
    "src/webgl/extensions/WEBGL_compressed_texture_atcImpl.h",
    "src/webgl/extensions/WEBGL_compressed_texture_etc1Impl.h",
    "src/webgl/extensions/WEBGL_compressed_texture_etcImpl.h",
    "src/webgl/extensions/WEBGL_compressed_texture_pvrtcImpl.h",
    "src/webgl/extensions/WEBGL_compressed_texture_s3tc_srgbImpl.h",
    "src/webgl/extensions/WEBGL_compressed_texture_s3tcImpl.h",
    "src/webgl/extensions/WEBGL_depth_textureImpl.h",
    "src/webgl/extensions/WEBGL_draw_buffersImpl.h",
    "src/webgl/extensions/WEBGL_lose_contextImpl.h",
];

const CPP_WEBGL_SOURCE: [&str; 12] = [
    "src/webgl/WebGL.cpp",
    "src/webgl/WebGLRenderingContextBase.cpp",
    "src/webgl/WebGLRenderingContext.cpp",
    "src/webgl/WebGLBuffer.cpp",
    "src/webgl/WebGLFramebuffer.cpp",
    "src/webgl/WebGLProgram.cpp",
    "src/webgl/WebGLRenderbuffer.cpp",
    "src/webgl/WebGLShader.cpp",
    "src/webgl/WebGLTexture.cpp",
    "src/webgl/WebGLUniformLocation.cpp",
    "src/webgl/WebGLShaderPrecisionFormatImpl.cpp",
    "src/webgl/WebGLActiveInfoImpl.cpp",
];

const CPP_WEBGL_SOURCE_HEADERS: [&str; 12] = [
    "src/webgl/WebGL.h",
    "src/webgl/WebGLRenderingContextBase.h",
    "src/webgl/WebGLRenderingContext.h",
    "src/webgl/WebGLBuffer.h",
    "src/webgl/WebGLFramebuffer.h",
    "src/webgl/WebGLProgram.h",
    "src/webgl/WebGLRenderbuffer.h",
    "src/webgl/WebGLShader.h",
    "src/webgl/WebGLTexture.h",
    "src/webgl/WebGLUniformLocation.h",
    "src/webgl/WebGLShaderPrecisionFormatImpl.h",
    "src/webgl/WebGLActiveInfoImpl.h",
];

const CPP_WEBGL2_SOURCE: [&str; 7] = [
    "src/webgl2/WebGL2.cpp",
    "src/webgl2/WebGL2RenderingContext.cpp",
    "src/webgl2/WebGLQuery.cpp",
    "src/webgl2/WebGLSampler.cpp",
    "src/webgl2/WebGLSyncImpl.cpp",
    "src/webgl2/WebGLTransformFeedback.cpp",
    "src/webgl2/WebGLVertexArrayObject.cpp",
];

const CPP_WEBGL2_SOURCE_HEADERS: [&str; 7] = [
    "src/webgl2/WebGL2.h",
    "src/webgl2/WebGL2RenderingContext.h",
    "src/webgl2/WebGLQuery.h",
    "src/webgl2/WebGLSampler.h",
    "src/webgl2/WebGLSyncImpl.h",
    "src/webgl2/WebGLTransformFeedback.h",
    "src/webgl2/WebGLVertexArrayObject.h",
];

fn main() {
    let target_str = env::var("TARGET").unwrap();
    let mut include_dir = String::from("-I");
    let target: Vec<String> = target_str.split("-").map(|s| s.into()).collect();
    if target.len() < 3 {
        panic!("Failed to parse TARGET {}", target_str);
    }

    let abi = if target.len() > 3 {
        Some(target[3].clone())
    } else {
        None
    };

    let target = Target {
        architecture: target[0].clone(),
        vendor: target[1].clone(),
        system: target[2].clone(),
        abi,
    };

    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=src/bridges/context.rs");

    match target.system.borrow() {
        "android" | "androideabi" => {
            let build_target;
            include_dir.push_str(&ndk());
            // after moving to newer ndk
            // include_dir.push_str("/toolchains/llvm/prebuilt/darwin-x86_64");
            if target.architecture.eq("armv7") {
                build_target = "armv7-linux-androideabi";
            } else if target.architecture.eq("aarch64") {
                build_target = "aarch64-linux-android";
            } else if target.architecture.eq("i686") {
                build_target = "i686-linux-android";
            } else if target.architecture.eq("x86_64") {
                build_target = "x86_64-linux-android";
            } else {
                return;
            }

            include_dir.push_str("/sysroot/usr/include");
            //println!("cargo:rustc-link-search={}", include_dir);
            println!("cargo:rustc-link-lib=EGL"); // the "-l" flag
            println!("cargo:rustc-link-lib=GLESv2"); // the "-l" flag
            println!("cargo:rustc-link-lib=GLESv3"); // the "-l" flag
            println!("cargo:rustc-link-lib=jnigraphics"); // the "-l" flag
            println!("cargo:rustc-link-lib=android"); // the "-l" flag
                                                      // the resulting bindings.

            // println!("cargo:rerun-if-changed={}", "wrapper.h");
            let bindings = bindgen::Builder::default()
                // The input header we would like to generate
                // bindings for.
                .header("wrapper.h")
                .clang_arg(&format!("--target={}", target_str))
                .clang_arg(&format!("--sysroot={}/sysroot", ndk()))
                .clang_arg(&format!("-I{}/sources/android/cpufeatures", ndk()))
                .clang_arg(&include_dir)
                // Finish the builder and generate the bindings.
                .generate()
                // Unwrap the Result and panic on failure.
                .expect("Unable to generate bindings");

            // Write the bindings to the $OUT_DIR/bindings.rs file.

            let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
            bindings
                .write_to_file(out_path.join("bindings.rs"))
                .expect("Couldn't write bindings!");
        }
        _ => {}
    }

    let mut dir_tmp = "";

    if target.architecture.eq("armv7") {
        dir_tmp = "vendor/armeabi-v7a";
        println!(
            "cargo:rustc-link-search=native={}",
            &format!(
                "{}{}/armeabi-v7a",
                ndk(),
                "/sources/cxx-stl/llvm-libc++/libs"
            )
        );
        println!("cargo:rustc-link-lib=static=android_support");
    } else if target.architecture.eq("aarch64") {
        dir_tmp = "vendor/arm64-v8a";
    } else if target.architecture.eq("i686") {
        dir_tmp = "vendor/x86";
        println!(
            "cargo:rustc-link-search=native={}",
            &format!("{}{}/x86", ndk(), "/sources/cxx-stl/llvm-libc++/libs")
        );
        println!("cargo:rustc-link-lib=static=android_support");
    } else if target.architecture.eq("x86_64") {
        dir_tmp = "vendor/x86_64";
    }

    let dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap()).join(dir_tmp);

    println!("cargo:rustc-link-search=native={}", dir.to_string_lossy());
    println!("cargo:rustc-link-lib=static=v8"); // the "-l" flag
    println!("cargo:rustc-link-lib=static=zip");
    println!("cargo:rustc-link-arg=-Wl,--allow-multiple-definition");
    println!("cargo:rustc-link-arg=-Wl,-fuse-ld=lld-13");
    // println!("cargo:rustc-link-arg=-Wl,-nostdlib++");
    // println!("cargo:rustc-link-arg=-Wl,-ldl");
    println!("cargo:rustc-link-lib=static=c++abi");
    println!("cargo:rustc-link-lib=static=c++");

    let mut build = cxx_build::bridges(["src/lib.rs", "src/bridges/context.rs"]);

    build
        // .include("include")
        // .include("src")
        // .include("src/canvas2d")
        // .include("src/webgl")
        // .include("src/webgl2")
        .flag("-pthread")
        .cpp_link_stdlib("c++_static")
        .flag_if_supported("-std=c++14")
        .flag(&format!("--target={}", target_str))
        .flag(&format!("--sysroot={}/sysroot", ndk()))
        .flag(&format!("-I{}/sources/android/cpufeatures", ndk()))
        .flag(&format!(
            "-isystem{}/sources/cxx-stl/llvm-libc++/include",
            ndk()
        ))
        .flag(&include_dir)
        .define("_LIBCPP_ABI_UNSTABLE", None)
        // .define("_LIBCPP_ABI_VERSION", "Cr")
        .define("_LIBCPP_ENABLE_NODISCARD", None)
        .define("_LIBCPP_ABI_UNSTABLE", None)
        .define("V8_31BIT_SMIS_ON_64BIT_ARCH", None)
        // .define("V8_ENABLE_REGEXP_INTERPRETER_THREADED_DISPATCH", None)
        .define("V8_EMBEDDED_BUILTINS", None)
        .files(&CPP_SOURCE)
        .files(&CPP_2D_SOURCE)
        .files(&CPP_WEBGL_SOURCE)
        .files(&CPP_WEBGL2_SOURCE)
        .files(&CPP_WEBGL_EXT_SOURCE)
        .file("src/Bridge.cpp");

    build.extra_warnings(false);

    let mut all: Vec<&str> = vec![];
    all.extend(CPP_SOURCE.as_slice());
    all.extend(CPP_2D_SOURCE.as_slice());
    all.extend(CPP_WEBGL_SOURCE.as_slice());
    all.extend(CPP_WEBGL2_SOURCE.as_slice());

    for item in all.into_iter() {
        println!("cargo:rerun-if-changed={}", item);
    }

    let mut all_headers: Vec<&str> = vec![];
    all_headers.extend(CPP_SOURCE_HEADERS.as_slice());
    all_headers.extend(CPP_2D_SOURCE_HEADERS.as_slice());
    all_headers.extend(CPP_WEBGL_SOURCE_HEADERS.as_slice());
    all_headers.extend(CPP_WEBGL2_SOURCE_HEADERS.as_slice());
    all_headers.extend(CPP_WEBGL_EXT_SOURCE.as_slice());
    all_headers.extend(CPP_WEBGL_EXT_SOURCE_HEADERS.as_slice());

    for item in all_headers.into_iter() {
        println!("cargo:rerun-if-changed={}", item);
    }

    if target.architecture.eq("aarch64") || target.architecture.eq("x86_64") {
        build.define("ANDROID_PLATFORM", "android-21");
        build.define("APP_PLATFORM", "android-21");
        build.define("V8_COMPRESS_POINTERS", None);
    }

    if target.architecture.eq("armv7") || target.architecture.eq("x86") {
        build.define("ANDROID_PLATFORM", "android-17");
        build.define("APP_PLATFORM", "android-17");
        build.define("V8_COMPRESS_POINTERS", None);
    }

    let flags: Vec<_> = FLAGS_STR.split(" ").collect();

    for flag in flags.into_iter() {
        build.flag_if_supported(flag);
    }

    build.warnings(false);

    build.compile("androidv8");
}
