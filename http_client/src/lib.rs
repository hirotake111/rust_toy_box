use hyper::StatusCode;

pub async fn fetch(url: &str) -> Result<StatusCode, Box<dyn std::error::Error>> {
    let status_code = reqwest::get(url).await?.status();
    Ok(status_code)
}
