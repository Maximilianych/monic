use std::collections::HashMap;

use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;

use crate::{config::Config, scheduler::start_service_monitor};

#[derive(Debug)]
pub struct ServiceManager {
    current_config: Config,
    active_service_tokens: HashMap<String, CancellationToken>,
    task_handles: JoinSet<()>,
}

impl ServiceManager {
    pub fn new(config: Config) -> Self {
        let mut active_service_tokens = HashMap::with_capacity(config.services.len());
        let mut task_handles = JoinSet::new();

        for service in config.services.clone() {
            let cancel_token = CancellationToken::new();
            active_service_tokens.insert(service.name.clone(), cancel_token.clone());
            task_handles.spawn(start_service_monitor(service, cancel_token));
        }

        Self {
            current_config: config,
            active_service_tokens,
            task_handles,
        }
    }
}
