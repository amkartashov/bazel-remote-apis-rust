#[cfg(not(doctest))]
pub mod build {
    pub mod bazel {
        pub mod remote {
            pub mod execution {
                pub mod v2 {
                    #[cfg(not(feature = "json"))]
                    include!("generated/build.bazel.remote.execution.v2.rs");
                    #[cfg(feature = "json")]
                    include!("generated_json/build.bazel.remote.execution.v2.rs");
                    #[cfg(feature = "json")]
                    include!("generated_json/build.bazel.remote.execution.v2.serde.rs");
                }
            }
            pub mod asset {
                pub mod v1 {
                    #[cfg(not(feature = "json"))]
                    include!("generated/build.bazel.remote.asset.v1.rs");
                    #[cfg(feature = "json")]
                    include!("generated_json/build.bazel.remote.asset.v1.rs");
                    #[cfg(feature = "json")]
                    include!("generated_json/build.bazel.remote.asset.v1.serde.rs");
                }
            }
            pub mod logstream {
                pub mod v1 {
                    #[cfg(not(feature = "json"))]
                    include!("generated/build.bazel.remote.logstream.v1.rs");
                    #[cfg(feature = "json")]
                    include!("generated_json/build.bazel.remote.logstream.v1.rs");
                    #[cfg(feature = "json")]
                    include!("generated_json/build.bazel.remote.logstream.v1.serde.rs");
                }
            }
        }
        pub mod semver {
            #[cfg(not(feature = "json"))]
            include!("generated/build.bazel.semver.rs");
            #[cfg(feature = "json")]
            include!("generated_json/build.bazel.semver.rs");
            #[cfg(feature = "json")]
            include!("generated_json/build.bazel.semver.serde.rs");
        }
    }
}

#[cfg(not(doctest))]
pub mod google {
    pub mod api {
        #[cfg(not(feature = "json"))]
        include!("generated/google.api.rs");
        #[cfg(feature = "json")]
        include!("generated_json/google.api.rs");
        #[cfg(feature = "json")]
        include!("generated_json/google.api.serde.rs");
    }
    pub mod bytestream {
        #[cfg(not(feature = "json"))]
        include!("generated/google.bytestream.rs");
        #[cfg(feature = "json")]
        include!("generated_json/google.bytestream.rs");
        #[cfg(feature = "json")]
        include!("generated_json/google.bytestream.serde.rs");
    }
    pub mod longrunning {
        #[cfg(not(feature = "json"))]
        include!("generated/google.longrunning.rs");
        #[cfg(feature = "json")]
        include!("generated_json/google.longrunning.rs");
        #[cfg(feature = "json")]
        include!("generated_json/google.longrunning.serde.rs");
    }
    pub mod rpc {
        #[cfg(not(feature = "json"))]
        include!("generated/google.rpc.rs");
        #[cfg(feature = "json")]
        include!("generated_json/google.rpc.rs");
        #[cfg(feature = "json")]
        include!("generated_json/google.rpc.serde.rs");
    }
}

#[cfg(test)]
#[cfg(feature = "json")]
mod json_tests {
    use build::bazel::remote::execution::v2::NodeProperties;

    use super::*;

    #[test]
    fn simple() {
        let ar = build::bazel::remote::execution::v2::ActionResult {
            output_files: vec![build::bazel::remote::execution::v2::OutputFile {
                path: "packages/types/cjs/tasks-config.d.ts.map".to_string(),
                node_properties: Some(NodeProperties {
                    mtime: Some(::pbjson_types::Timestamp {
                        ..Default::default()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }],
            ..Default::default()
        };
        println!("simple test");
        println!("{}", serde_json::to_string_pretty(&ar).unwrap());

        let ar_json = r#"
            {
              "output_files": [
                {
                  "path": "packages/types/cjs/tasks-config.d.ts.map",
                  "digest": {
                    "hash": "0ba57c041317f5b5cfd3dcaca8dd6ac7f27b67f94ad1ff18ce34912fc2a534c5",
                    "size_bytes": 7272
                  },
                  "is_executable": false
                }
              ]
            }
  "#;

        let ar2 =
            serde_json::from_str::<build::bazel::remote::execution::v2::ActionResult>(ar_json)
                .unwrap();
        println!("ar2: {}", serde_json::to_string_pretty(&ar2).unwrap());
    }
}
