fn main() {
    cc::Build::new()
        .cpp(true)
        .std("c++17")
        .include("../include")
        .file("test.cxx")
        .compile("test");
}
