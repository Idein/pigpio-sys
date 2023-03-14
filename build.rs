extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=pigpio");

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=wrapper.h");

    // Path to directories of C header
    let include_dirs: Vec<PathBuf> = env::var("LIBCLANG_INCLUDE_PATH")
        .map(|path| vec![PathBuf::from(path)])
        .unwrap_or_default();

    let include_args: Vec<_> = include_dirs
        .iter()
        .flat_map(|path| vec!["-I", path.to_str().unwrap()])
        .collect();
    println!("cargo:warning={:?}", include_args);

    let target_args = vec!["-target".into(), env::var("TARGET").unwrap()];
    println!("cargo:warning={:?}", target_args);

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
        .clang_args(&target_args)
        .clang_args(&include_args)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
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
