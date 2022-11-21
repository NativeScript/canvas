use std::borrow::Borrow;
use std::fmt;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::Path;
use regex::Regex;

const _IOS_SRC_BINDINGS_RS: &str = "src/bindings.rs";
const _ANDROID_SRC_BINDINGS_RS: &str = "src/bindings.rs";

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
            self.abi.as_deref(),
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
            Ok(())
        }
    }
}

pub fn ndk() -> String {
    std::env::var("ANDROID_NDK").expect("ANDROID_NDK variable not set")
}

fn host_tag() -> String {
    // Because this is part of build.rs, the target_os is actually the host system
    if cfg!(target_os = "windows") {
        "windows-x86_64"
    } else if cfg!(target_os = "linux") {
        "linux-x86_64"
    } else if cfg!(target_os = "macos") {
        "darwin-x86_64"
    } else {
        panic!("host os is not supported")
    }
        .to_string()
}

/// Get NDK major version from source.properties
fn ndk_major_version(ndk_dir: &Path) -> u32 {
    // Capture version from the line with Pkg.Revision
    let re = Regex::new(r"Pkg.Revision = (\d+)\.(\d+)\.(\d+)").unwrap();
    // There's a source.properties file in the ndk directory, which contains
    let mut source_properties =
        File::open(ndk_dir.join("source.properties")).expect("Couldn't open source.properties");
    let mut buf = "".to_string();
    source_properties
        .read_to_string(&mut buf)
        .expect("Could not read source.properties");
    // Capture version info
    let captures = re
        .captures(&buf)
        .expect("source.properties did not match the regex");
    // Capture 0 is the whole line of text
    captures[1].parse().expect("could not parse major version")
}

fn main() {
    let target_str = std::env::var("TARGET").unwrap();
    //let mut include_dir = String::from("-I");
    let target: Vec<String> = target_str.split('-').map(|s| s.into()).collect();
    if target.len() < 3 {
        assert!(!(target.len() < 3), "Failed to parse TARGET {}", target_str);
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

    match target.system.borrow() {
        "android" | "androideabi" => {
            // println!("cargo:rustc-link-lib=jnigraphics"); // the "-l" flag
            //let build_target;

            let mut bindings = bindgen::Builder::default();


            let ndk = ndk();
            let major = ndk_major_version(Path::new(&ndk));
            // Version 22 is the first version that moved sysroot to toolchains folder
            if major < 22 {
                // sysroot is just in the ndk directory
                bindings = bindings.clang_args([
                    &format!("--sysroot={}/sysroot", ndk),

                    // note: Adding C++ includes messes with Apple's CLang 11 in the binding generator,
                    // Which means that only we support the official LLVM versions for Android builds on macOS.

                    &format!(
                        "-isystem{}/sources/cxx-stl/llvm-libc++/include",
                        ndk
                    )
                ]);
            } else {
                // NDK versions >= 22 have the sysroot in the llvm prebuilt by
                let host_toolchain = format!("{}/toolchains/llvm/prebuilt/{}", ndk, host_tag());
                // sysroot is stored in the prebuilt llvm, under the host
                bindings = bindings.clang_arg(&format!("--sysroot={}/sysroot", host_toolchain));
            };


            // println!("cargo:rustc-link-search=native={}", include_dir);


            // if target.architecture.eq("armv7") {
            //     build_target = "armv7-linux-androideabi";
            // } else if target.architecture.eq("aarch64") {
            //     build_target = "aarch64-linux-android";
            //     println!("cargo:rustc-link-lib=GLESv3"); // the "-l" flag
            // } else if target.architecture.eq("i686") {
            //     build_target = "i686-linux-android";
            // } else if target.architecture.eq("x86_64") {
            //     build_target = "x86_64-linux-android";
            //     println!("cargo:rustc-link-lib=GLESv3"); // the "-l" flag
            // } else {
            //     return;
            // }

            // println!("cargo:rustc-link-lib=jnigraphics"); // the "-l" flag
            //  println!("cargo:rustc-link-lib=EGL"); // the "-l" flag
            //  println!("cargo:rustc-link-lib=GLESv2"); // the "-l" flag
           // println!("cargo:rustc-link-lib=GLESv3");
           // println!("cargo:rustc-link-lib=c++_shared");

            // The bindgen::Builder is the main entry point
            // to bindgen, and lets you build up options for
            // the resulting bindings.
            let bindings = bindings
                // The input header we would like to generate
                // bindings for.
                .header("wrapper.h")
              //  .clang_arg(include_dir)
                // Finish the builder and generate the bindings.
                .generate()
                // Unwrap the Result and panic on failure.
                .expect("Unable to generate bindings");

            // Write the bindings to the $OUT_DIR/bindings.rs file.

            let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
            bindings
                .write_to_file(out_path.join("bindings.rs"))
                .expect("Couldn't write bindings!");

            // fs::copy(out_path.join("android_bindings.rs"), ANDROID_SRC_BINDINGS_RS).expect("Couldn't copy bindings!");;;
        }
        "ios" | "darwin" => {
            let target = std::env::var("TARGET").unwrap();
            let directory = sdk_path(&target).ok();
            build(directory.as_ref().map(String::as_ref), &target);
        }
        _ => {}
    }
}

fn sdk_path(target: &str) -> Result<String, std::io::Error> {
    use std::process::Command;
    let sdk = if target.contains("apple-darwin")
        || target == "aarch64-apple-ios-macabi"
        || target == "x86_64-apple-ios-macabi" {
        "macosx"
    } else if target == "x86_64-apple-ios" || target == "i386-apple-ios" || target == "aarch64-apple-ios-sim" {
        "iphonesimulator"
    } else if target == "aarch64-apple-ios"
        || target == "armv7-apple-ios"
        || target == "armv7s-apple-ios"
    {
        "iphoneos"
    } else {
        unreachable!();
    };

    let output = Command::new("xcrun")
        .args(&["--sdk", sdk, "--show-sdk-path"])
        .output()?
        .stdout;
    let prefix_str = std::str::from_utf8(&output).expect("invalid output from `xcrun`");
    Ok(prefix_str.trim_end().to_string())
}

fn build(sdk_path: Option<&str>, target: &str) {
    // Generate one large set of bindings for all frameworks.
    //
    // We do this rather than generating a module per framework as some frameworks depend on other
    // frameworks and in turn share types. To ensure all types are compatible across each
    // framework, we feed all headers to bindgen at once.
    //
    // Only link to each framework and include their headers if their features are enabled and they
    // are available on the target os.

    use std::env;
    use std::path::PathBuf;

    let mut headers: Vec<&str> = vec![];


    if target.contains("apple-ios") && !target.contains("macabi") {
        println!("cargo:rustc-link-lib=framework=GLKit");
        println!("cargo:rustc-link-lib=framework=OpenGLES");
        headers.push("OpenGLES/ES2/gl.h");
        headers.push("OpenGLES/ES2/glext.h");
        headers.push("OpenGLES/ES3/gl.h");
        headers.push("OpenGLES/ES3/glext.h");
        headers.push("OpenGLES/EAGL.h");
    } else {
        println!("cargo:rustc-link-lib=framework=GLKit");
        println!("cargo:rustc-link-lib=framework=OpenGL");
        headers.push("OpenGL/gl.h");
        headers.push("OpenGL/glext.h");
        headers.push("OpenGL/gl3.h");
        headers.push("OpenGL/gl3ext.h");
        headers.push("OpenGL/OpenGL.h");
    }


    println!("cargo:rerun-if-env-changed=BINDGEN_EXTRA_CLANG_ARGS");
    // Begin building the bindgen params.
    let mut builder = bindgen::Builder::default();

    builder = builder.clang_args(&["-x", "objective-c", "-fblocks"]);
    // builder = builder.objc_extern_crate(false);
    //  builder = builder.block_extern_crate(false);
    //  builder = builder.generate_block(true);
    builder = builder.rustfmt_bindings(true);
    // See https://github.com/rust-lang/rust-bindgen/issues/1211
    // Technically according to the llvm mailing list, the argument to clang here should be
    // -arch arm64 but it looks cleaner to just change the target.
    let target = if target == "aarch64-apple-ios" {
        "arm64-apple-ios"
    } else {
        target
    };

    builder = builder.clang_args(&[&format!("--target={}", target)]);

    if let Some(sdk_path) = sdk_path {
        builder = builder.clang_args(&["-isysroot", sdk_path]);
    }
    if target.contains("apple-ios") {
        // builder = builder.clang_args(&["-x", "objective-c", "-fblocks"]);
        builder = builder
            .clang_args(&["-x", "objective-c", "-fblocks"])
            .objc_extern_crate(true)
            .block_extern_crate(true)
            .allowlist_function("gl.*")
            .allowlist_recursively(true)
            .allowlist_var("GL_.*")
            .allowlist_type("GL_.*");
    }

    let meta_header: Vec<_> = headers
        .iter()
        .map(|h| format!("#include <{}>\n", h))
        .collect();

    builder = builder.header_contents("GL.h", &meta_header.concat());

    // Generate the bindings.
    builder = builder.trust_clang_mangling(false).derive_default(true);

    let bindings = builder.generate().expect("unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
