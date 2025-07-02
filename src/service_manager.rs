use std::collections::HashMap;

use tokio::{sync::mpsc::Receiver, task::JoinSet};
use tokio_util::sync::CancellationToken;

use crate::{
    config::{Config, ServiceConfig},
    config_watcher::ConfigReloadEvent,
    scheduler::start_service_monitor,
};

#[derive(Debug)]
pub struct ServiceManager {
    current_config: Config,
    active_service_tokens: HashMap<String, CancellationToken>,
    task_handles: JoinSet<()>,
    receiver_config: Receiver<ConfigReloadEvent>,
}

impl ServiceManager {
    pub async fn new(mut receiver_config: Receiver<ConfigReloadEvent>) -> Self {
        let mut active_service_tokens = HashMap::new();
        let mut task_handles = JoinSet::new();

        let config = match receiver_config.recv().await.unwrap() {
            ConfigReloadEvent::Reload(config) => config,
            ConfigReloadEvent::Error(e) => {
                panic!("{e}")
            }
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
            receiver_config,
        }
    }

    pub async fn start_manager(&mut self) {
        while let Some(res) = self.receiver_config.recv().await {
            match res {
                ConfigReloadEvent::Reload(config) => match self.config_comparison(&config) {
                    Some(config_diff) => {
                        println!("Config changed: {:?}", config_diff);
                        self.stop_services(config_diff.services_to_cancel);
                        self.start_services(config_diff.services_to_start);
                        self.current_config = config;
                    }
                    None => {
                        println!("Config did not change");
                    }
                },
                ConfigReloadEvent::Error(e) => {
                    eprintln!("{e}")
                }
            }
        }
    }

    /// Возвращает None, если конфиг не изменился, или Some(ConfigDiff), если изменился; первый элемент - что нужно остановить, второй - что запустить
    fn config_comparison(&self, other_config: &Config) -> Option<ConfigDiff> {
        if self.current_config != *other_config {
            let mut config_diff = ConfigDiff::new();
            for service in &other_config.services {
                if !self.current_config.services.contains(service) {
                    config_diff.services_to_start.push(service.clone());
                }
            }
            for service in &self.current_config.services {
                if !other_config.services.contains(service) {
                    config_diff.services_to_cancel.push(service.name.clone());
                }
            }
            return Some(config_diff);
        }
        None
    }

    fn stop_services(&mut self, services_to_cancel: Vec<String>) {
        for service_name in services_to_cancel {
            if let Some(token) = self.active_service_tokens.get(&service_name) {
                println!("Cancelling {}", service_name);
                token.cancel();
                self.active_service_tokens.remove(&service_name);
            } else {
                println!("Service {} not found", service_name);
            }
        }
        while let Some(_) = self.task_handles.try_join_next() {}
    }

    fn start_services(&mut self, services_to_start: Vec<ServiceConfig>) {
        for service in services_to_start {
            println!("Starting {}", service.name);
            let cancel_token = CancellationToken::new();
            self.active_service_tokens.insert(service.name.clone(), cancel_token.clone());
            self.task_handles.spawn(start_service_monitor(service, cancel_token));
        }
    }
}

#[derive(Debug)]
struct ConfigDiff {
    services_to_cancel: Vec<String>,
    services_to_start: Vec<ServiceConfig>,
}

impl ConfigDiff {
    fn new() -> Self {
        Self {
            services_to_cancel: vec![],
            services_to_start: vec![],
        }
    }
}
