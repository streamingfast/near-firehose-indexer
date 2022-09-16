fn main() {
    tonic_build::configure()
        .out_dir("./src/codec")
        .format(true)
        .compile(
            &["sf/near/codec/v1/codec.proto"],
            &["../proto", "../proto-near"],
        )
        .unwrap_or_else(|e| panic!("Failed to compile near NEAR firehose proto(s) {:?}", e));
}
