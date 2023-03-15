extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rustc-link-lib=pigpio");

    println!("cargo:rerun-if-changed=wrapper.h");

    if let Ok(libdir_path) = env::var("LIBPIGPIO_LIB_PATH") {
        println!("cargo:rustc-link-search={}", libdir_path);
    }

    // Path to directories of C header
    let libclang_include_dirs: Vec<PathBuf> = env::var("LIBCLANG_INCLUDE_PATH")
        .map(|path| vec![PathBuf::from(path)])
        .unwrap_or_default();

    let libgpio_include_dirs = env::var("LIBPIGPIO_INCLUDE_PATH")
        .map(|path| vec![PathBuf::from(path)])
        .unwrap_or_default();

    let include_args: Vec<_> = libclang_include_dirs
        .iter()
        .chain(libgpio_include_dirs.iter())
        .flat_map(|path| vec!["-I", path.to_str().unwrap()])
        .collect();
    println!("cargo:warning={:?}", include_args);

    let target_args = vec!["-target".into(), env::var("TARGET").unwrap()];
    println!("cargo:warning={:?}", target_args);

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .clang_args(&target_args)
        .clang_args(&include_args)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
