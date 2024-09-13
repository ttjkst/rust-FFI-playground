use std::env;
use std::path::PathBuf;
use bindgen::CargoCallbacks;

fn main() {
    cc::Build::new().flag("-v")
        .file("third-party/sqlite3/sqlite3.c")
        //.file("third-party/sqlite3/sqlite3.h")
        .compile("sqlite3");

    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header("third-party/sqlite3/sqlite3.h")
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(CargoCallbacks{}))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR")
        .unwrap()).join("bindings.rs");
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");

}