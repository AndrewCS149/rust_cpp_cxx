fn main() {
    cxx_build::bridge("src/main.rs")
        .file("src/math_utils.cpp")
        .std("c++14")
        .compile("cxxbridge-rust_cpp_cxx");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=src/math_utils.cpp");
    println!("cargo:rerun-if-changed=include/math_utils.h");
}
