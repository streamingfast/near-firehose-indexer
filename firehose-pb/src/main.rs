fn main() {
    tonic_build::configure()
        .out_dir("./src/codec")
        .format(true)
        .compile(&["sf/near/type/v1/type.proto"], &["../sf-near/proto/"])
        .unwrap_or_else(|e| panic!("Failed to compile near NEAR firehose proto(s) {:?}", e));
}
