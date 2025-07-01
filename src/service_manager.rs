use std::collections::HashMap;

use tokio::{sync::mpsc::Receiver, task::JoinSet};
use tokio_util::sync::CancellationToken;

use crate::{config::Config, config_watcher::ConfigReloadEvent, scheduler::start_service_monitor};

#[derive(Debug)]
pub struct ServiceManager {
    current_config: Config,
    active_service_tokens: HashMap<String, CancellationToken>,
    task_handles: JoinSet<()>,
    receiver_config: Receiver<ConfigReloadEvent>
}

impl ServiceManager {
    pub async fn new(mut receiver_config: Receiver<ConfigReloadEvent>) -> Self {
        let mut active_service_tokens = HashMap::new();
        let mut task_handles = JoinSet::new();

        let config = match receiver_config.recv().await.unwrap() {
            ConfigReloadEvent::Reload(config) => { config },
            ConfigReloadEvent::Error(e) => { panic!("{e}") }
        };

        for service in config.services.clone() {
            let cancel_token = CancellationToken::new();
            active_service_tokens.insert(service.name.clone(), cancel_token.clone());
            task_handles.spawn(start_service_monitor(service, cancel_token));
        }

        Self {
            current_config: config,
            active_service_tokens,
            task_handles,
            receiver_config
        }
    }

}
