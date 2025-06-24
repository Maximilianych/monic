use std::path::Path;

use anyhow::Ok;
use notify::{recommended_watcher, Config, Watcher};
use tokio::sync::mpsc;

pub enum ConfigReloadEvent {
    Reload(Config),
    Error(anyhow::Error),
}

pub async fn config_watcher(path: String) -> anyhow::Result<()> {
    let (tx_notify_events, mut rx_notify_events) = mpsc::channel(64);

    let mut watcher = recommended_watcher(move |res| {
        futures::executor::block_on(async {
            if let Err(e) = tx_notify_events.send(res).await {
                eprintln!("Error sending notify event: {}", e);
            }
        });
    })?;

    watcher.watch(Path::new(&path), notify::RecursiveMode::NonRecursive)?;

    while let Some(res) = rx_notify_events.recv().await {
        match res {
            Result::Ok(event) => {
                println!("ИВЕНТ")
            },
            Result::Err(e) => {
                eprintln!("ХАНА")
            }
        }
    }

    Ok(())
}
