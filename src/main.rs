mod config;
mod scheduler;
mod checker;

use config::Config;

use anyhow::Ok;
use clap::Parser;
use tokio::task::JoinSet;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "config.yaml")]
    config: String
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config = Config::from_file(&args.config).unwrap();
    println!("Config:\n{:#?}", config);

    let mut tasks = JoinSet::new();

    for service in config.services {
        tasks.spawn(async move {
            scheduler::start_service_monitor(service).await;
        });
    }

    while let Some(_) = tasks.join_next().await {}

    Ok(())
}
