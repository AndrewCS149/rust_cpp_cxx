load("//tools/buck:rust_cxx_bridge.bzl", "rust_cxx_bridge")

rust_binary(
    name = "rust_cpp_cxx",
    srcs = glob(["src/**/*.rs"]),
    edition = "2021",
    deps = [
        ":math-sys",
        ":bridge",
        "//:cxx",
    ],
)

rust_cxx_bridge(
    name = "bridge",
    src = "src/main.rs",
    deps = [":math-include"],
)

cxx_library(
    name = "math-sys",
    srcs = ["src/math_utils.cpp"],
    preferred_linkage = "static",
    deps = [
        ":math-include",
        ":bridge/include",
    ],
)

cxx_library(
    name = "math-include",
    exported_deps = ["//:core"],
    exported_headers = ["include/math_utils.h"],
)
