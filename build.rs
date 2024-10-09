extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to rerun the build script if the wrapper or the C source changes
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=mosfhet/src/");

    // Compile the C library
    cc::Build::new()
        .include("mosfhet/include")
        .files([
            "mosfhet/src/bootstrap.c",
            "mosfhet/src/bootstrap_ga.c",
            "mosfhet/src/keyswitch.c",
            "mosfhet/src/misc.c",
            "mosfhet/src/polynomial.c",
            "mosfhet/src/register.c",
            "mosfhet/src/tlwe.c",
            "mosfhet/src/trgsw.c",
            "mosfhet/src/trlwe.c",
            "mosfhet/src/trlwe_compressed.c",
        ])
        .flag_if_supported("-mrdrnd")
        .flag_if_supported("-mavx")    
        .flag_if_supported("-mavx2")
        .flag_if_supported("-Wno-sign-compare")
        .compile("mosfhet");

    // Generate bindings with bindgen
    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .allowlist_function(".*") 
        .allowlist_type(".*")      
        .clang_arg("-Imosfhet/include")  
        .generate()
        .expect("Unable to generate bindings");

    // Write the bindings to the $OUT_DIR/bindings.rs file.
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
