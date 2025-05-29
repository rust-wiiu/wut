extern crate bindgen;
extern crate semver;

use semver::Version;
use std::{env, fs};

const MIN_VERSION: Version = Version::new(14, 2, 0);

fn main() {
    println!("cargo:rerun-if-changed=src/wrapper.h");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=build.rs");

    let link_search_path = "cargo:rustc-link-search=native";
    let link_lib = "cargo:rustc-link-lib=static";

    let dkp = env::var("DEVKITPRO").expect("Please provided DEVKITPRO via env variables");
    let ppc = env::var("DEVKITPPC").expect("Please provided DEVKITPPC via env variables");

    let gcc_dir = format!("{ppc}/lib/gcc/powerpc-eabi");
    let version = fs::read_dir(&gcc_dir)
        .unwrap_or_else(|_| panic!("Failed to read directory: {gcc_dir}"))
        .filter_map(|entry| {
            entry
                .ok()?
                .file_name()
                .to_str()
                .and_then(|name| Version::parse(name).ok())
                .filter(|version| version >= &MIN_VERSION)
        })
        .max()
        .expect(&format!(
            "No valid versions >= {MIN_VERSION} found in {gcc_dir} directory"
        ));

    println!("{link_search_path}={ppc}/powerpc-eabi/lib",);
    println!("{link_search_path}={ppc}/lib/gcc/powerpc-eabi/{version}");
    println!("{link_search_path}={dkp}/wut/lib/");

    println!("{link_lib}=wut");
    println!("{link_lib}=m");
    println!("{link_lib}=c");
    println!("{link_lib}=g");
    println!("{link_lib}=gcc");
    println!("{link_lib}=sysbase");
    println!("{link_lib}=stdc++");

    /*
     * These bindings will create many errors since the target cpu is a 32bit system and the host (the compilation PC) is likely a 64bit system.
     * There are alignment and size checks which will fail, because pointers have different sizes.
     */
    let bindings = bindgen::Builder::default()
        .use_core()
        .header("./src/wrapper.h")
        .emit_builtins()
        .generate_cstr(true)
        .generate_comments(false)
        .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .prepend_enum_name(false)
        .layout_tests(false)
        .derive_default(true)
        .merge_extern_blocks(true)
        .clang_args(vec![
            "--target=powerpc-none-eabi",
            &format!("--sysroot={ppc}/powerpc-eabi"),
            &format!("-isystem{ppc}/powerpc-eabi/include"),
            "-xc++",
            "-std=c++17",
            "-m32",
            "-mfloat-abi=hard",
            &format!("-I{dkp}/wut/include"),
            &format!("-I{ppc}/powerpc-eabi/include"),
            &format!("-I{ppc}/powerpc-eabi/include/c++/{version}"),
            &format!("-I{ppc}/powerpc-eabi/include/c++/{version}/powerpc-eabi"),
            "-Wno-return-type-c-linkage", // ig we can ignore these
        ])
        .allowlist_file(".*/wut/include/.*.h")
        // we need some extra functions
        .header_contents(
            "single_symbols.h",
            r#"
                #pragma once

                #include <unistd.h>
                #include <errno.h>
            "#,
        )
        .allowlist_function("close")
        .allowlist_function("__errno")
        .allowlist_var("^E[A-Z0-9_]+$")
        //
        .opaque_type("MEMFrmHeap")
        .opaque_type("MEMExpHeap")
        .opaque_type("MEMUnitHeap")
        .opaque_type("MEMBlockHeap")
        .raw_line("#![allow(non_upper_case_globals)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .raw_line("#![doc(hidden)]")
        .generate()
        .expect("Unable to generate bindings");

    let out = std::path::PathBuf::from("./src/bindings.rs");
    bindings
        .write_to_file(&out)
        .expect("Unable to write bindings to file");
}
