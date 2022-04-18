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

const CPP_SOURCE: [&str; 4] = [
    "src/Caches.cpp",
    "src/Helpers.cpp",
    "src/OnImageAssetLoadCallbackHolder.cpp",
    "src/ImageAssetImpl.cpp",
];

const CPP_SOURCE_HEADERS: [&str; 4] = [
    "src/Caches.h",
    "src/Helpers.h",
    "src/OnImageAssetLoadCallbackHolder.h",
    "src/ImageAssetImpl.h",
];

const CPP_2D_SOURCE: [&str; 5] = [
    // "src/canvas2d/CanvasGradient.cpp",
    // "src/canvas2d/CanvasPattern.cpp",
    // "src/canvas2d/CanvasRenderingContext2D.cpp",
    "src/canvas2d/ImageDataImpl.cpp",
    "src/canvas2d/Path2D.cpp",
    "src/canvas2d/MatrixImpl.cpp",
    "src/canvas2d/TextMetricsImpl.cpp",
    "src/canvas2d/Canvas2D.cpp",
];

const CPP_2D_SOURCE_HEADERS: [&str; 5] = [
    // "src/canvas2d/CanvasGradient.cpp",
    // "src/canvas2d/CanvasPattern.cpp",
    // "src/canvas2d/CanvasRenderingContext2D.cpp",
    "src/canvas2d/ImageDataImpl.h",
    "src/canvas2d/Path2D.h",
    "src/canvas2d/MatrixImpl.h",
    "src/canvas2d/TextMetricsImpl.h",
    "src/canvas2d/Canvas2D.h",
];

const CPP_WEBGL_SOURCE: [&str; 2] = ["src/webgl/WebGL.cpp", "src/webgl/WebGLRenderingContext.cpp"];

const CPP_WEBGL_SOURCE_HEADERS: [&str; 2] =
    ["src/webgl/WebGL.h", "src/webgl/WebGLRenderingContext.h"];

const CPP_WEBGL2_SOURCE: [&str; 2] = [
    "src/webgl2/WebGL2.cpp",
    "src/webgl2/WebGL2RenderingContext.cpp",
];

const CPP_WEBGL2_SOURCE_HEADERS: [&str; 2] =
    ["src/webgl2/WebGL2.h", "src/webgl2/WebGL2RenderingContext.h"];

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
            println!("cargo:rustc-link-lib=jnigraphics"); // the "-l" flag
            println!("cargo:rustc-link-lib=android"); // the "-l" flag
                                                      // the resulting bindings.
            let bindings = bindgen::Builder::default()
                // The input header we would like to generate
                // bindings for.
                .header("wrapper.h")
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
        // .define("V8_31BIT_SMIS_ON_64BIT_ARCH", None)
        // .define("V8_ENABLE_REGEXP_INTERPRETER_THREADED_DISPATCH", None)
        .define("V8_EMBEDDED_BUILTINS", None)
        .files(&CPP_SOURCE)
        .files(&CPP_2D_SOURCE)
        .files(&CPP_WEBGL_SOURCE)
        .files(&CPP_WEBGL2_SOURCE)
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
