#[cfg(not(doctest))]
#[path = "."]
pub mod build {
    #[path = "."]
    pub mod bazel {
        #[path = "."]
        pub mod remote {
            #[path = "."]
            pub mod execution {
                #[path = "generated/build.bazel.remote.execution.v2.rs"]
                pub mod v2;
            }
            #[path = "."]
            pub mod asset {
                #[path = "generated/build.bazel.remote.asset.v1.rs"]
                pub mod v1;
            }
            #[path = "."]
            pub mod logstream {
                #[path = "generated/build.bazel.remote.logstream.v1.rs"]
                pub mod v1;
            }
        }
        #[path = "generated/build.bazel.semver.rs"]
        pub mod semver;
    }
}

#[cfg(not(doctest))]
#[path = "."]
pub mod google {
    #[path = "generated/google.api.rs"]
    pub mod api;
    #[path = "generated/google.bytestream.rs"]
    pub mod bytestream;
    #[path = "generated/google.longrunning.rs"]
    pub mod longrunning;
    #[path = "generated/google.protobuf.rs"]
    pub mod protobuf;
    #[path = "generated/google.rpc.rs"]
    pub mod rpc;
}
