fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("cargo:rerun-if-changed=build.rs");
    #[cfg(feature = "codegen")]
    {
        let out_dir = std::path::Path::new("src/generated");
        let out_dir_serde = std::path::Path::new("src/generated_serde");
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

        for proto_file in protos {
            println!("cargo:rerun-if-changed={}", proto_file);
        }

        tonic_build::configure()
            .out_dir(out_dir)
            .compile_protos(protos, includes)?;

        let descriptor_path = out_dir_serde.join("proto_descriptor.bin");
        tonic_build::configure()
            .out_dir(out_dir_serde)
            .file_descriptor_set_path(&descriptor_path)
            .compile_well_known_types(true)
            .extern_path(".google.protobuf", "::pbjson_types")
            .compile_protos(protos, includes)?;

        let descriptor_set = std::fs::read(descriptor_path)?;
        pbjson_build::Builder::new()
            .register_descriptors(&descriptor_set)?
            .out_dir(out_dir_serde)
            .build(&["."])?;
    }
    Ok(())
}
