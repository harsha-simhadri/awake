extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=src/rwutils.c");
    // println!("cargo:rustc-link-search=src/");
    cc::Build::new()
        .file("src/c_file_rw_utils.c")
        .compile("c_file_rw_utils");
}
