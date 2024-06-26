extern crate cc;

fn main() {
    println!("cargo:rerun-if-changed=src/c/file_rw_utils.c");
    println!("cargo:rerun-if-changed=src/c/ffi_async.c");
    // println!("cargo:rustc-link-search=src/");
    cc::Build::new()
        .file("src/c/file_rw_utils.c")
        .compile("c_file_rw_utils");
    cc::Build::new()
        .file("src/c/ffi_async.c")
        .compile("c_ffi_async");
}
