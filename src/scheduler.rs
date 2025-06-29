use tokio_util::sync::CancellationToken;

use crate::{checker::check_http, config::ServiceConfig};

pub async fn start_service_monitor(service: ServiceConfig, cancel_token: CancellationToken) {
    let mut interval = tokio::time::interval(service.interval);

    loop {
        // Ожидание тика или завершения от токена
        tokio::select! {
            _ = interval.tick() => {}
            _ = cancel_token.cancelled() => break
        }

        match service.service_type.as_str() {
            "http" => {
                let result = check_http(&service.target, service.timeout).await;
                println!("[{}] Status: {:?}", service.name, result.unwrap_or(false))
            }
            _ => println!("Unsupported service type"),
        }
    }
}
