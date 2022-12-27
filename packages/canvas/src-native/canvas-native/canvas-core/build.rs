use std::{env, fmt};
use std::borrow::Borrow;
use std::fmt::{Display, Formatter};
use std::fs::File;
use std::io::Read;
use std::path::{Path, PathBuf};
use regex::Regex;

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
            Ok(())
        }
    }
}

pub fn ndk() -> String {
    env::var("ANDROID_NDK").expect("ANDROID_NDK variable not set")
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
    let target_str = env::var("TARGET").unwrap();
    //let mut include_dir = String::from("-I");
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

    match target.system.borrow() {
        "android" | "androideabi" => {
            // let build_target;
            // let target_version;

            let mut bindings = bindgen::Builder::default();


            let ndk = ndk();
            let major = ndk_major_version(Path::new(&ndk));
            // Version 22 is the first version that moved sysroot to toolchains folder
            if major < 22 {
                bindings = bindings.clang_args([
                    // sysroot is just in the ndk directory
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

            // if target.architecture.eq("armv7") {
            //     build_target = "arm-linux-androideabi";
            //     target_version = "19";
            // } else if target.architecture.eq("aarch64") {
            //     build_target = "aarch64-linux-android";
            //     target_version = "21";
            // } else if target.architecture.eq("i686") {
            //     build_target = "i686-linux-android";
            //     target_version = "19";
            // } else if target.architecture.eq("x86_64") {
            //     build_target = "x86_64-linux-android";
            //     target_version = "21";
            // } else {
            //     return;
            // }


          //  println!("cargo:rustc-link-search=native={}", format!("{:}/sysroot/usr/lib/{}", include_dir, build_target));
           // println!("cargo:rustc-link-search=native={}", format!("{:}/sysroot/usr/lib/{}/{}", include_dir, build_target, target_version));
            // println!("cargo:rustc-link-lib=jnigraphics"); // the "-l" flag
            println!("cargo:rustc-link-lib=android"); // the "-l" flag
            println!("cargo:rustc-link-lib=c++_shared");

            // the resulting bindings.
            let bindings = bindings
                // The input header we would like to generate
                // bindings for.
                .header("wrapper.h")
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
}
