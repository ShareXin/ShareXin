extern crate bindgen;
extern crate pkg_config;

use std::env;
use std::path::PathBuf;

fn write_bindings(library : pkg_config::Library) {
    let mut bindings = bindgen::Builder::default()
        .no_unstable_rust()
        .header("wrapper.h")
        // Hide Gtk types, as these will be filled in via gtk-sys
        .hide_type("Gtk.*")
        .whitelisted_type(".*AppIndicator.*")
        .whitelisted_function("app_indicator_.*");

    for p in library.include_paths {
        bindings = bindings
            .clang_arg("-I")
            .clang_arg(format!("{}", p.as_path().display()));
    }

    let gen_bindings = bindings.generate().expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    gen_bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
fn main() {
    println!("cargo:rustc-link-lib=appindicator3");

    match pkg_config::probe_library("appindicator3") {
        Ok(library) => write_bindings(library),
        Err(_) => {
            match pkg_config::probe_library("appindicator3-0.1") {
                Ok(library) => write_bindings(library),
                Err(_) => panic!("libappindicator3 library not found!")
            }
        }
    };
}
