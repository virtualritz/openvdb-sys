// build.rs
extern crate bindgen;

use std::{env, path::PathBuf};
use std::process::Command;


fn main() {
    // The crate major.minor version should track the
    // OpenVDB lib version
    let openvdb_version = "7.0.0";

    // FIXME make this generic & work on Linux, macOS & Windows
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    let current_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Check out the tag we're tracking with our crate version
    Command::new("git")
        .arg(format!("checkout tags/v{}", openvdb_version))
        .current_dir(current_dir)
        .output()
        .expect(format!("Could not check out OpenVDB v{}.", openvdb_version).as_str());

    // Build OpenVDB
    let openvdb_path = PathBuf::from(cmake::build("openvdb"));

    // Generate Rust bindings
    let openvdb_bindings = bindgen::Builder::default()
        .header("wrapper.hpp")

        //#[cfg(target_os = "macos")]
        //.opaque_type("std::.*")
        //.whitelist_type("arrow::.*")
        //.enable_cxx_namespaces()
        //.clang_arg("-xc++") // change to cpp mode
        .clang_arg("-std=c++11")
        .clang_arg(format!("-I/{}", openvdb_path.join("include").display()))
        .clang_arg("-I/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/CoreFoundation.framework/Versions/A/Headers/")
        .clang_arg("-I/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/CoreServices.framework/Versions/A/Headers/")
        .clang_arg("-I/Library/Developer/CommandLineTools/usr/include/c++/v1/")
        .clang_arg("-I/Applications/Xcode.app/Contents/Developer/Platforms/MacOSX.platform/Developer/SDKs/MacOSX.sdk/usr/include/")
        .clang_arg("-F/Library/Developer/CommandLineTools/SDKs/MacOSX.sdk/System/Library/Frameworks/")

        .generate()
        .expect("Unable to generate OpenVDB bindings.");

    // Write the bindings to the $OUT_DIR/openvdb_bindings.rs file.
    openvdb_bindings
        .write_to_file(out_path.join("openvdb_bindings.rs"))
        .expect("Couldn't write OpenVDB bindings.");

    // Emit linker settings
    println!("cargo:rustc-link-search={}", openvdb_path.join("lib").display());
    println!("cargo:rustc-link-lib=openvdb");
}
