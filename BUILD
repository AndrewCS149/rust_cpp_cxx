load("@rules_cc//cc:defs.bzl", "cc_library")
load("@rules_rust//rust:defs.bzl", "rust_binary")
load("//tools/bazel:rust_cxx_bridge.bzl", "rust_cxx_bridge")

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

cc_library(
    name = "math-sys",
    srcs = ["src/math_utils.cpp"],
    deps = [
        ":math-include",
        ":bridge/include",
    ],
)

cc_library(
    name = "math-include",
    hdrs = ["include/math_utils.h"],
    deps = ["//:core"],
)
