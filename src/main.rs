mod checker;
mod config;
mod config_watcher;
mod scheduler;

use std::path::PathBuf;

use anyhow::Ok;
use clap::Parser;
use config::Config;
use tokio::{sync::mpsc, task::JoinSet};

use config_watcher::{ConfigReloadEvent, config_watcher};
use scheduler::start_service_monitor;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "config.yaml")]
    config: String,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    let config = Config::from_file(&PathBuf::from(&args.config)).unwrap(); // Удалить потом
    println!("Config:\n{:#?}", config);

    let (sender_config, mut receiver_config) = mpsc::channel(64);

    let watcher_handler = tokio::spawn(config_watcher(PathBuf::from(&args.config), sender_config));
    let mut tasks = JoinSet::new();

    while let Some(res) = receiver_config.recv().await {
        match res {
            ConfigReloadEvent::Reload(config) => {
                println!("Config reloaded. Restart monitors...");

                tasks.abort_all();
                for service in config.services {
                    tasks.spawn(async move {
                        start_service_monitor(service).await;
                    });
                }

                println!("All monitors restarted")
            }
            ConfigReloadEvent::Error(e) => {
                eprintln!("Error from watcher: {}", e);
            }
        }
    }

    Ok(())
}
