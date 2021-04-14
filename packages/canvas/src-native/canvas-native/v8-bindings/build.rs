use std::borrow::Borrow;
use std::{fmt, env};
use std::fmt::{Display, Formatter};

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
            Result::Ok(())
        }
    }
}

const ROOT_DIR: &str = "./vendor/android/";

fn get_path(path: &str)-> String{
    format!("{}{}",ROOT_DIR, path)
}

pub fn env_var(name: impl AsRef<str>) -> Option<String> {
    let name = name.as_ref();
    env::var(name).ok()
}


fn main() {
    let target_str = std::env::var("TARGET").unwrap();
    let mut include_libs_dir = String::from("");
    let arm_libs_dir = get_path("armeabi-v7a");
    let arm_64_libs_dir = get_path("arm64-v8a");
    let x86_libs_dir = get_path("x86");
    let x86_64_libs_dir = get_path("x86_64");
    let ndk = env_var("ANDROID_NDK").expect("ANDROID_NDK variable not set");
    let mut target_ndk = "";
    let arm_ndk = "/tmp/ndk_arm";
    let arm_64_ndk = "/tmp/ndk_arm64";
    let x86_ndk = "/tmp/ndk_x86";
    let x86_64_ndk = "/tmp/ndk_x86_64";

    let target: Vec<String> = target_str.split('-').map(|s| s.into()).collect();
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
            let build_target;
            if target.architecture.eq("armv7") {
                target_ndk = arm_ndk;
                build_target = "armv7-linux-androideabi";
                include_libs_dir.push_str(&arm_libs_dir);
            } else if target.architecture.eq("aarch64") {
                build_target = "aarch64-linux-android";
                target_ndk = arm_64_ndk;
                include_libs_dir.push_str(&arm_64_libs_dir);
            } else if target.architecture.eq("i686") {
                build_target = "i686-linux-android";
                target_ndk = x86_ndk;
                include_libs_dir.push_str(&x86_libs_dir);
            } else if target.architecture.eq("x86_64") {
                build_target = "x86_64-linux-android";
                target_ndk = x86_64_ndk;
                include_libs_dir.push_str(&x86_64_libs_dir);
            } else {
                return;
            }

            println!("target {:?}", build_target);
            let root_include = format!("--sysroot={}/sysroot",ndk);
            let cpu_features = format!("-I{}/sources/android/cpufeatures", ndk);
            let c_lang = format!("-isystem{}/sources/cxx-stl/llvm-libc++/include", ndk);

            //println!("cargo:rustc-link-search=native={}", include_dir);
           // println!("cargo:rustc-link-search=native={}", cpu_features);
            //println!("cargo:rustc-link-search=native={}", c_lang);
            println!("cargo:rustc-link-search=native={}", include_libs_dir);
            println!("cargo:rustc-link-lib=v8"); // the "-l" flag
            println!("cargo:rustc-link-lib=zip"); // the "-l" flag

            let bindings = bindgen::Builder::default()
                .header("wrapper.h")
                //.clang_arg(format!("--target={}", build_target))
                .clang_arg("-x")
                .clang_arg("c++")
                .clang_arg("-std=c++17")
                .clang_arg(root_include)
                .clang_arg(c_lang)

                //.clang_arg(cpu_features)
               // .clang_arg(include_dir)
                //.clang_arg(include_libs_dir)
                // Finish the builder and generate the bindings.
                .generate()
                // Unwrap the Result and panic on failure.
                .expect("Unable to generate bindings");

            // Write the bindings to the $OUT_DIR/bindings.rs file.

            let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
            bindings
                .write_to_file(out_path.join("bindings.rs"))
                .expect("Couldn't write bindings!");

        }
        _ => {}
    }
}