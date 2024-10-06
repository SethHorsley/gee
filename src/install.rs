use anyhow::{Context, Result};
use std::env;
use std::fs;
use std::os::unix::fs::PermissionsExt;
use std::process::Command;

use crate::github;
use crate::shell;

fn is_apple_silicon() -> bool {
    let output = Command::new("uname")
        .arg("-m")
        .output()
        .expect("Failed to execute uname command");
    
    String::from_utf8_lossy(&output.stdout).trim() == "arm64"
}

pub async fn install_binary(repo: &str) -> Result<()> {
    let parts: Vec<&str> = repo.split('/').collect();
    if parts.len() != 3 {
        anyhow::bail!("Invalid repository format. Use 'username/repo/binary'");
    }

    let (username, repo_name, binary_name) = (parts[0], parts[1], parts[2]);
    
    let release = github::fetch_latest_release(username, repo_name).await?;

    let os = env::consts::OS;
    let (arch, arm_variants) = if os == "macos" {
        if is_apple_silicon() {
            ("arm64", vec!["arm64", "aarch64"])
        } else {
            ("x86_64", vec!["x86_64"])
        }
    } else {
        (env::consts::ARCH, vec![env::consts::ARCH])
    };
    
    let possible_asset_names: Vec<String> = if os == "macos" {
        arm_variants.into_iter().flat_map(|arm_arch| {
            vec![
                format!("{}-darwin-{}", binary_name, arm_arch),
                format!("{}-macos-{}", binary_name, arm_arch),
            ]
        }).chain(vec![
            format!("{}-darwin-{}", binary_name, arch),
            format!("{}-macos-{}", binary_name, arch),
        ]).collect()
    } else {
        vec![format!("{}-{}-{}", binary_name, os, arch)]
    };

    println!("Looking for assets: {:?}", possible_asset_names);

    let asset = release.assets
        .iter()
        .find(|a| possible_asset_names.contains(&a.name))
        .context(format!("No suitable binary found for your system (OS: {}, Arch: {})", os, arch))?;

    println!("Found matching asset: {}", asset.name);
    println!("Downloading version {} from: {}", release.tag_name, asset.browser_download_url);

    let client = reqwest::Client::new();
    let response = client.get(&asset.browser_download_url)
        .send()
        .await?
        .bytes()
        .await?;

    let gee_dir = dirs::home_dir()
        .context("Failed to get home directory")?
        .join(".local/share/gee/bin");
    fs::create_dir_all(&gee_dir)?;

    let binary_path = gee_dir.join(binary_name);
    fs::write(&binary_path, response)?;

    let mut perms = fs::metadata(&binary_path)?.permissions();
    perms.set_mode(0o755);
    fs::set_permissions(&binary_path, perms)?;

    println!("Binary installed to: {}", binary_path.display());

    shell::update_shell_config(&gee_dir)?;

    Ok(())
}
