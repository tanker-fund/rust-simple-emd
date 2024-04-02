extern crate cc;

use std::env;

const SRC_FILE: &str = "extern/emd.c";


fn main() {
    // Compile
    cc::Build::new()
              .file(SRC_FILE)
              .compile("libemd.a");

    // Link library
    let project_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!("cargo:rustc-link-search={}/src/", project_dir);
    println!("proj dir: {}", project_dir);
    println!("cargo:rustc-link-lib=emd");
}
