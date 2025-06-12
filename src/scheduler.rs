use crate::{checker::check_http, config::ServiceConfig};

pub async fn start_service_monitor(service: ServiceConfig) {
    let mut interval = tokio::time::interval(service.interval);

    loop {
        interval.tick().await;

        match service.service_type.as_str() {
            "http" => {
                let result = check_http(&service.target, service.timeout).await;
                println!("[{}] Status: {:?}", service.name, result.unwrap_or(false))
            }
            _ => println!("Unsupported service type"),
        }
    }
}
