package(default_visibility = ["//build/bazel/remote/execution/v2:__pkg__"])

load("@io_bazel_rules_go//go:def.bzl", "go_library")
load("@io_bazel_rules_go//proto:def.bzl", "go_proto_library")

go_proto_library(
    name = "remote_execution_go_proto",
    compilers = ["@io_bazel_rules_go//proto:go_grpc"],
    importpath = "github.com/bazelbuild/remote-apis/build/bazel/remote/execution/v2",
    proto = "//build/bazel/remote/execution/v2:remote_execution_proto",
    deps = [
        "//build/bazel/semver:semver_go_proto",
        "@go_googleapis//google/api:annotations_go_proto",
        "@go_googleapis//google/longrunning:longrunning_go_proto",
        "@go_googleapis//google/rpc:status_go_proto",
    ],
)

go_library(
    name = "go_default_library",
    embed = [":remote_execution_go_proto"],
    importpath = "github.com/bazelbuild/remote-apis/build/bazel/remote/execution/v2",
)
