mod checker;
mod config;
mod config_watcher;
mod service_manager;
mod scheduler;

use std::path::PathBuf;

use anyhow::Ok;
use clap::Parser;
use tokio::{sync::mpsc, task::JoinSet};
use tokio_util::sync::CancellationToken;

use config_watcher::{ConfigReloadEvent, config_watcher};
use scheduler::start_service_monitor;
use service_manager::ServiceManager;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "config.yaml")]
    config: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();

    let (sender_config, mut receiver_config) = mpsc::channel(64);

    let watcher_handler = tokio::spawn(config_watcher(PathBuf::from(&args.config), sender_config));

    // Получение первого конфига
    let service_manager = match receiver_config.recv().await.unwrap() {
        ConfigReloadEvent::Reload(config) => {
            ServiceManager::new(config)
        }
        ConfigReloadEvent::Error(e) => {panic!("{e}")}
    };

    println!("Initial config:\n{:#?}", service_manager);

    Ok(())
}
