use std::env;

extern crate bindgen;
extern crate cc;

fn main() {
    let link_search_path = "cargo:rustc-link-search=native";
    let link_lib = "cargo:rustc-link-lib=static";

    println!(
        "{link_search_path}={}/powerpc-eabi/lib",
        env::var("DEVKITPPC").expect("Please provided DEVKITPPC via env variables")
    );
    println!(
        "{link_search_path}={}/lib/gcc/powerpc-eabi/13.1.0",
        env::var("DEVKITPPC").expect("Please provided DEVKITPPC via env variables")
    );
    println!(
        "{link_search_path}={}/wut/lib/",
        env::var("DEVKITPRO").expect("Please provided DEVKITPRO via env variables")
    );

    println!("{link_lib}=wut");
    // println!("{link_lib}=m");
    println!("{link_lib}=c");
    println!("{link_lib}=g");
    println!("{link_lib}=gcc");
    println!("{link_lib}=sysbase");

    println!("cargo:rerun-if-changed=wrapper.h");
    let bindings = bindgen::Builder::default()
        .use_core()
        .header("wrapper.h")
        .clang_args(vec![
            "--target=powerpc-none-eabi",
            "--sysroot=/opt/devkitpro/devkitPPC/powerpc-eabi",
            "-isystem/opt/devkitpro/devkitPPC/powerpc-eabi/include",
            // "-isystem/usr/lib/clang/18.1.3/include",
        ])
        .clang_args(vec![
            "-I/opt/devkitpro/wut/include",
            "-I/opt/devkitpro/devkitPPC/powerpc-eabi/include",
            "-mfloat-abi=hard",
            // "-nostdinc",
            // "-Wno-macro-redefined",
            // "-Wno-incompatible-library-redeclaration",
        ])
        .generate()
        .expect("Error");

    let out = std::path::PathBuf::from("./src/");
    bindings
        .write_to_file(out.join("bindings.rs"))
        .expect("Error");
}
