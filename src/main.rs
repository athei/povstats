use clap::Parser;
use futures::{StreamExt, TryStreamExt};
use std::sync::Arc;
use subxt::{ClientBuilder, DefaultConfig, SubstrateExtrinsicParams};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// The node to connect to.
    #[clap(long, default_value = "ws://localhost:9944/")]
    url: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let client = Arc::new(ClientBuilder::new().set_url(args.url).build().await?);

    let mut stats = povstats::subscribe_stats(client).await?.into_stream();

    while let Some(stat) = stats.next().await {
        println!("{}", stat?);
    }

    Ok(())
}
