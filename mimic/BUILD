load("@crate_index_cargo_local//:defs.bzl", "aliases", "all_crate_deps")
load("@rules_rust//rust:defs.bzl", "rust_library", "rust_binary", "rust_test")

rust_binary(
    name = "run_mimic",
    srcs = glob(["**/*.rs"]),
    data = [
        "input.txt",
    ],
    edition = "2021",
    proc_macro_deps = all_crate_deps(proc_macro = True),
    deps = all_crate_deps(normal = True) + ["//lib:lib"],
)

genrule(
    name = "run_mimic_gen",
    srcs = ["input.txt"],
    outs = ["output.txt"],
    cmd = "$(location :run_mimic) --input_file $(location input.txt) --output_file $(location output.txt)",
    tools = [":run_mimic"],
)
