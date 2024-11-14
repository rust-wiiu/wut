use std::env;

extern crate bindgen;

fn main() {
    let link_search_path = "cargo:rustc-link-search=native";
    let link_lib = "cargo:rustc-link-lib=static";

    let dkp = env::var("DEVKITPRO").expect("Please provided DEVKITPRO via env variables");
    let ppc = env::var("DEVKITPPC").expect("Please provided DEVKITPPC via env variables");

    println!("{link_search_path}={ppc}/powerpc-eabi/lib",);
    println!("{link_search_path}={ppc}/lib/gcc/powerpc-eabi/13.1.0");
    println!("{link_search_path}={dkp}/wut/lib/");

    println!("{link_lib}=wut");
    // println!("{link_lib}=m");
    println!("{link_lib}=c");
    println!("{link_lib}=g");
    println!("{link_lib}=gcc");
    println!("{link_lib}=sysbase");

    println!("cargo:rerun-if-changed=wrapper.h");
    println!("cargo:rerun-if-changed=build.rs");

    /*
     * These bindings will create many errors since the target cpu
     * is a 32bit system and the host (the compilation PC) is likely
     * a 64bit system. There are alignment and size checks which will
     * fail, because pointers have different sizes.
     */
    let bindings = bindgen::Builder::default()
        .use_core()
        .header("src/wrapper.h")
        .emit_builtins()
        .generate_cstr(true)
        .generate_comments(false)
        // .default_enum_style(bindgen::EnumVariation::ModuleConsts)
        .prepend_enum_name(false)
        .layout_tests(false)
        .derive_default(true)
        .clang_args(vec![
            "--target=powerpc-none-eabi",
            "-m32",
            "-mfloat-abi=hard",
            format!("-I{dkp}/wut/include").as_str(),
            format!("-I{ppc}/powerpc-eabi/include").as_str(),
            format!("-I{ppc}/lib/gcc/powerpc-eabi/13.1.0/include/").as_str(),
        ])
        .raw_line("#![allow(non_upper_case_globals)]")
        .raw_line("#![allow(non_camel_case_types)]")
        .raw_line("#![allow(non_snake_case)]")
        .generate()
        .expect("Unable to generate bindings");

    let out = std::path::PathBuf::from("./src/bindings.rs");
    bindings
        .write_to_file(out)
        .expect("Unable to write bindings to file");
}
