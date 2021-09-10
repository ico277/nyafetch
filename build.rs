extern crate bindgen;
extern crate cc;

use std::env;
use std::fs;
use std::path::PathBuf;

fn main() {
    let paths = fs::read_dir("./distro_art/").unwrap();

    for path in paths {
        let path = &path.unwrap().path();
        let file_name = path.to_str().unwrap().split("/").collect::<Vec<&str>>();
        let file_name = file_name[file_name.len() - 1];
        fs::create_dir_all("/usr/local/share/nyafetch/").unwrap();
        let file_name = format!("/usr/local/share/nyafetch/{}", file_name);
        match fs::copy(&path, &file_name) {
            Ok(_) => {
                println!("Copied '{}' to '{}'", &path.to_str().unwrap(), &file_name);
            }
            Err(err) => {
                println!(
                    "cargo:warning=There was an error whilst copying the distro art files: {}",
                    err
                );
            }
        }
    }

    println!("cargo:rustc-link-lib=pci");
    cc::Build::new()
        .file("src/c/get_gpu.c")
        .compile("libget_gpu.so");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("wrapper.h")
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
