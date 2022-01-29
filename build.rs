extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever any of the
    // c files changed.
    println!("cargo:rerun-if-changed=src/c/get_gpu.c");

    // Compile get_cpu.c
    cc::Build::new()
        .file("src/c/get_gpu.c")
        .compile("libget_gpu.so");
    
    // Link libs
    println!("cargo:rustc-link-lib=pci");
    println!("cargo:rustc-link-lib=libget_gpu.so");

    let bindings = bindgen::Builder::default()
        // The input header
        .header("wrapper.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // header files changed.
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
