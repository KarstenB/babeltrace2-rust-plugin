// Copyright 2023 - 2023, Karsten Becker and the rust babeltrace2-plugin contributors
// SPDX-License-Identifier: GPL-2.0-or-later
use std::env;
use std::path::PathBuf;

fn main() {
    cc::Build::new()
        .debug(true)
        .file("src/wrapper.c")
        .compile("inline_wrapper");

    println!("cargo:rustc-link-lib=babeltrace2");
    println!("cargo:rerun-if-changed=src/wrapper.h");

    let bindings = bindgen::Builder::default()
        .header("src/wrapper.h")
        .generate_comments(true)
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        .generate()
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");

}

