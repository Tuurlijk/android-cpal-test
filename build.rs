use std::{env, path::{Path, PathBuf}};

fn main() {
    if env::var("CARGO_CFG_TARGET_OS").unwrap() == "android" {
        android();
    }
    uniffi::generate_scaffolding("src/androidcpaltest.udl").unwrap();
}

fn android() {
    println!("cargo:rustc-link-lib=c++_shared");

    if let Ok(output_path) = env::var("CARGO_NDK_OUTPUT_PATH") {
        println!("ndk output: {:?}", output_path);
        let sysroot_libs_path = PathBuf::from(env::var_os("CARGO_NDK_SYSROOT_LIBS_PATH").unwrap());
        println!("sysroot_libs_path: {:?}", sysroot_libs_path);
        let lib_path = sysroot_libs_path.join("libc++_shared.so");
        println!("lib path: {:?}", lib_path.as_path());
        let out_path = Path::new(&output_path)
            .join(&env::var("CARGO_NDK_ANDROID_TARGET").unwrap());
        std::fs::create_dir_all(out_path.clone()).unwrap();
        println!("out path: {:?}", out_path);
        std::fs::copy(
            lib_path.as_path(),
            out_path.join("libc++_shared.so"),
        ).expect("Failed to copy file");
    }
}
