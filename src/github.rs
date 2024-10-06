use anyhow::Result;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct Release {
    pub tag_name: String,
    pub assets: Vec<Asset>,
}

#[derive(Deserialize)]
pub struct Asset {
    pub name: String,
    pub browser_download_url: String,
}

pub async fn fetch_latest_release(username: &str, repo_name: &str) -> Result<Release> {
    let releases_url = format!(
        "https://api.github.com/repos/{}/{}/releases/latest",
        username, repo_name
    );
    
    println!("Fetching release info from: {}", releases_url);

    let client = reqwest::Client::new();
    let release: Release = client
        .get(&releases_url)
        .header("User-Agent", "gee-installer")
        .send()
        .await?
        .json()
        .await?;

    Ok(release)
}

