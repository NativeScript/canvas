use std::borrow::Borrow;
use std::fmt::{Display, Formatter};
use std::path::PathBuf;
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

    dbg!("system target {:?}", &target.system);
    match target.system.borrow() {
        "android" | "androideabi" => {
            let build_target;
            include_dir.push_str(&ndk());
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
            println!("cargo:rustc-link-search=native={}", include_dir);
            println!("cargo:rustc-link-lib=jnigraphics"); // the "-l" flag
            println!("cargo:rustc-link-lib=android"); // the "-l" flag
                                                      // the resulting bindings.
            let bindings = bindgen::Builder::default()
                // The input header we would like to generate
                // bindings for.
                .header("wrapper.h")
                .clang_arg(include_dir)
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
