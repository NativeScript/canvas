use std::borrow::Borrow;
use std::fmt;
use std::fmt::{Display, Formatter};

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
            Result::Ok(())
        }
    }
}

pub fn ndk() -> String {
    std::env::var("ANDROID_NDK").expect("ANDROID_NDK variable not set")
}

fn main() {
    let target_str = std::env::var("TARGET").unwrap();
    let mut include_dir = String::from("-I");
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

    println!("system {:?}", &target.system);
    println!("target {:?}", &target);
    match target.system.borrow() {
        "android" | "androideabi" => {
            // println!("cargo:rustc-link-lib=jnigraphics"); // the "-l" flag
            let build_target;
            include_dir.push_str(&ndk());
            // after moving to newer ndk
           // include_dir.push_str("/toolchains/llvm/prebuilt/darwin-x86_64");


           include_dir.push_str("/sysroot/usr/include");
            //println!("cargo:rustc-link-search=native={}", include_dir);

            if target.architecture.eq("armv7") {
                build_target = "armv7-linux-androideabi";
            } else if target.architecture.eq("aarch64") {
                build_target = "aarch64-linux-android";
                println!("cargo:rustc-link-lib=GLESv3"); // the "-l" flag
            } else if target.architecture.eq("i686") {
                build_target = "i686-linux-android";
            } else if target.architecture.eq("x86_64") {
                build_target = "x86_64-linux-android";
                println!("cargo:rustc-link-lib=GLESv3"); // the "-l" flag
            } else {
                return;
            }

            println!("target {:?}", build_target);
            println!("cargo:rustc-link-lib=jnigraphics"); // the "-l" flag
            println!("cargo:rustc-link-lib=EGL"); // the "-l" flag
            println!("cargo:rustc-link-lib=GLESv2"); // the "-l" flag
            
                                                     // The bindgen::Builder is the main entry point
                                                     // to bindgen, and lets you build up options for
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

            let out_path = std::path::PathBuf::from(std::env::var("OUT_DIR").unwrap());
            bindings
                .write_to_file(out_path.join("bindings.rs"))
                .expect("Couldn't write bindings!");

            // fs::copy(out_path.join("android_bindings.rs"), ANDROID_SRC_BINDINGS_RS).expect("Couldn't copy bindings!");;;
        }
        "ios" | "darwin" => {
            let target = std::env::var("TARGET").unwrap();
            let directory = sdk_path(&target).ok();
            println!("sdk_path {:?}", directory);
            build(directory.as_ref().map(String::as_ref), &target);
        }
        _ => {}
    }
}

fn sdk_path(target: &str) -> Result<String, std::io::Error> {
    use std::process::Command;
    let sdk = if target.contains("apple-darwin") 
    || target == "aarch64-apple-ios-macabi"
    || target == "x86_64-apple-ios-macabi"  {
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


    if target.contains("apple-ios") &&  !target.contains("macabi"){
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
            .allowlist_type("GL_.*")
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
