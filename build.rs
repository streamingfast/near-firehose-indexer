fn main() {
    println!("cargo:rerun-if-changed=../proto");
    println!("cargo:rerun-if-changed=../proto-near");
    tonic_build::configure()
        .out_dir("./src/codec")
        .format(true)
        .compile(
            &["sf/near/codec/v1/codec.proto"],
            &["../proto", "../proto-near"],
        )
        .unwrap_or_else(|e| panic!("Failed to compile near dm proto(s) {:?}", e));
}
