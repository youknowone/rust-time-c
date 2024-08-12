fn main() {
    cc::Build::new()
        .cpp(true)
        .std("c++17")
        .include("../include")
        .file("../src/rust_time.cxx")
        .compile("rust_time");
}
