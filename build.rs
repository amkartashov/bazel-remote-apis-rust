fn main() -> Result<(), Box<dyn std::error::Error>> {
    #[cfg(feature = "codegen")]
    println!("cargo:rerun-if-changed=vendor");
    #[cfg(feature = "codegen")]
    {
        let protos = &[
            "vendor/github.com/bazelbuild/remote-apis/build/bazel/remote/execution/v2/remote_execution.proto",
            "vendor/github.com/bazelbuild/remote-apis/build/bazel/remote/asset/v1/remote_asset.proto",
            "vendor/github.com/bazelbuild/remote-apis/build/bazel/remote/logstream/v1/remote_logstream.proto",
            "vendor/github.com/googleapis/googleapis/google/bytestream/bytestream.proto",
        ];

        let includes = &[
            "vendor/github.com/bazelbuild/remote-apis/",
            "vendor/github.com/googleapis/googleapis",
        ];

        let builder = tonic_build::configure().out_dir("src/generated");
        let type_attr =
            "#[cfg_attr(feature = \"serde\", derive(serde::Deserialize, serde::Serialize))]";

        let builder = builder
            .compile_well_known_types(true)
            .type_attribute(".", type_attr);

        builder.compile_protos(protos, includes)?;
    }
    Ok(())
}
