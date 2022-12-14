/*
* Copyright (c) 2022 Rodrigo Peixoto <rodrigopex@gmail.com>
* SPDX-License-Identifier: Apache-2.0
*/
extern crate bindgen;

use std::env;
use std::path::PathBuf;

fn main() {
    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed=../src/messages.h");

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("../src/messages.h")
        // .clang_arg("--gcc-install-dir=~/.local/zephyr-sdk-0.15.2/arm-zephyr-eabi/lib/gcc/arm-zephyr-eabi/12.1.0")
        // .clang_arg("--target=thumbv7-eabif")
        // .clang_arg("-mfloat-abi=hard")
        .clang_arg("--target=riscv32")
        .clang_arg("-march=rv32imc")
        // .clang_arg("-march=rv32imac")
        .derive_default(true)
        .size_t_is_usize(true)
        .rustfmt_bindings(true)
        .wrap_unsafe_ops(true)
        .c_naming(true)
        .layout_tests(false)
        .use_core()
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
