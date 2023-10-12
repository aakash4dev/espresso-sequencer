use async_compatibility_layer::logging::{setup_backtrace, setup_logging};
use clap::Parser;
use hotshot_web_server::run_web_server;
use sequencer::SeqTypes;

#[derive(Parser)]
struct Args {
    /// Port to run the server on.
    #[clap(short, long, env = "ESPRESSO_WEB_SERVER_PORT")]
    port: u16,
}

#[async_std::main]
async fn main() {
    setup_logging();
    setup_backtrace();

    let args = Args::parse();

    tracing::info!("starting web server on port {}", args.port);
    run_web_server::<
        <SeqTypes as hotshot_types::traits::node_implementation::NodeType>::SignatureKey,
    >(None, args.port)
    .await
    .unwrap();
}
