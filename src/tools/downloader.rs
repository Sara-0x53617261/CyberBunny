use tracing::{debug, warn};
use tokio::io::AsyncWriteExt;
use tokio::fs::File;
use reqwest::StatusCode;

pub async fn download(url: &str, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let tmp_dir = "tmp/";
    let mut out = File::create(format!("{}/{}", tmp_dir, filename)).await?;
    
    let file = reqwest::get(url).await?;

    match file.status() {
        StatusCode::OK => {
            debug!("Downloading {} from {}", filename, url);
            out.write_all(&file.bytes().await.unwrap()).await?;
        }
        _ => {
            warn!("Error trying to download {}\nStatus code: {}", url, file.status());
        }
    }
    Ok(())
}