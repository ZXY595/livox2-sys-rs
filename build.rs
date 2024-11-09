use std::env;
use std::path::PathBuf;
use cmake;

fn main() {

    // Run bindgen
    let bindings = bindgen::Builder::default()
        .header("src/livox2_include.hpp")
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

    // Build the Livox SDK
    let dst = cmake::Config::new("src/Livox-SDK2")
        .build_target("livox_lidar_sdk_static")
        .build();
    println!("cargo:rustc-link-search={}/build/sdk_core", dst.display());
    println!("cargo:rustc-link-lib=static=livox_lidar_sdk_static");

    println!("cargo:rustc-link-lib=dylib=stdc++");
}