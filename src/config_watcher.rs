use std::path::PathBuf;

use notify::{Watcher, recommended_watcher};
use tokio::sync::mpsc;

use crate::config::Config;

pub enum ConfigReloadEvent {
    Reload(Config),
    Error(anyhow::Error),
}

pub async fn config_watcher(
    path: PathBuf,
    sender_config: mpsc::Sender<ConfigReloadEvent>,
) -> anyhow::Result<()> {
    let (tx_notify_events, mut rx_notify_events) = mpsc::channel(64);

    // Создание наблюдателя, следящего за событиями в файле
    let mut watcher = recommended_watcher(move |res| {
        futures::executor::block_on(async {
            if let Err(e) = tx_notify_events.send(res).await {
                eprintln!("Error sending notify event: {}", e);
            }
        });
    })?;
    watcher.watch(path.as_path(), notify::RecursiveMode::NonRecursive)?;

    // Отправка начального конфига
    match Config::from_file(&path) {
        Ok(config) => {
            println!("Initial configuration loading...");
            sender_config
                .send(ConfigReloadEvent::Reload(config))
                .await?
        }
        Err(e) => {
            eprintln!("Initial config load failed: {}", e);
            sender_config.send(ConfigReloadEvent::Error(e)).await?
        }
    }

    // Отравка нового конфига при изменении
    while let Some(res) = rx_notify_events.recv().await {
        match res {
            Ok(event) => {
                if event.kind.is_modify() {
                    println!("Config file changed. Attempting to reload...");
                    match Config::from_file(&path) {
                        Ok(config) => {
                            if let Err(e) =
                                sender_config.send(ConfigReloadEvent::Reload(config)).await
                            {
                                eprintln!("Error sending reloaded config: {}", e);
                            }
                        }
                        Err(e) => {
                            if let Err(e) = sender_config.send(ConfigReloadEvent::Error(e)).await {
                                eprintln!("Error when sending config reload error: {}", e);
                            }
                        }
                    }
                }
            }
            Err(e) => {
                if let Err(e) = sender_config
                    .send(ConfigReloadEvent::Error(anyhow::Error::new(e)))
                    .await
                {
                    eprintln!("Error when sending config reload error: {}", e);
                }
            }
        }
    }

    Ok(())
}
