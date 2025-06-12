use reqwest::Client;

pub async fn check_http(target: &str, timeout: std::time::Duration) -> anyhow::Result<bool> {
    let client = Client::new();
    let _res = client
        .get(target)
        .timeout(timeout)
        .body(reqwest::Body::default())
        .send()
        .await?;

    Ok(true)
}
