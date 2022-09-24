fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "codegen")]
    println!("cargo:rerun-if-changed=vendor");
    #[cfg(feature = "codegen")]
    tonic_build::configure()
        .out_dir("src/generated")
        .compile(&[
            "vendor/github.com/bazelbuild/remote-apis/build/bazel/remote/execution/v2/remote_execution.proto",
            "vendor/github.com/bazelbuild/remote-apis/build/bazel/remote/asset/v1/remote_asset.proto",
            "vendor/github.com/bazelbuild/remote-apis/build/bazel/remote/logstream/v1/remote_logstream.proto",
            "vendor/github.com/googleapis/googleapis/google/bytestream/bytestream.proto",
        ],
        &[
            "vendor/github.com/bazelbuild/remote-apis/",
            "vendor/github.com/googleapis/googleapis",
        ],
    )?;
    Ok(())
}
