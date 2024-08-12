fn main() {
    println!("cargo:rerun-if-changed=../include/rust_time.h");
    cc::Build::new()
        .cpp(true)
        .std("c++17")
        .include("../include")
        .file("../src/rust_time.cxx")
        .compile("rust_time");
}
