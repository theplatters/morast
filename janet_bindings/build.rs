use std::{env, path::PathBuf};

fn main() {
    println!("cargo:rerun-if-changed=lib/janet.c");
    println!("cargo:rerun-if-changed=lib/janet.h");

    // 1) Compile janet.c into a static library
    cc::Build::new()
        .file("lib/janet.c")
        .include("lib")
        // Optional, but often helpful:
        .warnings(false)
        // If you need C99 (Janet is typically fine with this):
        .flag_if_supported("-std=c99")
        .compile("janet"); // produces libjanet.a

    // 2) Generate bindings from janet.h
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());

    let bindings = bindgen::Builder::default()
        .header("lib/janet.h")
        // so bindgen can find includes (if janet.h includes other headers in lib/)
        .clang_arg("-Ilib")
        // Cargo callbacks for rerun-if-changed on included headers (bindgen >= 0.69)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        // Commonly useful to reduce noise; remove if you want everything:
        .allowlist_function("janet_.*")
        .allowlist_type("Janet.*|janet_.*")
        .allowlist_var("JANET_.*|janet_.*")
        .generate()
        .expect("Unable to generate bindings for janet.h");

    bindings
        .write_to_file(out_dir.join("bindings.rs"))
        .expect("Couldn't write bindings.rs");

    // 3) Ensure Cargo links the compiled static lib (cc usually emits this already,
    // but it's harmless and can help in some setups)
    println!("cargo:rustc-link-lib=static=janet");

    // cc already arranges search paths via cargo metadata, but if you ever
    // need explicit search path, you'd add:
    // println!("cargo:rustc-link-search=native={}", out_dir.display());
}
