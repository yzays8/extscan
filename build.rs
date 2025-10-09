use std::{env, path::Path};

fn main() {
    // Currently, Magika has not enabled the `download-binaries` feature for the `ort` crate,
    // so you will need to prepare the ONNX Runtime binaries yourself and link them.
    // https://github.com/google/magika/blob/main/rust/lib/Cargo.toml
    if let Ok(lib_path) = env::var("ORT_LIB_LOCATION") {
        let lib_dir = Path::new(&lib_path);
        if !lib_dir.exists() {
            eprintln!(
                "ORT_LIB_LOCATION is set, but the path does not exist: {}",
                lib_path
            );
            std::process::exit(1);
        }
        println!("cargo:rustc-link-arg=-Wl,-rpath,{}", lib_dir.display());
    } else {
        eprintln!("ORT_LIB_LOCATION is not set. Please export it before building.");
        std::process::exit(1);
    }
}
