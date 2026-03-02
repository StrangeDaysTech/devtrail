use anyhow::{bail, Context, Result};
use indicatif::{ProgressBar, ProgressStyle};
use std::io::Read;
use std::path::Path;

const GITHUB_REPO: &str = "StrangeDaysTech/devtrail";
const GITHUB_API_BASE: &str = "https://api.github.com";

/// Information about a GitHub release
#[derive(Debug)]
pub struct ReleaseInfo {
    pub tag_name: String,
    pub zip_url: String,
}

/// Get the latest release info from GitHub
pub fn get_latest_release() -> Result<ReleaseInfo> {
    let url = format!("{}/repos/{}/releases/latest", GITHUB_API_BASE, GITHUB_REPO);

    let client = reqwest::blocking::Client::builder()
        .user_agent("devtrail-cli")
        .build()
        .context("Failed to create HTTP client")?;

    let response = client
        .get(&url)
        .send()
        .context("Failed to fetch latest release from GitHub")?;

    if response.status() == reqwest::StatusCode::FORBIDDEN {
        bail!(
            "GitHub API rate limit exceeded. Set GITHUB_TOKEN environment variable to increase limits."
        );
    }

    if response.status() == reqwest::StatusCode::NOT_FOUND {
        bail!("No releases found for {}", GITHUB_REPO);
    }

    if !response.status().is_success() {
        bail!("GitHub API error: {}", response.status());
    }

    let body: serde_json::Value = response.json().context("Failed to parse release JSON")?;

    let tag_name = body["tag_name"]
        .as_str()
        .context("Missing tag_name in release")?
        .to_string();

    // Look for the devtrail distribution ZIP in release assets
    let zip_url: Option<String> = if let Some(assets) = body["assets"].as_array() {
        let mut found = None;
        for asset in assets {
            if let Some(name) = asset["name"].as_str() {
                if name.starts_with("devtrail-") && name.ends_with(".zip") {
                    if let Some(url) = asset["browser_download_url"].as_str() {
                        found = Some(url.to_string());
                        break;
                    }
                }
            }
        }
        found
    } else {
        None
    };

    // Fallback to zipball if no distribution asset found
    let zip_url = zip_url.unwrap_or_else(|| {
        format!(
            "https://github.com/{}/archive/refs/tags/{}.zip",
            GITHUB_REPO, tag_name
        )
    });

    Ok(ReleaseInfo { tag_name, zip_url })
}

/// Download a ZIP file from a URL to a temporary file
pub fn download_zip(url: &str, dest: &Path) -> Result<()> {
    let client = reqwest::blocking::Client::builder()
        .user_agent("devtrail-cli")
        .build()
        .context("Failed to create HTTP client")?;

    let mut response = client
        .get(url)
        .send()
        .context("Failed to download release ZIP")?;

    if !response.status().is_success() {
        bail!("Download failed with status: {}", response.status());
    }

    let total_size = response.content_length().unwrap_or(0);

    let pb = ProgressBar::new(total_size);
    pb.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{bar:40.cyan/blue}] {bytes}/{total_bytes} ({eta})")
            .unwrap()
            .progress_chars("#>-"),
    );

    let mut file = std::fs::File::create(dest).context("Failed to create temporary file")?;
    let mut buffer = [0u8; 8192];
    let mut downloaded: u64 = 0;

    loop {
        let bytes_read = response.read(&mut buffer).context("Failed to read response")?;
        if bytes_read == 0 {
            break;
        }
        std::io::Write::write_all(&mut file, &buffer[..bytes_read])
            .context("Failed to write to file")?;
        downloaded += bytes_read as u64;
        pb.set_position(downloaded);
    }

    pb.finish_with_message("Download complete");
    Ok(())
}
