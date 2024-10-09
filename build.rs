extern crate bindgen;
extern crate cc;

use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=mosfhet/src/");

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

    let bindings = bindgen::Builder::default()
        .header("wrapper.h")
        .allowlist_function(".*") 
        .allowlist_type(".*")      
        .clang_arg("-Imosfhet/include")  
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
