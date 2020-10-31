use protoc_rust_grpc;

fn main() {
    protoc_rust_grpc::run(protoc_rust_grpc::Args {
        out_dir: "src",
        includes: &[],
        input: &["foobar.proto"]
        rust_protobuf: true,
    }).expect("Failed to generate Rust src");
}
