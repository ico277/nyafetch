extern crate bindgen;
extern crate cc;
extern crate reqwest;

use reqwest::blocking;
use std::collections::HashMap;
use std::env;
use std::fs;
use std::fs::File;
use std::i64;
use std::io::Write;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever any of the
    // c files changed.
    println!("cargo:rerun-if-changed=src/c/get_gpu.c.gen");

    // Download PCI IDs
    let mut vendors = HashMap::new();
    let body: String = blocking::get("https://pci-ids.ucw.cz/v2.2/pci.ids")
        .expect("Unable to download pci.ids")
        .text()
        .expect("Unable to parse pci.ids as text");
    // Parse PCI IDs
    for line in body.lines() {
        if !line.starts_with("\t") {
            match line.split_once("  ") {
                Some((id, name)) => {
                    let id = match id.split_once(" ") {
                        Some((key, _)) => key,
                        None => match id.split_once("\t") {
                            Some((key, _)) => key,
                            None => id.trim(),
                        },
                    };
                    let id = match i64::from_str_radix(id, 16) {
                        Ok(_) => id,
                        Err(_) => continue,
                    };
                    let name = name.trim().replace("\"", "\\\"");
                    vendors.insert(id.to_string(), name.to_string());
                }
                _ => continue,
            };
        };
    }
    // Generate PCI IDs
    let mut c_file = fs::read_to_string("src/c/get_gpu_gen.c").unwrap();
    let mut generated = String::new();
    for key in vendors.keys() {
        generated += format!("    case 0x{}:\n", key).as_ref();
        generated += format!("        return \"{}\";\n", vendors.get(key).unwrap()).as_ref();
    }
    c_file = c_file.replace("//%REPLACE%", generated.as_ref());
    let mut file = File::create("src/c/get_gpu.c").expect("Unable to create file src/c/get_gpu.c");
    write!(&mut file, "{}", c_file).expect("Unable to write to file src/c/get_gpu.c");

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
