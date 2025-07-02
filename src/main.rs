mod checker;
mod config;
mod config_watcher;
mod service_manager;
mod scheduler;
mod test;

use std::path::PathBuf;

use anyhow::Ok;
use clap::Parser;
use tokio::sync::mpsc;

use config_watcher:: config_watcher;
use service_manager::ServiceManager;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "config.yaml")]
    config: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let (sender_config, receiver_config) = mpsc::channel(64);

    let watcher_handler = tokio::spawn(config_watcher(PathBuf::from(&args.config), sender_config));

    // Получение первого конфига
    let mut service_manager = ServiceManager::new(receiver_config).await;

    service_manager.start_manager().await;

    Ok(())
}
