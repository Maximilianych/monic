#[cfg(test)]
mod test_into_hashmap {
    use std::collections::HashMap;

    use crate::config::{Config, ServiceConfig};

    #[test]
    fn into_hashmap() {
        let service_1 = ServiceConfig {
            name: "Test".to_string(),
            service_type: "http".to_string(),
            target: "https://test.ts".to_string(),
            interval: std::time::Duration::from_secs(10),
            timeout: std::time::Duration::from_secs(3),
        };
        let service_2 = ServiceConfig {
            name: "Test2".to_string(),
            service_type: "http".to_string(),
            target: "https://test2.ts".to_string(),
            interval: std::time::Duration::from_secs(10),
            timeout: std::time::Duration::from_secs(3),
        };
        let config = Config {
            services: vec![service_1.clone(), service_2.clone()],
        };
        let config_hashmap_1: HashMap<String, ServiceConfig> = (&config).into();
        let mut config_hashmap_2 = HashMap::new();
        config_hashmap_2.insert(
            "Test".to_string(),
            service_1,
        );
        config_hashmap_2.insert("Test2".to_string(), service_2);

        assert_eq!(config_hashmap_1, config_hashmap_2);
    }
}
