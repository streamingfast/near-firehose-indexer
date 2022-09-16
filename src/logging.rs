use tracing_subscriber::EnvFilter;

pub(crate) fn init() {
    let config = std::env::var("RUST_LOG").unwrap_or(
        "main=info,near=info,stats=info,firehose=info,indexer=info,network=warn,runtime=warn"
            .to_string(),
    );
    let env_filter = EnvFilter::new(config);

    tracing_subscriber::fmt::Subscriber::builder()
        .with_env_filter(env_filter)
        .with_writer(std::io::stderr)
        .init();
}
